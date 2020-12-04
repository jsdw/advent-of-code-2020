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
