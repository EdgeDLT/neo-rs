pub const ADDR_VERSION: &str = "17";

pub const ASSET_ID_NEO: &str = "c56f33fc6ecfcd0c225c4ab356fee59390af8560be0e930faebe74a6daff7c9b";
pub const ASSET_ID_GAS: &str = "602c79718b16e442de58778e148d0b1084e3b2dffd5de6b7b16cee7969282de7";


pub enum AssetType {
    CreditFlag = 0x40,
    DutyFlag = 0x80,
    GoverningToken = 0x00,
    UtilityToken = 0x01,
    Currency = 0x08,
    Share = 0x90,
    // (= DutyFlag | 0x10)
    Invoice = 0x98,
    // (= DutyFlag | 0x18)
    Token = 0x60, // (= CreditFlag | 0x20)
}

pub const CONTRACTS_RPX: &str = "ecc6b20d3ccac1ee9ef109af5a7cdb85706b1df9";
pub const CONTRACTS_TEST_RPX: &str = "5b7074e873973a6ed3708862f219a6fbf4d1c411";
pub const CONTRACTS_TEST_LWTF: &str = "d7678dd97c000be3f33e9362e673101bac4ca654";
pub const CONTRACTS_TEST_NXT: &str = "0b6c1f919e95fe61c17a7612aebfaf4fda3a2214";
pub const CONTRACTS_TEST_RHTT4: &str = "f9572c5b119a6b5775a6af07f1cef5d310038f55";

pub const DEFAULT_RPC_MAIN: &str = "https://seed11.ngd.network:10331";
pub const DEFAULT_RPC_TEST: &str = "https://seed11.ngd.network:20331";

// pub const DEFAULT_REQ = {
//   jsonrpc: "2.0",
//   method: "getblockcount",
//   params: [],
//   id: 1234,
// };

pub const SCRYPT_N: usize = 16384;
pub const SCRYPT_R: u32 = 8;
pub const SCRYPT_P: u32 = 8;
pub const SCRYPT_LOG_N: u8 = 14;
pub const SCRYPT_DK_LEN: usize = 64;

pub const NEP_HEADER_1: u8 = 0x01;
pub const NEP_HEADER_2: u8 = 0x42;
pub const NEP_FLAG: u8 = 0xe0;

// pub const DEFAULT_SYSFEE: { [key: string]: number } = {
pub const DEFAULT_SYSFEE_ENROLLMENT_TRANSACTION: u16 = 1000;
pub const DEFAULT_SYSFEE_ISSUE_TRANSACTION: u16 = 500;
pub const DEFAULT_SYSFEE_PUBLISH_TRANSACTION: u16 = 500;
pub const DEFAULT_SYSFEE_REGISTER_TRANSACTION: u16 = 10000;
// };

// pub const DEFAULT_WALLET = {
//   name: "myWallet",
//   version: "1.0",
//   scrypt: DEFAULT_SCRYPT,
//   extra: null,
// };

// pub const DEFAULT_ACCOUNT_CONTRACT = {
//   script: "",
//   parameters: [
//     {
//       name: "signature",
//       type: "Signature",
//     },
//   ],
//   deployed: false,
// };

pub enum NeoNetwork {
    MAIN,
    TEST,
}


// specified by nep2, same as bip38


// pub const NEP_FLAG: &str  = "e0";

pub const RPC_VERSION: &str = "2.3.2";

pub const TX_VERSION_CLAIM: u32 = 0;
pub const TX_VERSION_CONTRACT: u32 = 0;
pub const TX_VERSION_INVOCATION: u32 = 1;
pub const TX_VERSION_ISSUE: u32 = 0;
pub const TX_VERSION_STATE: u32 = 0;
pub const TX_VERSION_MINER: u32 = 0;
pub const TX_VERSION_ENROLLMENT: u32 = 0;
pub const TX_VERSION_PUBLISH: u32 = 0;
pub const TX_VERSION_REGISTER: u32 = 0;
