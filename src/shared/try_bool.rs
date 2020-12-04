/// A quick macro that you can wrap around expressions where
/// the return type is bool; if the thing you wrap this around is
/// Ok(thing)/Some(thing)/true you'll get the thing back, else
/// this will return false.
#[macro_export]
macro_rules! try_bool {
    ($e:expr) => ({
        use $crate::TryBool;
        match TryBool::try_bool($e) {
            Some(res) => res,
            None => { return false }
        }
    })
}

pub trait TryBool {
    type Output;
    fn try_bool(self) -> Option<Self::Output>;
}

impl <T> TryBool for Option<T> {
    type Output = T;
    fn try_bool(self) -> Option<T> {
        self
    }
}

impl <T,E> TryBool for Result<T,E> {
    type Output = T;
    fn try_bool(self) -> Option<T> {
        self.ok()
    }
}

impl TryBool for bool {
    type Output = bool;
    fn try_bool(self) -> Option<bool> {
        if self {
            Some(true)
        } else {
            None
        }
    }
}