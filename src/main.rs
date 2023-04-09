#![no_std]
#![no_main]

extern crate alloc;

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

    use core::{panic::PanicInfo, sync::atomic};
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f1xx_hal::{
        gpio::{self, ExtiPin},
        prelude::*,
        spi,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        state: Mode,
        card: sdmmc::Card,
        input_bus: sm2m::InvertedInputBus,
        output_bus: sm2m::InvertedOutputBus,
        trigger: gpio::PB13<gpio::Input<gpio::PullDown>>,
        sdmmc_detect_pin: gpio::PA3<gpio::Input<gpio::PullUp>>,
        error_led: gpio::PA0<gpio::Output>,
        write_led: gpio::PA1<gpio::Output>,
        read_led: gpio::PA2<gpio::Output>,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        {
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 1024 * 15;
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
        let error_led = gpioa
            .pa0
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);
        let write_led = gpioa
            .pa1
            .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::High);
        let read_led = gpioa
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

        // Steal buses
        // Unsafe: all use pins are set to appropriats states before.
        let input_bus = sm2m::InvertedInputBus::from(unsafe { sm2m::InputBus::steal() });
        let output_bus = sm2m::InvertedOutputBus::from(unsafe { sm2m::OutputBus::steal() });

        (
            Shared {},
            Local {
                state: Mode::Ready,
                card: embedded_sdmmc::SdMmcSpi::new(sdmmc_spi, sdmmc_cs_pin).into(),
                input_bus,
                output_bus,
                trigger,
                sdmmc_detect_pin,
                error_led,
                write_led,
                read_led,
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

    #[inline(never)]
    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        rprintln!("{}", info);
        loop {
            atomic::compiler_fence(atomic::Ordering::SeqCst);
        }
    }

    use crate::tasks::*;

    extern "Rust" {
        #[task(binds = EXTI0, local = [
            trigger,
            state,
            error_led,
            write_led,
            read_led,
            sdmmc_detect_pin,
            card,
            input_bus,
            output_bus
        ])]
        fn command(_: command::Context);
    }
}
