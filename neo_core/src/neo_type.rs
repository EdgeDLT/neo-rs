pub const PRIVATE_KEY_BIN_LEN: usize = 32;
pub const PUBLIC_KEY_BIN_LEN: usize = 33;
pub const ADDRESS_BIN_LEN: usize = 25;
pub const WIF_KEY_BIN_LEN: usize = 38;

pub const PRIVATE_KEY_HEX_LEN: usize = 64;
pub const PUBLIC_KEY_HEX_LEN: usize = 128;
pub const ADDRESS_HEX_LEN: usize = 32;
pub const WIF_KEY_HEX_LEN: usize = 38;

pub const SCRIPT_HASH_BIN_LEN: usize = 20;

pub const SIGNATURE_BIN_LEN:usize = 32;

pub type PublicKeyBin<'a> = &'a[u8; PUBLIC_KEY_BIN_LEN];
pub type PrivateKeyBin<'a> = &'a[u8; PRIVATE_KEY_BIN_LEN];
pub type AddressBin<'a> = &'a[u8; ADDRESS_BIN_LEN];
pub type WifKeyBin<'a> = &'a[u8; WIF_KEY_BIN_LEN];
pub type ScriptHashBin<'a> = &'a[u8; SCRIPT_HASH_BIN_LEN];
pub type SignatureBin<'a> = &'a[u8; SIGNATURE_BIN_LEN];

pub type PublicKeyHex = &'static str;
pub type PrivateKeyHex = &'static str;
pub type AddressHex = &'static str;
pub type WifKeyHex = &'static str;
pub type SignatureHex = &'static str;




