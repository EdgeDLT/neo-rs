use neo_core::stringstream::StringStream;
use std::io::Error;
use neo_core::fixed8::fixed8;
use neo_core::misc::reverseHex;
use neo_crypto::sha2;
use crate::transaction_output::TransactionOutput;
use crate::witness::Witness;

pub trait transaction_param {
    fn deserialize(&self, hex: &str) -> Result<Transaction, Error>;

    fn fromStream(&self, ss: &mut StringStream) -> Result<Transaction, Error>;

    fn serialize(&self) -> Result<String, Error>;

    fn equals(&self, other: &Transaction) -> bool;

    fn export(&self) -> Result<Transaction, Error>
        where
            Self: Sized;
}

pub trait transaction {
    fn symbol() -> &str {
        "Transaction"
    }

    fn hash(&self) -> Result<&str, Error>;

    fn fees(&self) -> fixed8 {fixed(0) }

    // HashMap<String, Ident>
    // fn exclusiveData(&self): { [key: string]: any };

    fn serializeExclusive(&self) -> Result<&str, Error>{Ok("")}

    fn addOutput(&mut self, txOut: &TransactionOutput) -> &self;

    fn addIntent(
        &mut self,
        symbol: &str,
        value: fixed8,
        address: &str,
    ) -> &self;

    fn addAttribute(&mut self, usage: usize, data: &str) -> &self;

    fn addRemark(&mut self, remark: &str) -> &self;

    fn addWitness(&mut self, witness: &Witness) -> &self;

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
    //     fees: fixed8
    // )-> &self;

    /**
     * Serialize the transaction and return it as a hexstring.
     * @param {boolean} signed  - Whether to serialize the signatures. Signing requires it to be serialized without the signatures.
     * @return {string} Hexstring.
     */
    fn serialize(&self, signed: bool) -> Result<&str, Error>;

    /**
     * Signs a transaction.
     * @param {Account|string} signer - Account, privateKey or WIF
     * @return {Transaction} self
     */
    fn sign(&self, signer: &str) -> &self;

    fn export(&self) -> transaction;
}