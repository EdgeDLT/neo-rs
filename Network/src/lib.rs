pub mod network;
pub mod protocol;
pub mod query;
pub mod rpc_client;
pub mod parse;
mod rpc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
