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
        gpiob.pb6.into_pull_down_input(&mut gpiob.crl); // Data_Bus_Input_00
        gpioe.pe0.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_01
        gpiob.pb9.into_pull_down_input(&mut gpiob.crh); // Data_Bus_Input_02
        gpioe.pe1.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_03
        gpioe.pe2.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_04
        gpioe.pe4.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_05
        gpioe.pe3.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_06
        gpioe.pe5.into_pull_down_input(&mut gpioe.crl); // Data_Bus_Input_07
        pb4.into_pull_down_input(&mut gpiob.crl); // Data_Bus_Input_08
        gpiod.pd6.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_09
        gpiod.pd5.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_10
        gpiod.pd7.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_11
        gpiod.pd4.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_12
        gpiod.pd1.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_13
        gpiod.pd3.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_14
        gpiod.pd0.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_15
        gpiob.pb8.into_pull_down_input(&mut gpiob.crh); // Reset_Input
        gpiod.pd2.into_pull_down_input(&mut gpiod.crl); // Data_Bus_Input_Control_0
        gpiob.pb7.into_pull_down_input(&mut gpiob.crl); // Data_Bus_Input_Control_1
        gpiob.pb14.into_pull_down_input(&mut gpiob.crh); // Data_Transfer_Control_Completed_Input
        let mut trigger = gpiob.pb13.into_pull_down_input(&mut gpiob.crh); // Data_Transfer_Long_Input
        trigger.make_interrupt_source(&mut afio);
        trigger.enable_interrupt(&mut cx.device.EXTI);
        trigger.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);
        gpiob.pb11.into_pull_down_input(&mut gpiob.crh); // Data_Transfer_Short_Input

        // Configure output bus pins
        gpioc.pc10.into_pull_down_input(&mut gpioc.crh); // Data_Bus_Output_00
        gpioa.pa12.into_pull_down_input(&mut gpioa.crh); // Data_Bus_Output_01
        gpioa.pa10.into_pull_down_input(&mut gpioa.crh); // Data_Bus_Output_02
        gpioa.pa8.into_pull_down_input(&mut gpioa.crh); // Data_Bus_Output_03
        gpioc.pc9.into_pull_down_input(&mut gpioc.crh); // Data_Bus_Output_04
        gpioc.pc8.into_pull_down_input(&mut gpioc.crh); // Data_Bus_Output_05
        gpioc.pc7.into_pull_down_input(&mut gpioc.crl); // Data_Bus_Output_06
        gpiod.pd15.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_07
        gpiod.pd14.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_08
        gpioc.pc6.into_pull_down_input(&mut gpioc.crl); // Data_Bus_Output_09
        gpiod.pd13.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_10
        gpiod.pd8.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_11
        gpiod.pd11.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_12
        gpiod.pd12.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_13
        gpiob.pb15.into_pull_down_input(&mut gpiob.crh); // Data_Bus_Output_14
        gpiod.pd9.into_pull_down_input(&mut gpiod.crh); // Data_Bus_Output_15
        gpiod.pd10.into_pull_down_input(&mut gpiod.crh); // Ready_Output
        gpioe.pe6.into_pull_down_input(&mut gpioe.crl); // External_Set_Output
        gpiob.pb12.into_pull_down_input(&mut gpiob.crh); // External_Reset_Output
        pa15.into_pull_down_input(&mut gpioa.crh); // External_Error_Output
        gpioc.pc12.into_pull_down_input(&mut gpioc.crh); // Data_Bus_Output_Control_State
        gpioa.pa11.into_pull_down_input(&mut gpioa.crh); // Data_Bus_Output_Control_0
        gpioa.pa9.into_pull_down_input(&mut gpioa.crh); // Data_Bus_Output_Control_1
        gpioc.pc11.into_pull_down_input(&mut gpioc.crh); // Data_Transfer_Completed_Output

        // Configure LED indicators
        let sdmmc_detached_led = gpioa
            .pa0
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::Low);

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
