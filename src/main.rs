#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

mod peripherals;
mod tasks;
mod types;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER])]
mod app {
    use crate::types::{sdmmc, sm2m, status};

    use crate::tasks::*;

    use stm32f1xx_hal::{
        gpio::{self, ExtiPin},
        pac,
        prelude::*,
        spi, timer,
    };

    #[shared]
    struct Shared {
        sdmmc_spi: sdmmc::Bus,
        sdmmc_attached_flag: bool,
        status_display: status::Display,
    }

    #[local]
    struct Local {
        timer: timer::CounterMs<pac::TIM1>,
        sm2m_input_bus: sm2m::InputBus,
        sm2m_output_bus: sm2m::OutputBus,
        sdmmc_detect_pin: sdmmc::DetectPin,
        sdmmc_detect_led: sdmmc::DetectLed,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut afio = cx.device.AFIO.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.MHz())
            .sysclk(72.MHz())
            .pclk1(36.MHz())
            .pclk2(72.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        let sm2m_input_bus = sm2m::InputBus::new(
            gpiob.pb5.into_pull_down_input(&mut gpiob.crl),
            gpiob.pb8.into_pull_down_input(&mut gpiob.crh),
            gpioe.pe1.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe3.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe5.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe7.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe9.into_pull_down_input(&mut gpioe.crh),
            gpiod.pd1.into_pull_down_input(&mut gpiod.crl),
            gpiob.pb6.into_pull_down_input(&mut gpiob.crl),
            gpiob.pb7.into_pull_down_input(&mut gpiob.crl),
            gpiob.pb9.into_pull_down_input(&mut gpiob.crh),
            gpioe.pe2.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe4.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe6.into_pull_down_input(&mut gpioe.crl),
            gpioe.pe8.into_pull_down_input(&mut gpioe.crh),
            gpioe.pe10.into_pull_down_input(&mut gpioe.crh),
        );

        let _ = gpiod.pd2; // КРО
        let _ = gpiod.pd3; // КР1
        let _ = gpiod.pd4; // ВП-И
        let _ = gpiod.pd5; // ВПИП
        let _ = gpiod.pd7; // ОСТ-ИП
        let _ = gpiod.pd6; // ОСБ-И

        let sm2m_output_bus = sm2m::OutputBus::new(
            gpioe.pe14.into_push_pull_output(&mut gpioe.crh),
            gpioa.pa11.into_push_pull_output(&mut gpioa.crh),
            gpioa.pa9.into_push_pull_output(&mut gpioa.crh),
            gpioc.pc9.into_push_pull_output(&mut gpioc.crh),
            gpioc.pc7.into_push_pull_output(&mut gpioc.crl),
            gpiod.pd15.into_push_pull_output(&mut gpiod.crh),
            gpiod.pd13.into_push_pull_output(&mut gpiod.crh),
            gpiod.pd11.into_push_pull_output(&mut gpiod.crh),
            gpiod.pd0.into_push_pull_output(&mut gpiod.crl),
            gpioa.pa12.into_push_pull_output(&mut gpioa.crh),
            gpioa.pa10.into_push_pull_output(&mut gpioa.crh),
            gpioa.pa8.into_push_pull_output(&mut gpioa.crh),
            gpioc.pc8.into_push_pull_output(&mut gpioc.crh),
            gpioc.pc6.into_push_pull_output(&mut gpioc.crl),
            gpiod.pd14.into_push_pull_output(&mut gpiod.crh),
            gpiod.pd12.into_push_pull_output(&mut gpiod.crh),
        );

        let _ = gpiob.pb14.into_push_pull_output(&mut gpiob.crh); // КРО
        let _ = gpiob.pb15.into_push_pull_output(&mut gpiob.crh); // КР1
        let _ = gpiob.pb10.into_push_pull_output(&mut gpiob.crh); // ВНУ
        let _ = gpioe.pe12.into_push_pull_output(&mut gpioe.crh); // ГТ-П
        let _ = gpioe.pe11.into_push_pull_output(&mut gpioe.crh); // ВНС

        let status_display = status::Display::new(
            gpioc
                .pc2
                .into_push_pull_output_with_state(&mut gpioc.crl, gpio::PinState::High),
            gpioc
                .pc0
                .into_push_pull_output_with_state(&mut gpioc.crl, gpio::PinState::High),
            gpioc
                .pc3
                .into_push_pull_output_with_state(&mut gpioc.crl, gpio::PinState::High),
            gpioa
                .pa3
                .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High),
            gpioa
                .pa0
                .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High),
            gpioa
                .pa6
                .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High),
        );

        let sdmmc_detect_led = sdmmc::DetectLed::new(
            gpiod
                .pd7
                .into_push_pull_output_with_state(&mut gpiod.crl, gpio::PinState::Low),
        );

        let mut sdmmc_detect_pin = gpioe.pe0.into_pull_up_input(&mut gpioe.crl);
        sdmmc_detect_pin.make_interrupt_source(&mut afio);
        sdmmc_detect_pin.enable_interrupt(&mut cx.device.EXTI);
        sdmmc_detect_pin.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::RisingFalling);

        let mut timer = cx.device.TIM1.counter_ms(&clocks);
        timer.start(1.secs()).unwrap();
        timer.listen(timer::Event::Update);

        let sdmmc_cs_pin = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let sdmmc_mosi_pin = gpioc.pc12.into_alternate_push_pull(&mut gpioc.crh);
        let sdmmc_sck_pin = gpioc.pc10.into_alternate_push_pull(&mut gpioc.crh);
        let sdmmc_miso_pin = gpioc.pc11;
        let sdmmc_spi = spi::Spi::spi3(
            cx.device.SPI3,
            (sdmmc_sck_pin, sdmmc_miso_pin, sdmmc_mosi_pin),
            &mut afio.mapr,
            spi::Mode {
                phase: spi::Phase::CaptureOnSecondTransition,
                polarity: spi::Polarity::IdleHigh,
            },
            100.kHz(),
            clocks,
        );
        let sdmmc_spi = embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs_pin);

        (
            Shared {
                sdmmc_spi,
                status_display,
                sdmmc_attached_flag: false,
            },
            Local {
                timer,
                sm2m_input_bus,
                sm2m_output_bus,
                sdmmc_detect_pin,
                sdmmc_detect_led,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi()
        }
    }

    extern "Rust" {
        #[task(binds = TIM1_UP, local = [timer, sdmmc_detect_pin, sdmmc_detect_led], shared = [sdmmc_spi, sdmmc_attached_flag])]
        fn sdmmc_detect(_: sdmmc_detect::Context);
    }
}
