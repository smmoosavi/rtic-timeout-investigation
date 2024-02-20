#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod inspect_size;

use panic_rtt_target as _;

#[rtic::app(device = stm32l0xx_hal::pac, peripherals = true, dispatchers = [SPI1, SPI2])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        rtt_init_print!();

        rprintln!("Hello, world!");

        (Shared {}, Local {})
    }
}
