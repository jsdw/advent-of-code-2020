/// Convert a thing into an option:
pub trait ToOption {
    type Item;
    fn to_option(self) -> Option<Self::Item>;
}

impl <T> ToOption for Option<T> {
    type Item = T;
    fn to_option(self) -> Option<Self::Item> {
        self
    }
}

impl <T,E> ToOption for Result<T,E> {
    type Item = T;
    fn to_option(self) -> Option<Self::Item> {
        self.ok()
    }
}

impl ToOption for bool {
    type Item = bool;
    fn to_option(self) -> Option<bool> {
        if self {
            Some(true)
        } else {
            None
        }
    }
}