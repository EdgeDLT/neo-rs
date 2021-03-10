pub const PRIVATE_KEY_BIN_LEN: usize = 32;
pub const PUBLIC_KEY_BIN_LEN: usize = 33;
pub const ADDRESS_BIN_LEN: usize = 25;
pub const WIF_KEY_BIN_LEN: usize = 38;

pub const PRIVATE_KEY_HEX_LEN: usize = 64;
pub const PUBLIC_KEY_HEX_LEN: usize = 128;
pub const ADDRESS_HEX_LEN: usize = 32;
pub const WIF_KEY_HEX_LEN: usize = 52;

pub const SCRIPT_HASH_BIN_LEN: usize = 20;

pub const SIGNATURE_BIN_LEN:usize = 32;

pub type PublicKeyBin = [u8; PUBLIC_KEY_BIN_LEN];
pub type PrivateKeyBin = [u8; PRIVATE_KEY_BIN_LEN];
pub type AddressBin = [u8; ADDRESS_BIN_LEN];
pub type WifKeyBin = [u8; WIF_KEY_BIN_LEN];
pub type ScriptHashBin = [u8; SCRIPT_HASH_BIN_LEN];
pub type SignatureBin = [u8; SIGNATURE_BIN_LEN];

pub type PublicKeyHex = String;
pub type PrivateKeyHex = String;
pub type AddressHex = String;
pub type WifKeyHex = String;
pub type SignatureHex = String;




