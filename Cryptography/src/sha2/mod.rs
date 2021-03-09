pub extern crate digest;

#[cfg(feature = "std")]
extern crate std;

mod consts;
mod sha256;
mod sha256_utils;
mod sha512;
mod sha512_utils;

pub use digest::Digest;
pub use sha256::{Sha224, Sha256};
#[cfg(feature = "compress")]
pub use sha256_utils::compress256;
pub use sha512::{Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
#[cfg(feature = "compress")]
pub use sha512_utils::compress512;
