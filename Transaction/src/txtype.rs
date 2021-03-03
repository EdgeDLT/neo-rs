
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;


#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum TransactionType {
  MinerTransaction = 0x00,
  IssueTransaction = 0x01,
  ClaimTransaction = 0x02,
  EnrollmentTransaction = 0x20,
  RegisterTransaction = 0x40,
  ContractTransaction = 0x80,
  StateTransaction = 0x90,
  PublishTransaction = 0xd0,
  InvocationTransaction = 0xd1,
}

pub fn toTxType(tp: usize) -> Result<TransactionType, Error> {
   match TransactionType::try_from(te) {
        Ok(tp) => Ok(tp),
        Err(_) => Err(()),
    }
}