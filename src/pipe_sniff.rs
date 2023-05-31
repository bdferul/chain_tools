pub trait Pipe {
    fn pipe<O, F: FnOnce(Self) -> O>(self, f: F) -> O
    where
        Self: Sized,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

pub trait Sniff {
    /// # Usage
    /// ```
    /// # use chain_tools::Sniff;
    /// 
    fn sniff<F: FnOnce(&Self)>(self, f: F) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }
}

impl<T> Sniff for T {}
