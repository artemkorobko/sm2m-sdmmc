#![no_std]
#![no_main]

extern crate alloc;

use defmt_rtt as _;
use panic_probe as _;

mod error;
mod mode;
mod peripherals;
mod tasks;

#[global_allocator]
static HEAP: embedded_alloc::Heap = embedded_alloc::Heap::empty();

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use crate::mode::Mode;
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
        mode: Mode,
        card: sdmmc::Card,
        bus: sm2m::Bus,
        trigger: gpio::PB13<gpio::Input<gpio::PullDown>>,
        sdmmc_detect_pin: gpio::PA3<gpio::Input<gpio::PullUp>>,
        indicators: Indicators,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        {
            defmt::info!("Create heap...");
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 1024 * 16;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { super::HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }

        // Configure MCU
        defmt::info!("Configure MCU...");
        let mut afio = cx.device.AFIO.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(16.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .freeze(&mut flash.acr);

        // Configure GPIO
        defmt::info!("Configure GPIO...");
        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        // Disable JTAG
        let (pa15, _, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure SM2M bus
        let sm2m_gpioa_config = sm2m::GPIOAConfig {
            pa8: gpioa.pa8,
            pa9: gpioa.pa9,
            pa10: gpioa.pa10,
            pa11: gpioa.pa11,
            pa12: gpioa.pa12,
            pa15,
            crh: &mut gpioa.crh,
        };

        let sm2m_gpiob_config = sm2m::GPIOBConfig {
            pb4,
            pb6: gpiob.pb6,
            pb7: gpiob.pb7,
            pb8: gpiob.pb8,
            pb9: gpiob.pb9,
            pb12: gpiob.pb12,
            pb14: gpiob.pb14,
            pb15: gpiob.pb15,
            crl: &mut gpiob.crl,
            crh: &mut gpiob.crh,
        };

        let sm2m_gpioc_config = sm2m::GPIOCConfig {
            pc3: gpioc.pc3,
            pc6: gpioc.pc6,
            pc7: gpioc.pc7,
            pc8: gpioc.pc8,
            pc9: gpioc.pc9,
            pc10: gpioc.pc10,
            pc11: gpioc.pc11,
            pc12: gpioc.pc12,
            crl: &mut gpioc.crl,
            crh: &mut gpioc.crh,
        };

        let sm2m_gpiod_config = sm2m::GPIODConfig {
            pd0: gpiod.pd0,
            pd1: gpiod.pd1,
            pd2: gpiod.pd2,
            pd3: gpiod.pd3,
            pd4: gpiod.pd4,
            pd5: gpiod.pd5,
            pd6: gpiod.pd6,
            pd7: gpiod.pd7,
            pd8: gpiod.pd8,
            pd9: gpiod.pd9,
            pd10: gpiod.pd10,
            pd11: gpiod.pd11,
            pd12: gpiod.pd12,
            pd13: gpiod.pd13,
            pd14: gpiod.pd14,
            pd15: gpiod.pd15,
            crl: &mut gpiod.crl,
            crh: &mut gpiod.crh,
        };

        let sm2m_gpioe_config = sm2m::GPIOEConfig {
            pe0: gpioe.pe0,
            pe1: gpioe.pe1,
            pe2: gpioe.pe2,
            pe3: gpioe.pe3,
            pe4: gpioe.pe4,
            pe5: gpioe.pe5,
            pe6: gpioe.pe6,
            crl: &mut gpioe.crl,
        };

        let bus = sm2m::Bus::configure(
            sm2m_gpioa_config,
            sm2m_gpiob_config,
            sm2m_gpioc_config,
            sm2m_gpiod_config,
            sm2m_gpioe_config,
        );

        // Configure LED indicators
        defmt::info!("Configure LED indicators...");
        let indicator_pins = IndicatorPins {
            pa0: gpioa.pa0,
            pa1: gpioa.pa1,
            pa2: gpioa.pa2,
            crl: &mut gpioa.crl,
        };

        let mut indicators = Indicators::configure(indicator_pins);

        // Configure SDMMC
        defmt::info!("Configure SDMMC...");
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
                phase: spi::Phase::CaptureOnSecondTransition,
                polarity: spi::Polarity::IdleHigh,
            },
            100.kHz(),
            clocks,
        );

        let card = sdmmc::Card::from(embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs_pin));

        // Indicate adapter startup
        defmt::info!("Indicate adapter setup");
        indicators.all_on();
        cortex_m::asm::delay(72_000_000 * 2);
        indicators.all_off();

        // Enable SM2M bus interrupt
        defmt::info!("Enable SM2M interrupt");
        let mut trigger = gpiob.pb13.into_pull_down_input(&mut gpiob.crh); // DTLI
        trigger.make_interrupt_source(&mut afio);
        trigger.enable_interrupt(&mut cx.device.EXTI);
        trigger.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);

        defmt::info!("Run");

        (
            Shared {},
            Local {
                mode: Mode::Ready,
                card,
                bus,
                trigger,
                sdmmc_detect_pin,
                indicators,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue; // Use when printing debug messages
                      // cortex_m::asm::wfi(); // Use in production
        }
    }

    use crate::tasks::*;

    extern "Rust" {
        #[task(binds = EXTI0, local = [
            mode,
            card,
            bus,
            trigger,
            sdmmc_detect_pin,
            indicators,
        ])]
        fn command(_: command::Context);
    }
}
