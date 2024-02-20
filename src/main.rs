#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod dummy_future;
mod inspect_size;

use panic_rtt_target as _;

#[rtic::app(device = stm32l0xx_hal::pac, peripherals = true, dispatchers = [SPI1, SPI2])]
mod app {
    use crate::dummy_future::DummyFuture;
    use crate::inspect_size::InspectSize;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        rtt_init_print!();

        let d = DummyFuture::<8>::new();
        d.inspect_size("dummy_future");

        (Shared {}, Local {})
    }
}
