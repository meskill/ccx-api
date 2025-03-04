#[cfg(feature = "console_formatter")]
pub mod nice_num;
#[cfg(feature = "console_formatter")]
pub use console;
mod error;

pub use error::*;

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}
