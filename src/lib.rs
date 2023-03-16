pub trait FPrint {
    /// Prints itself using [std::fmt::Display]
    /// # Usage
    /// Can be used to print
    /// ```
    /// # use bowel::*;
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
    /// # use bowel::*;
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
