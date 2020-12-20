/// Unwrap an expression from a result/option,
/// running the statement given as a second argument
/// if the unwrapping "fails".
///
/// For example:
///
/// ```ignore
/// let foo_inner = unwrap_or!(foo, continue);
/// let foo_inner = unwrap_or!(foo, return Vec::new());
/// ```
#[macro_export]
macro_rules! unwrap_or {
    ($try_this:expr, $or_do:stmt) => {
        match $crate::ToOption::to_option($try_this) {
            Some(val) => val,
            None => { $or_do }
        }
    }
}

