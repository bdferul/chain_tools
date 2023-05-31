pub trait Ternary {
    fn or<O>(&self, t: O, f: O) -> O;
    fn or_else<O, F1: FnOnce() -> O, F2: FnOnce() -> O>(&self, t: F1, f: F2) -> O;
}

impl Ternary for bool {
    fn or<O>(&self, t: O, f: O) -> O {
        if *self {
            t
        } else {
            f
        }
    }

    fn or_else<O, F1: FnOnce() -> O, F2: FnOnce() -> O>(&self, t: F1, f: F2) -> O {
        if *self {
            t()
        } else {
            f()
        }
    }
}
