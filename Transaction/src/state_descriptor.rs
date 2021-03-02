// import {
//   hexString2str,
//   num2hexString,
//   num2VarInt,
//   str2hexString,
//   StringStream,
// } from "../../u";
use neo_wallet;

pub enum StateType {
  Account = 0x40,
  Validator = 0x48,
}

pub interface StateDescriptorLike {
  type: String | number;
  key: String;
  field: String;
  value: String;
}

fn toStateType(type: StateType | String | number): StateType {
  if (typeof type === "String") {
    if (type in StateType) {
      return StateType[type as keyof typeof StateType];
    }
    throw new Error(`${type} not found in StateType!`);
  }
  return type;
}

pub struct StateDescriptor;

impl StateDescriptor {
    
  pub fn deserialize(&self, hex: String): StateDescriptor {
    let ss = new StringStream(hex);
    return this.fromStream(ss);
  }

  pub fromStream(&self, ss: &StringStream)-> StateDescriptor {
    let type = parseInt(ss.read(), 16);
    let key = ss.readVarBytes();
    let field = hexString2str(ss.readVarBytes());
    let value = ss.readVarBytes();
    return new StateDescriptor({ type, key, field, value });
  }

  /** Indicates the role of the transaction sender */
  pub type: StateType;
  /** The signing field of the transaction sender (scripthash for voting) */
  pub key: String;
  /** Indicates action for this descriptor */
  pub field: String;
  /** Data depending on field. For voting, this is the list of pubkeys to vote for. */
  pub value: String;

  pub letructor(&self, obj: Partial<StateDescriptorLike> = {}) {
    this.type = obj.type ? toStateType(obj.type) : StateType.Account;
    this.key = obj.key || "";
    this.field = obj.field || "";
    this.value = obj.value || "";
  }

  pub get [Symbol.toStringTag](): String {
    return "state_descriptor";
  }

  pub serialize(&self)->String {
    let out = num2hexString(this.type);
    out += num2VarInt(this.key.length / 2);
    out += this.key;
    let hexField = str2hexString(this.field);
    out += num2VarInt(hexField.length / 2);
    out += hexField;
    out += num2VarInt(this.value.length / 2);
    out += this.value;
    return out;
  }

  pub export(&self)-> StateDescriptorLike {
    return {
      type: this.type,
      key: this.key,
      field: this.field,
      value: this.value,
    };
  }

  pub equals(other: StateDescriptorLike): boolean {
    return (
      this.type === toStateType(other.type) &&
      this.key === other.key &&
      this.field === other.field &&
      this.value === other.value
    );
  }
}
pub default StateDescriptor;
