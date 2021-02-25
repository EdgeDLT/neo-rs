#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png")]

// Give relevant error messages if the user tries to enable AArch64 asm on unsupported platforms.
#[cfg(all(
    feature = "asm-aarch64",
    target_arch = "aarch64",
    not(target_os = "linux")
))]
compile_error!("Your OS isn’t yet supported for runtime-checking of AArch64 features.");
#[cfg(all(feature = "asm-aarch64", not(target_arch = "aarch64")))]
compile_error!("Enable the \"asm\" feature instead of \"asm-aarch64\" on non-AArch64 systems.");
#[cfg(all(
    feature = "asm-aarch64",
    target_arch = "aarch64",
    target_feature = "crypto"
))]
compile_error!("Enable the \"asm\" feature instead of \"asm-aarch64\" when building for AArch64 systems with crypto extensions.");
#[cfg(all(
    not(feature = "asm-aarch64"),
    feature = "asm",
    target_arch = "aarch64",
    not(target_feature = "crypto"),
    target_os = "linux"
))]
compile_error!("Enable the \"asm-aarch64\" feature on AArch64 if you want to use asm detected at runtime, or build with the crypto extensions support, for instance with RUSTFLAGS='-C target-cpu=native' on a compatible CPU.");

// extern crate block_buffer;
// extern crate fake_simd as simd;

// #[macro_use]
// extern crate opaque_debug;

// #[macro_use]
pub extern crate digest;

#[cfg(feature = "asm-aarch64")]
extern crate libc;

#[cfg(feature = "asm")]
extern crate sha2_asm;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "asm-aarch64")]
mod aarch64;
mod consts;
mod sha256;
#[cfg(any(not(feature = "asm"), feature = "asm-aarch64", feature = "compress"))]
mod sha256_utils;
mod sha512;
#[cfg(any(not(feature = "asm"), target_arch = "aarch64", feature = "compress"))]
mod sha512_utils;

pub use digest::Digest;
pub use sha256::{Sha224, Sha256};
#[cfg(feature = "compress")]
pub use sha256_utils::compress256;
pub use sha512::{Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};
#[cfg(feature = "compress")]
pub use sha512_utils::compress512;
