pub mod verify;
pub mod nep2;
pub mod nep6;
pub mod signing;
pub mod core;
mod coin;
mod claim_item;
mod asset_balance;
mod private_key;
mod public_key;
mod address;
mod wif;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
