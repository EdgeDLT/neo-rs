// import axios, { AxiosRequestConfig } from "axios";
// import { DEFAULT_REQ } from "../consts";
// import { compareArray } from "../helper";
// import logger from "../logging";
// import { timeout } from "../settings";
// import { BaseTransaction } from "../tx/Transaction/BaseTransaction";

// const log = logger("rpc");
use jsonrpc_core_client::transports::local;
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_derive::rpc;
use neo_tx::transaction_base::BaseTransaction;
use neo_tx::txmodel::Transaction;
use neo_core::helper::compare_array;
use serde_json::Value;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct RPCRequest<T> {
  method: String,
  params: Vec<T>,
  id: i64
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct RPCResponse<T> {
  jsonrpc: String,
  id: usize,
  result: T,
  error: Option<RPCErrorResponse>
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct RPCErrorResponse {
  code: usize,
  message: String,
}

/**
 * Wrapper for querying node RPC
 * @param url Node URL.
 * @param req RPC Request object.
 * @param config Configuration to pass down to axios
 * @returns RPC Response
 */
pub async fn queryRPC<T>(
  url: &str,
  req: &RPCRequest<T>,
  config: AxiosRequestConfig
): Promise<RPCResponse> {

  const body = Object.assign({}, DEFAULT_REQ, req);
  const conf = Object.assign(
    {
      headers: { "Content-Type": "application/json" },
      timeout: timeout.rpc,
    },
    config
  );
  const response = await axios.post(url, body, conf);
  return response.data;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Query<'a> {
    method: &'a str,
    params: Option<Value>,
}

/**
 * A Query object helps us to construct and record requests
 */
impl Query {
  /**
   * @param addr address in Base58 encoding (starting with A)
   */
  pub fn getAccountState(addr: &str)-> Query {
    Query{
      method: "getaccountstate",
      params: None//json!({addr}).into(),
    }}
// json!({"interval": 1000}).into()
  /**
   * @param assetId
   */
  pub fn getAssetState(assetId: &str)->Query {
    Query{
      method: "getassetstate",
      params: None//[assetId],
    }}

  /**
   * self Query returns the specified block either as a hexstring or human readable JSON.
   * @param indexOrHash height or hash of block.
   * @param verbose 0 for hexstring, 1 for JSON. Defaults to 1.
   */
  pub fn getBlock(indexOrHash: usize, verbose:usize)->Query {
    Query{
      method: "getblock",
      params: None //[indexOrHash, verbose],
    }}

  /**
   * self Query returns the hash of a specific block.
   * @param {number} index height of block.
   */
  pub fn getBlockHash(index: usize)->Query {
    Query{
      method: "getblockhash",
      params: None//[index],
    }}

  /**
   * self Query returns the hash of the highest block.
   */
  pub fn getBestBlockHash()-> Query {
    Query{
      method: "getbestblockhash",
      params: None
    }}

  /**
   * self Query returns the current block height.
   */
  pub fn getBlockCount()->Query {
    Query{
      method: "getblockcount",
      params:None
    }}

  /**
   * self Query returns the amount of GAS burnt as fees within a specific block.
   * @param index height of block.
   */
  pub fn getBlockSysFee(index: number)->Query {
    Query{
      method: "getblocksysfee",
      params: None//[index],
    }}

  /**
   * self Query returns the number of other nodes that self node is connected to.
   */
  pub fn getConnectionCount()->Query {
    Query{
      method: "getconnectioncount",
      params:None
    }}
  /**
   * self Query returns information about the smart contract registered at the specific hash.
   * @param scriptHash hash of contract
   */
  pub fn getContractState(scriptHash: &str)->Query {
    Query{
      method: "getcontractstate",
      params: None//[scriptHash],
    }}

  /**
   * self Query returns the list of nodes that self node is connected to.
   */
  pub fn getPeers()->Query {
    Query{
      method: "getpeers",
      params:None
    }}

  /**
   * self Query returns the Transaction hashes of the transactions waiting to be processed at the node.
   */
  pub fn getRawMemPool()->Query {
    Query{
      method: "getrawmempool",
      params:None
    }}

  /**
   * self Query returns information about a specific Transaction in either hexstring or human readable JSON.
   * @param txid hash of the specific Transaction.
   * @param verbose 0 for hexstring, 1 for JSON. Defaults to 1.
   */
  pub fn getRawTransaction(txid: &str, verbose:usize)->Query {
    Query{
      method: "getrawtransaction",
      params: None//[txid, verbose],
    }}

  /**
   * self Query returns the raw value stored at the specific key under a specific contract.
   * @param scriptHash hash of contract.
   * @param key
   */
  pub fn getStorage(scriptHash: &str, key: &str)->Query {
    Query{
      method: "getstorage",
      params: None//[scriptHash, key],
    }}

  /**
   * self Query returns the status of a TransactionOutput. If the output has been spent, self will return null.
   * @param txid hash of Transaction.
   * @param index position of output in the vout array.
   */
  pub fn getTxOut(txid:&str, index: usize)->Query {
    Query{
      method: "gettxout",
      params: None//[txid, index],
    }}

  /**
   * Gets the list of candidates available for voting.
   * @return List of validators
   */
  pub fn getValidators()->Query {
    Query{
      method: "getvalidators",
      params:None
    }}

  /**
   * self Query returns the node version.
   */
  pub fn getVersion()->Query {
    Query{
      method: "getversion",
      params:None
    }}

  /**
   * self Query invokes the VM to run the given contract with the given parameters.
   * @param scriptHash hash of contract to test.
   * @param params parameters to pass into the VM.
   */
  //TODO:
  pub fn invoke(scriptHash: &str, params: &str)->Query {
    Query{
      method: "invoke",
      params: None//[scriptHash, params],
    }}

  /**
   * self Query invokes the VM to run the specific contract with the provided operation and params. Do note that self function only suits contracts with a Main(string, args[]) entry method.
   * @param scriptHash hash of contract to test.
   * @param operation name of operation to call (first argument)
   * @param params parameters to pass (second argument)
   */
  pub fn invokeFunction(
    scriptHash: string,
    operation: string
  )->Query {
    Query{
      method: "invokefunction",
      params: None//[scriptHash, operation, params],
    }}

  /**
   * self Query runs the specific script through the VM.
   * @param script
   */
  pub fn invokeScript(script: &str)->Query {
    Query{
      method: "invokescript",
      params: None//[script],
    }}

  /**
   * self Query transmits the specific Transaction to the node.
   * @param Transaction Transaction as a Transaction object or hexstring.
   */
  pub fn sendRawTransaction(
    transaction: &BaseTransaction
  )->Query {

    let  serialized =transaction.serialize(true).unwrap();
    Query{
      method: "sendrawtransaction",
      params: None//[serialized],
    }}

  /**
   * self Query submits a block for processing.
   * @param block
   */
  pub fn submitBlock(block: &str)->Query {
    Query{
      method: "submitblock",
      params: None//[block],
    }}

  /**
   * self Query submits an address for validation.
   * @param addr Address to validate.
   */
  pub fn validateAddress(addr: &str)->Query {
    Query{
      method: "validateaddress",
      params: None//[addr],
    }}

  /**
   * self Query Returns information of the unspent UTXO assets at the specified address.
   * @param addr Address to get the UTXO
   */
  pub fn getUnspents(addr: &str)->Query {
    Query{
      method: "getunspents",
      params: None//[addr],
    }}

  /**
   * self Query returns unclaimed GAS amount of the specified address.
   * @param addr Address to get the unclaimed gas
   */
  pub fn getUnclaimed(addr: &str)->Query {
    Query{
      method: "getunclaimed",
      params: None//[addr],
    }}

  /**
   * self Query returns claimable GAS information of the specified address.
   * @param addr Address to get the claimable gas
   */
  pub fn getClaimable(addr: &str)->Query {
    Query{
      method: "getclaimable",
      params: None//[addr],
    }}


  pub fn get_id(&self)-> usize {
    return self.req.id;
  }
  pub fn  get_method(&self)->&str {
    self.req.method;
  }

  pub fn  get_params(&self): any[] {
    self.req.params
  }

  // pub fn  constructor(req: &RPCRequest<T>) {
  //   self.req = Object.assign({}, DEFAULT_REQ, req);
  //   self.completed = false;
  // }

  pub fn  get_symbol()->&'static str {
    "Query"
  }

  /**
   * Attaches a parser method to the Query. self method will be used to parse the response.
   */
  // pub fn parseWith(&self, parser: (res: any) => any): self {
  //   self.parse = parser;
  //   return self;
  // }

  /**
   * Executes the Query by sending the RPC request to the provided net.
   * @param url The URL of the node.
   * @param config Request configuration
   */
  // pub fn  async execute(
  //   url: &str
  // ): Promise<any> {
  //
  //   if (self.completed) {
  //     throw new Error("self request has been sent");
  //   }
  //
  //   const response = await queryRPC(url, self.req, config);
  //   self.res = response;
  //   self.completed = true;
  //   if (response.error) {
  //     throw new Error(`${url}: ${response.error.message}`);
  //   }
  //   if (self.parse) {
  //     log.info(`Query[${self.req.method}] successful`);
  //     return self.parse(response.result);
  //   }
  //   return response;
  // }


  pub fn  equals(&self, other: &RPCRequest<T>) -> bool {

      self.req.id == other.id &&
      self.req.method == other.method &&
      compare_array(&self.req.params, &other.params)
  }
}
