#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

mod input;
mod keyboard;
mod output;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use stm32f1xx_hal::{gpio, gpio::ExtiPin, pac, prelude::*, timer};

    use crate::{input, keyboard, output};

    #[shared]
    struct Shared {
        input: input::Bus,
        output: output::Bus,
    }

    #[local]
    struct Local {
        keyboard: keyboard::Keyboard,
        led: gpio::Pin<'D', 7, gpio::Output>,
        timer: timer::CounterUs<pac::TIM1>,
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
            // .use_hse(25.MHz())
            .sysclk(72.MHz())
            .freeze(&mut flash.acr);

        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        // Disable JTAG
        let (pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        let led = gpiod
            .pd7
            .into_push_pull_output_with_state(&mut gpiod.crl, gpio::PinState::High);

        let config = keyboard::Pins {
            reset: gpioe.pe8.into_pull_down_input(&mut gpioe.crh),
            run: gpioe.pe9.into_pull_down_input(&mut gpioe.crh),
            step: gpioe.pe10.into_pull_down_input(&mut gpioe.crh),
        };

        let keyboard = keyboard::Keyboard::new(config);

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
            erro: gpioe.pe15.into_pull_down_input(&mut gpioe.crh),
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

        // Configure RDY interrupt
        let mut trigger = pa15.into_pull_down_input(&mut gpioa.crh); // RDY
        trigger.make_interrupt_source(&mut afio);
        trigger.enable_interrupt(&mut cx.device.EXTI);
        trigger.trigger_on_edge(&mut cx.device.EXTI, gpio::Edge::Falling);

        // Configure keyboard timer
        let mut timer = cx.device.TIM1.counter_us(&clocks);
        timer.start(1.millis()).unwrap();
        timer.listen(timer::Event::Update);

        (
            Shared { input, output },
            Local {
                keyboard,
                led,
                timer,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM1_UP, priority = 2, local = [keyboard, led, timer, debouncer: u32 = 0, notified: bool = false])]
    fn keyboard_timer(cx: keyboard_timer::Context) {
        const MAX_DEBOUNCES: u32 = 10;
        let debouncer = cx.local.debouncer;
        let notified = cx.local.notified;
        let keys = cx.local.keyboard.read_keys();

        if keys.is_pressed() && *debouncer < MAX_DEBOUNCES {
            *debouncer += 1;
        } else if *debouncer > 0 {
            *debouncer -= 1;
        }

        if *debouncer == MAX_DEBOUNCES && !*notified {
            keyboard_handler::spawn(keys).ok();
            *notified = true;
        } else if *debouncer == 0 && *notified {
            *notified = false;
        }

        cx.local.timer.clear_interrupt(timer::Event::Update);
    }

    #[task(priority = 3, shared = [output])]
    fn keyboard_handler(mut cx: keyboard_handler::Context, keys: keyboard::Keys) {
        if keys.reset {
            defmt::println!("Send RESET CMD");
            cx.shared
                .output
                .lock(|output| output.write_reversed(output::Frame::Stop));
        } else if keys.step {
            defmt::println!("Step is pressed");
        }
    }

    #[task(binds = EXTI15_10, priority = 1)]
    fn rdy(_: rdy::Context) {
        defmt::println!("Ready");
    }
}
