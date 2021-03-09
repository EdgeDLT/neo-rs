use std::collections::HashMap;
use neo_core::helper::compare_object;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Protocol {
    magic: u64,
    address_version: usize,
    standby_validators: Vec<String>,
    seed_list: Vec<String>,
    system_fee: HashMap<String, i64>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct ProtocolJSON {
    magic: u64,
    address_version: usize,
    standby_validators: Vec<String>,
    seed_list: Vec<String>,
    system_fee: HashMap<String, i64>,
}


fn compare_arrays<'a, T>(current: &'a [T], other: &'a [T]) -> bool
    where T: Eq {
    if current.len() != other.len() {
        false
    }
    for i in 0..current.len() {
        if current[i] != other[i] {
            false
        }
    }
    true
}

/**
 * Model of the protocol configuration file used by the C# implementation.
 */
impl Protocol {

    //
    // public constructor(config: Partial<ProtocolLike & ProtocolJSON> = {}) {
    //   self.magic = config.magic || config.magic || 0;
    //   self.address_version = config.address_version || config.address_version || 23;
    //   self.standby_validators =
    //     config.standby_validators || config.standby_validators || [];
    //   self.seed_list = config.seed_list || config.seed_list || [];
    //   self.system_fee = Object.assign(
    //     {},
    //     config.system_fee || config.system_fee || DEFAULT_SYSFEE
    //   );
    // }

    pub fn get_symbol() -> &'static str {
        "Protocol"
    }

    pub fn export(&mut self) -> ProtocolJSON {
        ProtocolJSON {
            magic: self.magic,
            address_version: self.address_version,
            standby_validators: self.standby_validators.clone(),
            seed_list: self.seed_list.clone(),
            system_fee: self.system_fee.clone(),
        }
    }

    pub fn equals(&self, other: &Protocol) -> bool {
        self.magic == other.magic &&
            self.address_version == other.address_version &&
            compare_arrays(&self.seed_list, &other.seed_list) &&
            compare_arrays(&self.standby_validators, &other.standby_validators) &&
            compare_object(&self.system_fee, &other.system_fee)
    }
}
