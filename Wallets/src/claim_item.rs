// import Fixed8 from "../../u/Fixed8";

use neo_core::fixed8::fixed8;

#[derive(Ord, PartialOrd, Eq, PartialEq,Debug,Hash)]
pub struct claim_item<'a> {
  claim: fixed8,
  txid: &'a str,
  index: u32,
  value: i32,
  start: Option<u32>,
  end: Option<u32>,
}

/**
 * Contains the information necessary to validate a GAS Claim.
 * It is a reference to a spent coin.
 */
impl claim_item {

  pub fn export(&self)->claim_item {
    claim_item {
      claim: self.claim.clone(),
      txid: self.txid.clone(),
      index: self.index,
      value: self.value,
      start: self.start,
      end: self.end,
    }
  }

  pub fn equals(&self, other: &claim_item)-> bool {

      self.claim.equals(&other.claim) &&
      self.txid == other.txid &&
      self.index == other.index &&
      self.value == other.value &&
      self.start == other.start &&
      self.end == other.end
  }
}