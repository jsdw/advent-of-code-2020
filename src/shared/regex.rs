/// A streamlined macro for compile-once "local" regexs.
///
/// Usage:
///
/// ```ignore
/// let re = regex!("some-pattern");
/// ```
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}