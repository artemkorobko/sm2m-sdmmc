#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[rtic::app(device = stm32f1xx_hal::pac, dispatchers = [TAMPER, PVD, CAN_RX1, CAN_SCE])]
mod app {
    use stm32f1xx_hal::{gpio, pac, prelude::*, timer};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let clocks = cx
            .device
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.MHz())
            .sysclk(72.MHz())
            .freeze(&mut flash.acr);

        (Shared {}, Local {}, init::Monotonics())
    }
}

// XTAL 25Mhz