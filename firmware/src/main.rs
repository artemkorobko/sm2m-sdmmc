#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod adapter;
mod error;
mod peripherals;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use crate::adapter;
    use crate::peripherals::*;

    use stm32f1xx_hal::{
        gpio::{self, ExtiPin},
        prelude::*,
        spi,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        adapter: adapter::Device,
        dtli: gpio::PB13<gpio::Input<gpio::PullDown>>,
    }

    macro_rules! into_output {
        ($pin:expr, $cr:expr) => {
            $pin.into_push_pull_output_with_state($cr, gpio::PinState::High)
        };
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut afio = cx.device.AFIO.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(16.MHz())
            .sysclk(72.MHz())
            .hclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .freeze(&mut flash.acr);

        // Configure GPIO
        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        // Configure LED indicators
        let pins = indicators::Pins {
            system_error: into_output!(gpioa.pa0, &mut gpioa.crl),
            write: into_output!(gpioa.pa1, &mut gpioa.crl),
            read: into_output!(gpioa.pa2, &mut gpioa.crl),
        };

        let mut indicators = indicators::Indicators::new(pins);

        // Indicate adapter startup
        indicators.system_error_on();
        indicators.write_on();
        indicators.read_on();

        // Disable JTAG
        let (pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure SM2M input bus
        let pins = sm2m::input::Pins {
            di_0: gpiob.pb7.into_pull_down_input(&mut gpiob.crl),
            di_1: gpioe.pe1.into_pull_down_input(&mut gpioe.crl),
            di_2: gpioe.pe0.into_pull_down_input(&mut gpioe.crl),
            di_3: gpioe.pe2.into_pull_down_input(&mut gpioe.crl),
            di_4: gpioe.pe3.into_pull_down_input(&mut gpioe.crl),
            di_5: gpioe.pe5.into_pull_down_input(&mut gpioe.crl),
            di_6: gpioe.pe4.into_pull_down_input(&mut gpioe.crl),
            di_7: gpioe.pe6.into_pull_down_input(&mut gpioe.crl),
            di_8: pb4.into_pull_down_input(&mut gpiob.crl),
            di_9: gpiod.pd6.into_pull_down_input(&mut gpiod.crl),
            di_10: gpiod.pd5.into_pull_down_input(&mut gpiod.crl),
            di_11: gpiod.pd7.into_pull_down_input(&mut gpiod.crl),
            di_12: gpiod.pd4.into_pull_down_input(&mut gpiod.crl),
            di_13: gpiod.pd1.into_pull_down_input(&mut gpiod.crl),
            di_14: gpiod.pd3.into_pull_down_input(&mut gpiod.crl),
            di_15: gpiod.pd0.into_pull_down_input(&mut gpiod.crl),
            ctrli_0: gpiod.pd2.into_pull_down_input(&mut gpiod.crl),
            ctrli_1: gpiob.pb8.into_pull_down_input(&mut gpiob.crh),
            rsti: gpiob.pb9.into_pull_down_input(&mut gpiob.crh),
            dtsi: gpiob.pb6.into_pull_down_input(&mut gpiob.crl),
            dtei: gpiob.pb14.into_pull_down_input(&mut gpiob.crh),
        };

        let input = sm2m::input::Bus::new(pins);

        // Configure SM2M output bus
        let pins = sm2m::output::Pins {
            do_0: into_output!(gpioc.pc10, &mut gpioc.crh),
            do_1: into_output!(gpioa.pa12, &mut gpioa.crh),
            do_2: into_output!(gpioa.pa10, &mut gpioa.crh),
            do_3: into_output!(gpioa.pa8, &mut gpioa.crh),
            do_4: into_output!(gpioc.pc9, &mut gpioc.crh),
            do_5: into_output!(gpioc.pc8, &mut gpioc.crh),
            do_6: into_output!(gpioc.pc7, &mut gpioc.crl),
            do_7: into_output!(gpiod.pd15, &mut gpiod.crh),
            do_8: into_output!(gpiod.pd14, &mut gpiod.crh),
            do_9: into_output!(gpioc.pc6, &mut gpioc.crl),
            do_10: into_output!(gpiod.pd13, &mut gpiod.crh),
            do_11: into_output!(gpiod.pd8, &mut gpiod.crh),
            do_12: into_output!(gpiod.pd11, &mut gpiod.crh),
            do_13: into_output!(gpiod.pd12, &mut gpiod.crh),
            do_14: into_output!(gpiob.pb15, &mut gpiob.crh),
            do_15: into_output!(gpiod.pd9, &mut gpiod.crh),
            ctrlo_0: into_output!(gpioa.pa11, &mut gpioa.crh),
            ctrlo_1: into_output!(gpioa.pa9, &mut gpioa.crh),
            rdy: into_output!(gpiod.pd10, &mut gpiod.crh),
            ctrl_d: into_output!(gpioc.pc12, &mut gpioc.crh),
            erro: into_output!(pa15, &mut gpioa.crh),
            rste: into_output!(gpiob.pb12, &mut gpiob.crh),
            sete: into_output!(gpioc.pc3, &mut gpioc.crl),
            dteo: into_output!(gpioc.pc11, &mut gpioc.crh),
        };

        let output = sm2m::output::Bus::new(pins);

        // Configure SDMMC
        let sdmmc_detect_pin = gpioa.pa3.into_pull_up_input(&mut gpioa.crl);
        let sdmmc_cs_pin = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let sdmmc_mosi_pin = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
        let sdmmc_sck_pin = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
        let sdmmc_miso_pin = gpioa.pa6;
        let sdmmc_spi = spi::Spi::spi1(
            cx.device.SPI1,
            (sdmmc_sck_pin, sdmmc_miso_pin, sdmmc_mosi_pin),
            &mut afio.mapr,
            spi::Mode {
                phase: spi::Phase::CaptureOnFirstTransition,
                polarity: spi::Polarity::IdleLow,
            },
            25.MHz(),
            clocks,
        );

        let sdmmc_spi = embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs_pin);
        let card = sdmmc::Card::new(sdmmc_spi, sdmmc_detect_pin);

        // Indicate adapter setup completion
        cortex_m::delay::Delay::new(cx.core.SYST, 72_000_000).delay_ms(200);
        indicators.system_error_off();
        indicators.write_off();
        indicators.read_off();

        // Create adapter
        let adapter = adapter::Device::new(input, output, card, indicators);

        // Enable SM2M bus interrupt
        let mut dtli = gpiob.pb13.into_pull_down_input(&mut gpiob.crh); // DTLI
        dtli.make_interrupt_source(&mut afio);
        dtli.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);
        dtli.enable_interrupt(&mut cx.device.EXTI);

        (Shared {}, Local { adapter, dtli }, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue; // Use when printing debug messages
                      // cortex_m::asm::wfi(); // Use in production
        }
    }

    #[task(binds = EXTI15_10, local = [
            adapter,
            dtli,
        ])]
    fn dtli(cx: dtli::Context) {
        cx.local.adapter.run();
        cx.local.dtli.clear_interrupt_pending_bit();
    }
}
