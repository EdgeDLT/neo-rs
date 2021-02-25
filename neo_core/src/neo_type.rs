pub const PRIVATE_KEY_BIN_LEN: usize = 32;
pub const PUBLIC_KEY_BIN_LEN: usize = 33;
pub const ADDRESS_BIN_LEN: usize = 25;
pub const WIF_KEY_BIN_LEN: usize = 38;

pub const PRIVATE_KEY_HEX_LEN: usize = 64;
pub const PUBLIC_KEY_HEX_LEN: usize = 128;
pub const ADDRESS_HEX_LEN: usize = 32;
pub const WIF_KEY_HEX_LEN: usize = 38;


pub type PublicKeyBin = [u8; PUBLIC_KEY_BIN_LEN];
pub type PrivateKeyBin = [u8; PRIVATE_KEY_BIN_LEN];
pub type AddressBin = [u8; ADDRESS_BIN_LEN];
pub type WifKeyBin = [u8; WIF_KEY_BIN_LEN];

pub type PublicKeyHex = &'static str;
pub type PrivateKeyHex = &'static str;
pub type AddressHex = &'static str;
pub type WifKeyHex = &'static str;

pub const SCRYPT_N: usize = 16384;
pub const SCRYPT_R: u32 = 8;
pub const SCRYPT_P: u32 = 8;
pub const SCRYPT_LOG_N: u8 = 14;
pub const SCRYPT_DK_LEN: usize = 64;

pub const NEP_HEADER_1: u8 = 0x01;
pub const NEP_HEADER_2: u8 = 0x42;
pub const NEP_FLAG: u8 = 0xe0;




