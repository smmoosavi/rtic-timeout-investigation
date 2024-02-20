pub trait InspectSize {
    fn inspect_size(self, name: &str) -> Self
    where
        Self: Sized;
}

impl<T> InspectSize for T {
    fn inspect_size(self, name: &str) -> Self
    where
        Self: Sized,
    {
        use rtt_target::rprintln;
        rprintln!("{} size: {}", name, core::mem::size_of::<T>());
        self
    }
}
