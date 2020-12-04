#[macro_use] mod regex;
#[macro_use] mod try_bool;
mod file_content;

pub use try_bool::TryBool;
pub use file_content::{ FileContent, FileContentOpts };