use crate::cli::types::*;


pub const COUNT: OptionType = (
    "[count] -c --count=[count] 'Generates a specified number of wallets'",
    &[],
    &[],
    &[],
);

// Import

pub const ADDRESS: OptionType = (
    "[address] --address=[address] 'Imports a partial wallet for a specified address'",
    &["count", "network", "private", "public"],
    &[],
    &[],
);

pub const PRIVATE: OptionType = (
    "[private] --private=[private key] 'Imports a wallet for a specified private key'",
    &["address", "count", "network", "public"],
    &[],
    &[],
);

pub const PUBLIC: OptionType = (
    "[public] --public=[public key] 'Imports a partial wallet for a specified public key'",
    &["address", "count", "private"],
    &[],
    &[],
);

pub const ACCOUNT: OptionType = (
    "[account] -a --account=[account] 'Imports an HD wallet for a specified account number for bip44 and bip49 derivations'",
    &[],
    &[],
    &[],
);
pub const EXTENDED_PUBLIC: OptionType = (
    "[extended public] --extended-public=[extended public] 'Imports a partial HD wallet for a specified extended public key'",
    &["account", "count", "extended private", "index", "mnemonic", "password"],
    &[],
    &[],
);