use std::collections::HashMap;

/**
 * RPC Response from test invokes
 */
#[derive(Debug,Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct RPCVMResponse {
  script: String,
  // state: "HALT, BREAK" | "FAULT, BREAK" | "HALT" | "FAULT";
  state:String,
  gas_consumed: String,
  stack: Vec<StackItem>,
}

pub type StackItemParser<T> = HashMap<StackItem, T>;//(item: StackItem) => any;

pub type VMResultParser<T> =  HashMap<RPCVMResponse, [T]>;//(result: RPCVMResponse) => any[];

/**
 * Builds a parser to parse the results of the stack.
 * @param args A list of functions to parse arguments. Each function is mapped to its corresponding StackItem in the result.
 * @returns parser function
 */
pub fn buildParser(...args: StackItemParser[]): VMResultParser {

  return (result: RPCVMResponse) => {
    if (result.stack.length !== args.length) {
      throw new Error(
        `Wrong number of items to parse! Expected ${args.length} but got ${result.stack.length}!`
      );
    }

    return result.stack.map((item, i) => args[i](item));
  };
}

/**
 * This just returns the value of the StackItem.
 */
pub fn NoOpParser<T>(item: &StackItem)->T {
  item.value
}

/**
 * Parses the result to an integer.
 */
pub fn IntegerParser(item: &StackItem): number {
  return parseInt((item.value as string) || "0", 10);
}

/**
 *  Parses the result to a ASCII string.
 */
pub fn StringParser(item: &StackItem): string {
  return hexstring2str(item.value as string);
}

/**
 * Parses the result to a Fixed8.
 */
pub fn Fixed8Parser(item: &StackItem): Fixed8 {
  return Fixed8.fromReverseHex(item.value as string);
}

/**
 * Parses the VM Stack and returns human readable strings. The types are inferred based on the StackItem type.
 * @param res RPC Response
 * @return Array of results
 */
pub fn SimpleParser(res: &RPCVMResponse): any[] {
  return res.stack.map((item) => {
    switch (item.type) {
      case "ByteArray":
        return StringParser(item);
      case "Integer":
        return IntegerParser(item);
      default:
        throw Error(`Unknown type: ${item.type}`);
    }
  });
}
