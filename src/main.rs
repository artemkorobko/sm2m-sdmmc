#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

mod buffer;
mod peripherals;
mod state;
mod tasks;

#[global_allocator]
static HEAP: embedded_alloc::Heap = embedded_alloc::Heap::empty();

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use crate::peripherals::*;
    use crate::state::AppState;
    use crate::tasks::*;

    use stm32f1xx_hal::{
        gpio::{self, ExtiPin},
        pac,
        prelude::*,
        spi, timer,
    };

    #[shared]
    struct Shared {
        state: AppState,
        card: sdmmc::Card,
    }

    #[local]
    struct Local {
        timer: timer::CounterMs<pac::TIM1>,
        trigger: gpio::PB13<gpio::Input<gpio::PullDown>>,
        sdmmc_detect_pin: gpio::PA3<gpio::Input<gpio::PullUp>>,
        sdmmc_detached_led: gpio::PA0<gpio::Output>,
        sdmmc_write_led: gpio::PA1<gpio::Output>,
        sdmmc_read_led: gpio::PA2<gpio::Output>,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        {
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 1024;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { super::HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }

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

        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        let (pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure input bus pins
        pb4.into_pull_down_input(&mut gpiob.crl); // DI_8
        gpiob.pb6.into_pull_down_input(&mut gpiob.crl); // DTSI
        gpiob.pb7.into_pull_down_input(&mut gpiob.crl); // DI_0
        gpiob.pb8.into_pull_down_input(&mut gpiob.crh); // CTRLI_1
        gpiob.pb9.into_pull_down_input(&mut gpiob.crh); // RST
        let mut trigger = gpiob.pb13.into_pull_down_input(&mut gpiob.crh); // DTLI
        trigger.make_interrupt_source(&mut afio);
        trigger.enable_interrupt(&mut cx.device.EXTI);
        trigger.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);
        gpiob.pb14.into_pull_down_input(&mut gpiob.crh); // DTEI
        gpiod.pd0.into_pull_down_input(&mut gpiod.crl); // DI_15
        gpiod.pd1.into_pull_down_input(&mut gpiod.crl); // DI_13
        gpiod.pd2.into_pull_down_input(&mut gpiod.crl); // CTRLI_0
        gpiod.pd3.into_pull_down_input(&mut gpiod.crl); // DI_14
        gpiod.pd4.into_pull_down_input(&mut gpiod.crl); // DI_12
        gpiod.pd5.into_pull_down_input(&mut gpiod.crl); // DI_10
        gpiod.pd6.into_pull_down_input(&mut gpiod.crl); // DI_9
        gpiod.pd7.into_pull_down_input(&mut gpiod.crl); // DI_11
        gpioe.pe0.into_pull_down_input(&mut gpioe.crl); // DI_2
        gpioe.pe1.into_pull_down_input(&mut gpioe.crl); // DI_1
        gpioe.pe2.into_pull_down_input(&mut gpioe.crl); // DI_3
        gpioe.pe3.into_pull_down_input(&mut gpioe.crl); // DI_4
        gpioe.pe4.into_pull_down_input(&mut gpioe.crl); // DI_6
        gpioe.pe5.into_pull_down_input(&mut gpioe.crl); // DI_5
        gpioe.pe6.into_pull_down_input(&mut gpioe.crl); // DI_7

        // Configure output bus pins
        gpioa.pa8.into_pull_down_input(&mut gpioa.crh); // DO_3
        gpioa.pa9.into_pull_down_input(&mut gpioa.crh); // CTRLO_1
        gpioa.pa10.into_pull_down_input(&mut gpioa.crh); // DO_2
        gpioa.pa11.into_pull_down_input(&mut gpioa.crh); // CTRLO_0
        gpioa.pa12.into_pull_down_input(&mut gpioa.crh); // DO_1
        pa15.into_pull_down_input(&mut gpioa.crh); // ERR
        gpiob.pb12.into_pull_down_input(&mut gpiob.crh); // RSTE
        gpiob.pb15.into_pull_down_input(&mut gpiob.crh); // DO_14
        gpioc.pc3.into_pull_down_input(&mut gpioc.crl); // SETE
        gpioc.pc6.into_pull_down_input(&mut gpioc.crl); // DO_9
        gpioc.pc7.into_pull_down_input(&mut gpioc.crl); // DO_6
        gpioc.pc8.into_pull_down_input(&mut gpioc.crh); // DO_5
        gpioc.pc9.into_pull_down_input(&mut gpioc.crh); // DO_4
        gpioc.pc10.into_pull_down_input(&mut gpioc.crh); // DO_0
        gpioc.pc11.into_pull_down_input(&mut gpioc.crh); // DTEO
        gpioc.pc12.into_pull_down_input(&mut gpioc.crh); // CTRLD
        gpiod.pd8.into_pull_down_input(&mut gpiod.crh); // DO_11
        gpiod.pd9.into_pull_down_input(&mut gpiod.crh); // DO_15
        gpiod.pd10.into_pull_down_input(&mut gpiod.crh); // RDY
        gpiod.pd11.into_pull_down_input(&mut gpiod.crh); // DO_12
        gpiod.pd12.into_pull_down_input(&mut gpiod.crh); // DO_13
        gpiod.pd13.into_pull_down_input(&mut gpiod.crh); // DO_10
        gpiod.pd14.into_pull_down_input(&mut gpiod.crh); // DO_8
        gpiod.pd15.into_pull_down_input(&mut gpiod.crh); // DO_7

        // Configure LED indicators
        let sdmmc_detached_led = gpioa
            .pa0
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::Low);
        let sdmmc_write_led = gpioa
            .pa1
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);
        let sdmmc_read_led = gpioa
            .pa2
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);

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
                phase: spi::Phase::CaptureOnSecondTransition,
                polarity: spi::Polarity::IdleHigh,
            },
            100.kHz(),
            clocks,
        );

        // Start SDMMC detection timer
        let mut timer = cx.device.TIM1.counter_ms(&clocks);
        timer.start(1.secs()).unwrap();
        timer.listen(timer::Event::Update);

        (
            Shared {
                state: AppState::NotReady,
                card: embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs_pin).into(),
            },
            Local {
                timer,
                trigger,
                sdmmc_detect_pin,
                sdmmc_detached_led,
                sdmmc_write_led,
                sdmmc_read_led,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    extern "Rust" {
        #[task(binds = TIM1_UP, local = [timer, sdmmc_detect_pin, sdmmc_detached_led], shared = [state])]
        fn sdmmc_detect(_: sdmmc_detect::Context);
        #[task(binds = EXTI0, local = [trigger], shared = [card])]
        fn trigger(_: trigger::Context);
    }
}
