use core::future::Future;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;

pub struct DummyFuture<const N: usize> {
    _data: [u8; N],
}

impl<const N: usize> DummyFuture<N> {
    pub fn new() -> Self {
        Self { _data: [0; N] }
    }
}

impl<const N: usize> Future for DummyFuture<N> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
