<<<<<<< HEAD
pub mod base;
pub mod netscape;
=======
mod base;
mod netscape;
>>>>>>> 8b20c0e9d13ff90f8c5ce835e402622171c2192c

pub use self::base::Converter;
pub type Netscape = netscape::Netscape;
