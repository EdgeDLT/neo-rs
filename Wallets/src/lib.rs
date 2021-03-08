pub mod verify;
pub mod nep2;
pub mod nep6;
pub mod signing;
pub mod core;
mod coin;
mod claim_item;
mod asset_balance;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
