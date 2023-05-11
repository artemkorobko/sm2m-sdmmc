#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod emulator;
mod input;
mod keyboard;
mod output;

const MAX_DEBUG_BYTES: u16 = 10;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use stm32f1xx_hal::{gpio, gpio::ExtiPin, pac, prelude::*, timer};

    use crate::{emulator, input, keyboard, output};

    #[shared]
    struct Shared {
        emulator: emulator::Machine,
        debug: bool,
    }

    #[local]
    struct Local {
        keyboard: keyboard::Keyboard,
        timer: timer::CounterUs<pac::TIM1>,
        erro: gpio::PB3<gpio::Input<gpio::PullDown>>,
        rdy: gpio::PA15<gpio::Input<gpio::PullDown>>,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let mut afio = cx.device.AFIO.constrain();
        let clocks = cx
            .device
            .RCC
            .constrain()
            .cfgr
            .hclk(72.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        // Disable JTAG
        let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure keyboard
        let config = keyboard::Pins {
            start_write: gpioe.pe8.into_pull_down_input(&mut gpioe.crh),
            start_read: gpioe.pe10.into_pull_down_input(&mut gpioe.crh),
            step: gpioe.pe9.into_pull_down_input(&mut gpioe.crh),
            stop: gpiob.pb2.into_pull_down_input(&mut gpiob.crl),
            debug: gpioe.pe7.into_pull_down_input(&mut gpioe.crl),
        };

        let keyboard = keyboard::Keyboard::new(config);

        // Configure LED
        let led = gpiod
            .pd7
            .into_push_pull_output_with_state(&mut gpiod.crl, gpio::PinState::High);

        // Configure input bus
        let input_pins = input::Pins {
            do_0: gpiod.pd0.into_pull_down_input(&mut gpiod.crl),
            do_1: gpiod.pd1.into_pull_down_input(&mut gpiod.crl),
            do_2: gpiod.pd2.into_pull_down_input(&mut gpiod.crl),
            do_3: gpiod.pd3.into_pull_down_input(&mut gpiod.crl),
            do_4: gpiod.pd4.into_pull_down_input(&mut gpiod.crl),
            do_5: gpiod.pd5.into_pull_down_input(&mut gpiod.crl),
            do_6: gpiod.pd6.into_pull_down_input(&mut gpiod.crl),
            do_7: gpioe.pe11.into_pull_down_input(&mut gpioe.crh),
            do_8: gpioa.pa8.into_pull_down_input(&mut gpioa.crh),
            do_9: gpioc.pc6.into_pull_down_input(&mut gpioc.crl),
            do_10: gpioc.pc7.into_pull_down_input(&mut gpioc.crl),
            do_11: gpiod.pd11.into_pull_down_input(&mut gpiod.crh),
            do_12: gpiod.pd12.into_pull_down_input(&mut gpiod.crh),
            do_13: gpiod.pd13.into_pull_down_input(&mut gpiod.crh),
            do_14: gpiod.pd14.into_pull_down_input(&mut gpiod.crh),
            do_15: gpiod.pd15.into_pull_down_input(&mut gpiod.crh),
            ctrlo_0: gpiob.pb15.into_pull_down_input(&mut gpiob.crh),
            ctrlo_1: gpiob.pb14.into_pull_down_input(&mut gpiob.crh),
            ctrld: gpiob.pb10.into_pull_down_input(&mut gpiob.crh),
            rste: gpioe.pe14.into_pull_down_input(&mut gpioe.crh),
            sete: gpioe.pe13.into_pull_down_input(&mut gpioe.crh),
            dteo: gpioe.pe12.into_pull_down_input(&mut gpioe.crh),
        };

        let input = input::Bus::new(input_pins);

        // Configure output bus
        let output_pins = output::Pins {
            di_0: pb4.into_push_pull_output(&mut gpiob.crl),
            di_1: gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
            di_2: gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
            di_3: gpiob.pb7.into_push_pull_output(&mut gpiob.crl),
            di_4: gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            di_5: gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            di_6: gpioe.pe2.into_push_pull_output(&mut gpioe.crl),
            di_7: gpioe.pe3.into_push_pull_output(&mut gpioe.crl),
            di_8: gpioe.pe4.into_push_pull_output(&mut gpioe.crl),
            di_9: gpioe.pe5.into_push_pull_output(&mut gpioe.crl),
            di_10: gpioe.pe6.into_push_pull_output(&mut gpioe.crl),
            di_11: gpioc.pc13.into_push_pull_output(&mut gpioc.crh),
            di_12: gpioc.pc0.into_push_pull_output(&mut gpioc.crl),
            di_13: gpioc.pc2.into_push_pull_output(&mut gpioc.crl),
            di_14: gpioc.pc3.into_push_pull_output(&mut gpioc.crl),
            di_15: gpioa.pa0.into_push_pull_output(&mut gpioa.crl),
            ctrli_0: gpioa.pa3.into_push_pull_output(&mut gpioa.crl),
            ctrli_1: gpioa.pa5.into_push_pull_output(&mut gpioa.crl),
            dtsi: gpioa.pa6.into_push_pull_output(&mut gpioa.crl),
            dtli: gpioa.pa7.into_push_pull_output(&mut gpioa.crl),
            dtei: gpioc.pc4.into_push_pull_output(&mut gpioc.crl),
            rsti: gpioc.pc5.into_push_pull_output(&mut gpioc.crl),
        };

        let output = output::Bus::new(output_pins);

        // Create emulator
        let debug = true;
        let max_bytes = if debug {
            crate::MAX_DEBUG_BYTES
        } else {
            u16::MAX
        };
        let emulator = emulator::Machine::new(input, output, led, max_bytes, debug);

        // Configure ERRO interrupt
        // let mut erro = gpioe.pe15.into_pull_down_input(&mut gpioe.crh);
        let mut erro = pb3.into_pull_down_input(&mut gpiob.crl);
        erro.make_interrupt_source(&mut afio);
        erro.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);
        erro.enable_interrupt(&mut cx.device.EXTI);

        // Configure RDY interrupt
        let mut rdy = pa15.into_pull_down_input(&mut gpioa.crh);
        rdy.make_interrupt_source(&mut afio);
        rdy.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);
        rdy.enable_interrupt(&mut cx.device.EXTI);

        // Configure keyboard timer
        let mut timer = cx.device.TIM1.counter_us(&clocks);
        timer.start(1.millis()).unwrap();
        timer.listen(timer::Event::Update);

        defmt::println!("Ready");

        (
            Shared { emulator, debug },
            Local {
                keyboard,
                timer,
                erro,
                rdy,
            },
            init::Monotonics(),
        )
    }

    #[task(
        binds = TIM1_UP,
        priority = 1,
        local = [keyboard, timer, debouncer: u32 = 0, notified: bool = false],
    )]
    fn keyboard_timer(cx: keyboard_timer::Context) {
        const MAX_DEBOUNCES: u32 = 10;
        let debouncer = cx.local.debouncer;
        let notified = cx.local.notified;
        let key = cx.local.keyboard.read_key();

        if key.is_some() && *debouncer < MAX_DEBOUNCES {
            *debouncer += 1;
        } else if *debouncer > 0 {
            *debouncer -= 1;
        }

        if *debouncer == MAX_DEBOUNCES && !*notified {
            keyboard_handler::spawn(key.unwrap()).ok();
            *notified = true;
        } else if *debouncer == 0 && *notified {
            *notified = false;
        }

        cx.local.timer.clear_interrupt(timer::Event::Update);
    }

    #[task(priority = 1, shared = [emulator, debug])]
    fn keyboard_handler(cx: keyboard_handler::Context, key: keyboard::Key) {
        let emulator = cx.shared.emulator;
        let debug = cx.shared.debug;

        (emulator, debug).lock(|emulator, debug| match key {
            keyboard::Key::StartRead => emulator.start_read(*debug),
            keyboard::Key::StartWrite => emulator.start_write(*debug),
            keyboard::Key::Step => emulator.step(),
            keyboard::Key::Stop => emulator.stop(),
            keyboard::Key::Debug => {
                *debug = !*debug;
                emulator.set_debug(*debug);
                let max_bytes = if *debug {
                    crate::MAX_DEBUG_BYTES
                } else {
                    u16::MAX
                };
                emulator.set_max_bytes(max_bytes);
                defmt::println!("Debug: {}", debug);
            }
        });
    }

    #[task(binds = EXTI3, priority = 2, local = [erro], shared = [emulator])]
    fn erro(mut cx: erro::Context) {
        cx.shared.emulator.lock(|emulator| {
            if let Some(opcode) = emulator.read() {
                defmt::println!("Received error: {}", opcode);
            } else {
                defmt::println!("Received unknown error");
            }
        });

        cx.local.erro.clear_interrupt_pending_bit();
    }

    #[task(binds = EXTI15_10, priority = 2, local = [rdy], shared = [emulator, debug])]
    fn rdy(cx: rdy::Context) {
        (cx.shared.emulator, cx.shared.debug).lock(|emulator, debug| {
            if !*debug {
                emulator.step();
            }
        });

        cx.local.rdy.clear_interrupt_pending_bit();
    }
}
