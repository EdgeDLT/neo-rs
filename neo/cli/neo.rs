use crate::cli::{flag, option, subcommand, types::*, CLIError, CLI};

use crate::neo_core::nep2::nep2;
use crate::neo_core::KeyPair;

use clap::{ArgMatches, Values};
use colored::*;
use rand::{rngs::StdRng, Rng};
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{fmt, fmt::Display, str::FromStr};

/// Represents a generic wallet to output
#[derive(Serialize, Debug, Default)]
struct NeoWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hex: Option<String>,
}

impl NeoWallet {
    pub fn new() -> NeoWallet {
        // let mut key = nep2::new();
        // let private_key = NeoPrivateKey::new(rng)?;
        // let public_key = private_key.to_public_key();
        // let address = public_key.to_address(&NeoFormat::Standard)?;
        Self {
            // private_key: Some(private_key.to_string()),
            // public_key: Some(public_key.to_string()),
            // address: Some(address.to_string()),
            ..Default::default()
        }
    }

    pub fn from_private_key(private_key: &str) -> Result<Self, CLIError> {
        // let private_key = NeoPrivateKey::from_str(private_key)?;
        // let public_key = private_key.to_public_key();
        // let address = public_key.to_address(&NeoFormat::Standard)?;
        Ok(Self {
            // private_key: Some(private_key.to_string()),
            // public_key: Some(public_key.to_string()),
            // address: Some(address.to_string()),
            ..Default::default()
        })
    }

    pub fn from_public_key(public_key: &str) -> Result<Self, CLIError> {
        // let public_key = NeoPublicKey::from_str(public_key)?;
        // let address = public_key.to_address(&NeoFormat::Standard)?;
        Ok(Self {
            // public_key: Some(public_key.to_string()),
            // address: Some(address.to_string()),
            ..Default::default()
        })
    }

    pub fn from_address(address: &str) -> Result<Self, CLIError> {
        // let address = NeoAddress::from_str(address)?;
        Ok(Self {
            // address: Some(address.to_string()),
            ..Default::default()
        })
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Display for NeoWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = [
            match &self.path {
                Some(path) => format!("      {}                 {}\n", "Path".cyan().bold(), path),
                _ => "".to_owned(),
            },
            match &self.password {
                Some(password) => format!("      {}             {}\n", "Password".cyan().bold(), password),
                _ => "".to_owned(),
            },
            match &self.private_key {
                Some(private_key) => format!("      {}          {}\n", "Private Key".cyan().bold(), private_key),
                _ => "".to_owned(),
            },
            match &self.public_key {
                Some(public_key) => format!("      {}           {}\n", "Public Key".cyan().bold(), public_key),
                _ => "".to_owned(),
            },
            match &self.address {
                Some(address) => format!("      {}              {}\n", "Address".cyan().bold(), address),
                _ => "".to_owned(),
            },
            match &self.transaction_id {
                Some(transaction_id) => format!("      {}       {}\n", "Transaction Id".cyan().bold(), transaction_id),
                _ => "".to_owned(),
            },
            match &self.transaction_hex {
                Some(transaction_hex) => {
                    format!("      {}      {}\n", "Transaction Hex".cyan().bold(), transaction_hex)
                }
                _ => "".to_owned(),
            },
        ]
        .concat();

        // Removes final new line character
        let output = output[..output.len() - 1].to_owned();
        write!(f, "\n{}", output)
    }
}

/// Represents parameters for an Neo transaction input
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NeoInput {
    pub to: String,
    pub value: String,
    pub gas: String,
    #[serde(rename(deserialize = "gasPrice"))]
    pub gas_price: String,
    pub nonce: u64,
    pub data: Option<String>,
}

/// Represents options for an Neo wallet
#[derive(Clone, Debug, Serialize)]
pub struct NeoOptions {
    // Standard command
    count: usize,
    json: bool,
    subcommand: Option<String>,
    derivation: String,
    index: u32,
    indices: u32,
    password: Option<String>,
    path: Option<String>,
    // Import subcommand
    address: Option<String>,
    private: Option<String>,
    public: Option<String>,
}

impl Default for NeoOptions {
    fn default() -> Self {
        Self {
            // Standard command
            count: 1,
            json: false,
            subcommand: None,
            derivation: "neo".into(),
            index: 0,
            indices: 1,
            password: None,
            path: None,
            // Import subcommand
            address: None,
            private: None,
            public: None,
        }
    }
}

