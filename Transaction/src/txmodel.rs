use std::io::Error;

use neo_core::fixed8::Fixed8;
use neo_core::misc::reverse_hex;
use neo_core::stringstream::StringStream;
use neo_crypto::sha2;

use crate::transaction_output::TransactionOutput;
use crate::witness::Witness;

pub trait TransactionParam {
    fn deserialize(&self, hex: &str) -> Result<dyn Transaction, Error>;

    fn from_stream(&self, ss: &mut StringStream) -> Result<dyn Transaction, Error>;

    fn serialize(&self) -> Result<String, Error>;

    fn equals(&self, other: &dyn Transaction) -> bool;

    fn export(&self) -> Result<dyn Transaction, Error>
        where
            Self: Sized;
}

pub trait Transaction {
    fn symbol() -> &str {
        "Transaction"
    }

    fn hash(&self) -> String;

    fn fees(&self) -> Fixed8 { fixed(0) }

    // HashMap<String, Ident>
    // fn exclusiveData(&self): { [key: string]: any };

    fn serialize_exclusive(&self) -> Result<&str, Error> { Ok("") }

    fn add_output(mut self, tx_out: &TransactionOutput) -> Self;

    fn add_intent(
        mut self,
        symbol: &str,
        value: Fixed8,
        address: &str,
    ) -> Self;

    fn add_attribute(mut self, usage: usize, data: &str) -> Self;

    fn add_remark(mut self, remark: &str) -> Self;

    fn add_witness(mut self, witness: &Witness) -> Self;

    /**
     * Calculate the inputs required based on existing outputs provided. Also takes into account the fees required through the gas property.
     * @param balance Balance to retrieve inputs from.
     * @param strategy
     * @param fees Additional network fees. Invocation gas and tx fees are already included automatically so self is additional fees for priority on the network.
     */
    // fn calculate(
    //     &self,
    //     balance: Balance,
    //     strategy?: calculationStrategyFunction,
    //     fees: Fixed8
    // )-> &self;

    /**
     * Serialize the Transaction and return it as a hexstring.
     * @param {boolean} signed  - Whether to serialize the signatures. Signing requires it to be serialized without the signatures.
     * @return {string} Hexstring.
     */
    fn serialize(&self, signed: bool) -> Result<&str, Error>;

    /**
     * Signs a Transaction.
     * @param {Account|string} signer - Account, privateKey or WIF
     * @return {Transaction} self
     */
    fn sign(&self, signer: &str) -> &self;

    fn export(&self) -> dyn Transaction;
}