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
    use rtic_monotonics::systick::{ExtU32, Systick};
    use rtt_target::rtt_init_print;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        rtt_init_print!();

        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(ctx.core.SYST, 36_000_000, systick_mono_token);

        (Shared {}, Local {})
    }

    #[task]
    async fn task1(_: task1::Context) {
        Systick::timeout_after(
            100.millis(),
            DummyFuture::<8>::new().inspect_size("dummy_future 8"),
        )
        .inspect_size("timeout_after 8")
        .await
        .ok();

        Systick::timeout_after(
            100.millis(),
            DummyFuture::<16>::new().inspect_size("dummy_future 16"),
        )
        .inspect_size("timeout_after 16")
        .await
        .ok();

        Systick::timeout_after(
            100.millis(),
            DummyFuture::<32>::new().inspect_size("dummy_future 32"),
        )
        .inspect_size("timeout_after 32")
        .await
        .ok();

        // Output:
        // dummy_future 8 size: 8
        // timeout_after 8 size: 152 (= 8 * 4 + 120)
        // dummy_future 16 size: 16
        // timeout_after 16 size: 184 (= 16 * 4 + 120)
        // dummy_future 32 size: 32
        // timeout_after 32 size: 248 (= 32 * 4 + 120)
    }
}
