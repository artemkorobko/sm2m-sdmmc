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
        let mut gpiod = cx.device.GPIOD.split();
        let mut gpioe = cx.device.GPIOE.split();

        // Disable JTAG
        let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        let led = gpiod
            .pd7
            .into_push_pull_output_with_state(&mut gpiod.crl, gpio::PinState::High);

        let config = keyboard::KeyboardPins {
            reset: pb3.into_pull_down_input(&mut gpiob.crl),
            step: gpioa.pa3.into_pull_down_input(&mut gpioa.crl),
            // pa5: gpioa.pa5.into_pull_down_input(&mut gpioa.crl),
        };

        let keyboard = keyboard::Keyboard::configure(config);

        // Configure input bus
        let gpioa_pins = input::GPIOAPins {
            ctrlo_0: gpioa.pa6.into_pull_down_input(&mut gpioa.crl),
            ctrlo_1: gpioa.pa7.into_pull_down_input(&mut gpioa.crl),
            ctrld: gpioa.pa8.into_pull_down_input(&mut gpioa.crh),
            erro: gpioa.pa9.into_pull_down_input(&mut gpioa.crh),
            rste: gpioa.pa10.into_pull_down_input(&mut gpioa.crh),
            sete: gpioa.pa11.into_pull_down_input(&mut gpioa.crh),
            dteo: gpioa.pa12.into_pull_down_input(&mut gpioa.crh),
        };

        let gpioe_pins = input::GPIOEPins {
            do_0: gpioe.pe0.into_pull_down_input(&mut gpioe.crl),
            do_1: gpioe.pe1.into_pull_down_input(&mut gpioe.crl),
            do_2: gpioe.pe2.into_pull_down_input(&mut gpioe.crl),
            do_3: gpioe.pe3.into_pull_down_input(&mut gpioe.crl),
            do_4: gpioe.pe4.into_pull_down_input(&mut gpioe.crl),
            do_5: gpioe.pe5.into_pull_down_input(&mut gpioe.crl),
            do_6: gpioe.pe6.into_pull_down_input(&mut gpioe.crl),
            do_7: gpioe.pe7.into_pull_down_input(&mut gpioe.crl),
            do_8: gpioe.pe8.into_pull_down_input(&mut gpioe.crh),
            do_9: gpioe.pe9.into_pull_down_input(&mut gpioe.crh),
            do_10: gpioe.pe10.into_pull_down_input(&mut gpioe.crh),
            do_11: gpioe.pe11.into_pull_down_input(&mut gpioe.crh),
            do_12: gpioe.pe12.into_pull_down_input(&mut gpioe.crh),
            do_13: gpioe.pe13.into_pull_down_input(&mut gpioe.crh),
            do_14: gpioe.pe14.into_pull_down_input(&mut gpioe.crh),
            do_15: gpioe.pe15.into_pull_down_input(&mut gpioe.crh),
        };

        let input = input::Bus::new(gpioa_pins, gpioe_pins);

        // Configure output bus
        let gpiob_pins = output::GPIOBPins {
            ctrli_0: gpiob.pb0.into_push_pull_output(&mut gpiob.crl),
            ctrli_1: gpiob.pb1.into_push_pull_output(&mut gpiob.crl),
            dtsi: pb4.into_push_pull_output(&mut gpiob.crl),
            dtli: gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
            dtei: gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
            di_7: gpiob.pb7.into_push_pull_output(&mut gpiob.crl),
            di_8: gpiob.pb8.into_push_pull_output(&mut gpiob.crh),
            di_9: gpiob.pb9.into_push_pull_output(&mut gpiob.crh),
            di_10: gpiob.pb10.into_push_pull_output(&mut gpiob.crh),
            rsti: gpiob.pb14.into_push_pull_output(&mut gpiob.crh),
        };

        let gpiod_pins = output::GPIODPins {
            di_0: gpiod.pd0.into_push_pull_output(&mut gpiod.crl),
            di_1: gpiod.pd1.into_push_pull_output(&mut gpiod.crl),
            di_2: gpiod.pd2.into_push_pull_output(&mut gpiod.crl),
            di_3: gpiod.pd3.into_push_pull_output(&mut gpiod.crl),
            di_4: gpiod.pd4.into_push_pull_output(&mut gpiod.crl),
            di_5: gpiod.pd5.into_push_pull_output(&mut gpiod.crl),
            di_6: gpiod.pd6.into_push_pull_output(&mut gpiod.crl),
            di_11: gpiod.pd11.into_push_pull_output(&mut gpiod.crh),
            di_12: gpiod.pd12.into_push_pull_output(&mut gpiod.crh),
            di_13: gpiod.pd13.into_push_pull_output(&mut gpiod.crh),
            di_14: gpiod.pd14.into_push_pull_output(&mut gpiod.crh),
            di_15: gpiod.pd15.into_push_pull_output(&mut gpiod.crh),
        };

        let output = output::Bus::new(gpiob_pins, gpiod_pins);

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

    #[task(priority = 3)]
    fn keyboard_handler(_: keyboard_handler::Context, keys: keyboard::Keys) {
        if keys.reset {
            defmt::println!("Reset is pressed");
        } else if keys.step {
            defmt::println!("Step is pressed");
        }
    }

    #[task(binds = EXTI15_10, priority = 1)]
    fn rdy(_: rdy::Context) {
        defmt::println!("Ready");
    }
}
