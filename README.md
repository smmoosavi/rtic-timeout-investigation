# Investigate RTIC timeout memory usage

## Introduction

This is a simple project to investigate the memory usage of the [RTIC] timeout function.
[`Systick::timeout_after`][timeout_after] function takes a Future and a duration and returns a new
Future.

```rust
pub async fn timeout_after<F: Future>(
    duration: <Self as Monotonic>::Duration,
    future: F
) -> Result<F::Output, TimeoutError> {}
```

In this repo there is a [simple example][example] that uses the `timeout_after` function.

## Future size

The returned Future size is more than expected.

```rust
#[task]
async fn task1(_: task1::Context) {
    Systick::timeout_after(
        100.millis(),
        DummyFuture::<8>::new(), // 8 bytes
    ) // 152 bytes
    .await
    .ok();
}
```

The `DummyFuture` is a simple Future with generic size. Timeout future and the `DummyFuture` for
various sizes are listed below:

| `DummyFuture` size | Timeout future size |
| ------------------ | ------------------- |
| 8                  | 152                 |
| 16                 | 184                 |
| 32                 | 248                 |
| 1024               | 4216                |
| `n`                | `n * 4 + 120`       |

The size of the timeout future is `n * 4 + 120` where `n` is the size of the input Future. The size
of the timeout future is more than expected. In an optimized build, the size of the timeout future
could be reduced to `n + c` where `n` is the size of the input Future and `c` is a constant.

Probably the source of the problem is this issue [rust-96084] in rust.

## BSS size

The size of BSS section is also increased by the size of the timeout future.

- v1: without `timeout<dummy<1024>>`
- v2: with `timeout<dummy<1024>>`

after add `timeout<dummy<1024>>`, bss section size changed from 1360 to 5328

```
5328 - 1360 = 3968 (bss<v2> - bss<v1>)
4216 - 248 = 3968 (timeout<dummy<1024>> - timeout<dummy<32>>)
```

- `bss<v1>` is the size of BSS section in v1
- `bss<v2>` is the size of BSS section in v2
- `timeout<dummy<32>>` is the largest struct in task1 in v1
- `timeout<dummy<1024>>` is the largest struct in task1 in v2

## Environment

```
RTIC: { version = "2.0.1", features = ["thumbv6-backend"] }
rustc 1.78.0-nightly (d44e3b95c 2024-02-09)
```

[RTIC]: https://github.com/rtic-rs/rtic
[timeout_after]:
  https://docs.rs/rtic-monotonics/latest/rtic_monotonics/systick/struct.Systick.html#method.timeout_after
[example]:
  https://github.com/smmoosavi/rtic-timeout-investigation/blob/b535653423e83887d479dacceb2935ab57410703/src/main.rs#L34
[rust-96084]: https://github.com/rust-lang/rust/issues/96084
