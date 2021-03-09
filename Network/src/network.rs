use crate::protocol::{Protocol, ProtocolJSON};
use std::collections::HashMap;
use neo_core::helper::{compare_unsorted_plain_arrays, compare_object};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Network<T> {
  name: String,
  protocol: Protocol,
  nodes: Vec<T>,
  extra: HashMap<String,String>,// { [key: string]: string };
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct NetworkJSON {
  name: String,
  protocol_configuration: ProtocolJSON,
  nodes: Vec<String>,
  extra_configuration:HashMap<String,String>
}

/**
 * Network interface representing a NEO blockchain network.
 * self inherits from the network.protocol file used in the C# implementation and adds in additional configurations.
 * @param config Network JS object
 */
impl Network<T> {

  // public constructor(
  //   config: Partial<Network & NetworkJSON> = {},
  //   name = null
  // ) {
  //   self.name = config.name || config.name || name || "RandomNet";
  //   const protocolLike = Object.assign(
  //     {},
  //     config.protocol || config.protocol_configuration || {}
  //   );
  //   self.protocol = new Protocol(protocolLike);
  //   self.nodes = config.nodes || config.nodes || [];
  //   self.extra = Object.assign(
  //     {},
  //     config.extra_configuration || config.extra || {}
  //   );
  // }

  /**
   * Exports the class as a JSON format.
   */
  pub fn export(&mut self) -> NetworkJSON {
    NetworkJSON {
      protocol_configuration: self.protocol.export(),
      name: self.name.clone(),
      extra_configuration: self.extra.clone(),
      nodes: self.nodes.clone,
    }
  }

  pub fn equals<T>(&self, other: &Network<T>) -> bool {

      self.name == other.name &&
      self.protocol.equals(&other.protocol) &&
      compare_unsorted_plain_arrays(&self.nodes, &other.nodes) &&
      compare_object(&self.extra, &other.extra)

  }
}