impl NeoOptions {
    fn parse(&mut self, arguments: &ArgMatches, options: &[&str]) {
        options.iter().for_each(|option| match *option {
        //     "address" => self.address(arguments.value_of(option)),
        //     "count" => self.count(clap::value_t!(arguments.value_of(*option), usize).ok()),
        //     "createrawtransaction" => self.create_raw_transaction(arguments.value_of(option)),
        //     "json" => self.json(arguments.is_present(option)),
        //     "index" => self.index(clap::value_t!(arguments.value_of(*option), u32).ok()),
        //     "indices" => self.indices(clap::value_t!(arguments.value_of(*option), u32).ok()),
        //     "password" => self.password(arguments.value_of(option)),
        //     "private" => self.private(arguments.value_of(option)),
        //     "public" => self.public(arguments.value_of(option)),
        //     "signrawtransaction" => self.sign_raw_transaction(arguments.values_of(option)),
            _ => (),
        });
    }

    fn address(&mut self, argument: Option<&str>) {
        if let Some(address) = argument {
            self.address = Some(address.to_string());
        }
    }

    fn count(&mut self, argument: Option<usize>) {
        if let Some(count) = argument {
            self.count = count;
        }
    }

    /// Sets `json` to the specified boolean value, overriding its previous state.
    fn json(&mut self, argument: bool) {
        self.json = argument;
    }


    /// Sets `password` to the specified password, overriding its previous state.
    /// If the specified argument is `None`, then no change occurs.
    fn password(&mut self, argument: Option<&str>) {
        if let Some(password) = argument {
            self.password = Some(password.to_string());
        }
    }

    fn private(&mut self, argument: Option<&str>) {
        if let Some(private_key) = argument {
            self.private = Some(private_key.to_string());
        }
    }

    fn public(&mut self, argument: Option<&str>) {
        if let Some(public_key) = argument {
            self.public = Some(public_key.to_string())
        }
    }
}

pub struct NeoCLI;

impl CLI for NeoCLI {
    type Options = NeoOptions;

    const NAME: NameType = "neo";
    const ABOUT: AboutType = "Generates a Neo wallet (include -h for more options)";
    const FLAGS: &'static [FlagType] = &[flag::JSON];
    const OPTIONS: &'static [OptionType] = &[option::COUNT];
    const SUBCOMMANDS: &'static [SubCommandType] = &[
    ];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(arguments: &ArgMatches) -> Self::Options {
        let mut options = NeoOptions::default();
        options.parse(arguments, &["count", "json"]);

        match arguments.subcommand() {
            ("import", Some(arguments)) => {
                options.subcommand = Some("import".into());
                options.parse(arguments, &["json"]);
                options.parse(arguments, &["address", "private", "public"]);
            }
            ("transaction", Some(arguments)) => {
                options.subcommand = Some("transaction".into());
                options.parse(arguments, &["createrawtransaction", "network", "signrawtransaction"]);
            }
            ("neo", Some(arguments)) => {
                let kp = KeyPair::KeyPair::new();
                println!("{}",kp);
                // println!("-----------");
                // options.subcommand = Some("transaction".into());
                // options.parse(arguments, &["createrawtransaction", "network", "signrawtransaction"]);
            }
            _ => {}
        };

        options
    }

    #[cfg_attr(tarpaulin, skip)]
    fn print(options: Self::Options) {
        fn output(options: NeoOptions) -> Result<(), CLIError> {
            let wallets = match options.subcommand.as_ref().map(String::as_str) {
                Some("import") => {
                    if let Some(private_key) = options.private {
                        vec![NeoWallet::from_private_key(&private_key)?]
                    } else if let Some(public_key) = options.public {
                        vec![NeoWallet::from_public_key(&public_key)?]
                    } else if let Some(address) = options.address {
                        vec![NeoWallet::from_address(&address)?]
                    } else {
                        vec![]
                    }
                }
                _ => (0..options.count)
                    .flat_map(|_| match NeoWallet::new() {
                        wallet => vec![wallet],
                        _ => vec![],
                    })
                    .collect(),
            };

            Ok(())
        }
    }
}
