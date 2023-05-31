pub trait Pipe {
    fn pipe<O, F: FnOnce(Self) -> O>(self, f: F) -> O
    where
        Self: Sized,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

pub trait XRay {
    fn xray<F: FnOnce(&Self) -> Self>(self, f: F) -> Self;
}

impl<T> XRay for Option<T> {
    fn xray<F: FnOnce(&Self) -> Self>(self, f: F) -> Self {
        if let Ok(x) = self {
            f(x)
        }
    }
}

pub trait FPrint {
    /// Prints itself using [std::fmt::Display]
    /// # Usage
    /// Can be used to print
    /// ```
    /// # use chain_tools::*;
    /// let h = ["Donald", "Tyler", "Noah"]
    ///     .get(1)
    ///     .unwrap()
    ///     .print() // We can print "tyler" here without disrupting the chain
    ///     .bytes()
    ///     .map(|s| s.to_string())
    ///     .collect::<Vec<String>>();
    /// ```
    fn print(self) -> Self;
}

impl<T: std::fmt::Display> FPrint for T {
    fn print(self) -> Self {
        println!("{}", &self);
        self
    }
}

pub trait FDebug {
    /// Prints itself using [std::fmt::Debug]
    /// # Usage
    /// Can be used to print
    /// ```
    /// # use chain_tools::*;
    /// let h = (10..=99)
    ///     .step_by(7)
    ///     .collect::<Vec<i32>>()
    ///     .debug() // We can debug at any link in the chain without breaking it
    ///     .windows(3)
    ///     .map(FDebug::debug);
    /// ```
    fn debug(self) -> Self;
}

impl<T: std::fmt::Debug> FDebug for T {
    fn debug(self) -> Self {
        println!("{:?}", &self);
        self
    }
}
