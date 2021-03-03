
pub mod usage;
pub mod txtype;
pub mod witness;
pub mod transaction_output;
pub mod transaction_input;
pub mod transaction_attribute;
pub mod state_descriptor;
mod utils;
mod txmodel;
mod transaction_base;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
