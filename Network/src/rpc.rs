#![deny(missing_docs)]

extern crate jsonrpc_core as rpc;
use serde;
use serde_json;
use tokio::runtime::Runtime;
use url::Url;
use serde_json::Value;
use self::rpc::FutureResult;
use std::error::Error;


/// Rpc trait
#[rpc(client)]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String, Error>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64, Error>;

	/// Ping server expect return in interval ms
	#[rpc(name = "ping", raw_params)]
	fn ping(&self, params: Value) -> Result<String, Error>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> FutureResult<String, Error>;
}


fn main() {
        let mut rt = Runtime::new().unwrap();

	let client_url = Url::parse("ws://127.0.0.1:8888/kurento").unwrap();
	let client = rt.block_on(ws::connect::<gen_client::Client>(&client_url)).unwrap();

	let mut map = serde_json::map::Map::new();
	map.insert("interval".to_string(), 1000.into());

	client
           .clone()
           .ping(json!({"interval": 1000}).into())
           .map(|res| println!("ping = {}", res))
           .wait()
	   .unwrap();

	rt.shutdown_now().wait().unwrap();
}