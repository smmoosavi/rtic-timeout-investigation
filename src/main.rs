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

        Systick::timeout_after(
            100.millis(),
            DummyFuture::<1024>::new().inspect_size("dummy_future 1024"),
        )
            .inspect_size("timeout_after 1024")
            .await
            .ok();

        // Output:
        // dummy_future 8 size: 8
        // timeout_after 8 size: 152 (= 8 * 4 + 120)
        // dummy_future 16 size: 16
        // timeout_after 16 size: 184 (= 16 * 4 + 120)
        // dummy_future 32 size: 32
        // timeout_after 32 size: 248 (= 32 * 4 + 120)
        // dummy_future 1024 size: 1024
        // timeout_after 1024 size: 4216 (= 1024 * 4 + 120)

        // # bss section size:
        // run `cargo size --release --target thumbv6m-none-eabi -- -A` to get the size
        //
        // v1: without dummy<1024>
        // v2: with dummy<1024>
        //
        // after add dummy<1024>, .bss section size changed from 1360 to 5328
        // 5328 - 1360 = 3968
        // 4216 - 248 = 3968 (timeout<dummy<1024>> - timeout<dummy<32>>)
        // timeout<dummy<32>> is the largest struct in task1 in v1
        // timeout<dummy<1024>> is the largest struct in task1 in v2
    }
}
