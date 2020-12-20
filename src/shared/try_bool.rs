/// A quick macro that you can wrap around expressions where
/// the return type is bool; if the thing you wrap this around is
/// Ok(thing)/Some(thing)/true you'll get the thing back, else
/// this will return false.
#[macro_export]
macro_rules! try_bool {
    ($e:expr) => ({
        match $crate::ToOption::to_option($e) {
            Some(res) => res,
            None => { return false }
        }
    })
}

