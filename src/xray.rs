/// A placeholder trait intended for depreciation once [result_option_inspect](https://github.com/rust-lang/rust/issues/91345) becomes stable
pub trait XRay {
    type Item;
    fn xray<F: FnOnce(&Self::Item)>(self, f: F) -> Self;
}

impl<T> XRay for Option<T> {
    type Item = T;
    fn xray<F: FnOnce(&T)>(self, f: F) -> Self {
        if let Some(x) = &self {
            f(x)
        }

        self
    }
}

impl<T, E> XRay for Result<T, E> {
    type Item = T;
    fn xray<F: FnOnce(&Self::Item)>(self, f: F) -> Self {
        if let Ok(x) = &self {
            f(x)
        }

        self
    }
}

pub trait XRayErr {
    type ErrItem;
    fn xray_err<F: FnOnce(&Self::ErrItem)>(self, f: F) -> Self;
}

impl<T, E> XRayErr for Result<T, E> {
    type ErrItem = E;
    fn xray_err<F: FnOnce(&Self::ErrItem)>(self, f: F) -> Self {
        if let Err(x) = &self {
            f(x)
        }

        self
    }
}
