use neo_core::convert::num2hexstring;
use neo_core::fixed8::Fixed8;
use neo_core::misc::reverse_hex;
use neo_core::no_std::io::Error;
use neo_crypto::sha2;

use crate::transaction_attribute::TransactionAttribute;
use crate::transaction_input::TransactionInput;
use crate::transaction_output::TransactionOutput;
use crate::txmodel::Transaction;
use crate::txtype::{toTxType, TransactionType};
use crate::usage::{toTxAttrUsage, TxAttrUsage};
use crate::witness::Witness;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct BaseTransaction {
    tx_type: TransactionType,
    version: u8,
    attributes: Vec<TransactionAttribute>,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    scripts: Vec<Witness>,
}

impl Transaction for BaseTransaction {
    fn hash(&self) -> String {
        reverse_hex(sha2::digest(self.serialize(false)))
    }


    fn add_output(mut self, txOut: &TransactionOutput) -> Self {
        self.outputs.push(txOut.clone());
        self
    }

    fn add_intent(mut self, symbol: &str, value: Fixed8, address: &str) -> Self {
        self.outputs.push(TransactionOutput.fromIntent(symbol, value, address));
        self
    }

    fn add_attribute(mut self, usage: usize, data: &str) -> Self {
        self.attributes.push(TransactionAttribute { usage: toTxAttrUsage(usage)?, data });
        self
    }

    fn add_remark(mut self, remark: &str) -> Self {
        let hexRemark = str2hexstring(&remark);
        self.add_attribute(TxAttrUsage::Remark as usize, hexRemark);
        self
    }

    fn add_witness(mut self, witness: &Witness) -> Self {
        self.scripts.push(witness.clone());

        // self.scripts = self.scripts.sort(
        //     (w1, w2) => parseInt(w1.scriptHash, 16) - parseInt(w2.scriptHash, 16)
        // );
        self.scripts.sort_by(|a, b| a.get_script_hash().cmp(&b.get_script_hash()));


        self
    }

    fn serialize(&self, signed: bool) -> Result<&str, Error> {
        unimplemented!()
    }

    fn sign(&self, signer: &str) -> &self {
        unimplemented!()
    }

    fn export(&self) -> Box<dyn Transaction> {
        unimplemented!()
    }
}

impl BaseTransaction {
    pub const tx_type: TransactionType = toTxType(0x00).unwrap();


    pub fn hash(&self) -> String {
        reverse_hex(sha2::digest(self.serialize(false)))
    }

    fn fees(&self) -> Fixed8 { Fixed8(0) }

    // fn exclusiveData(&self): { [key: string]: any };

    pub fn serialize_exclusive(&mut self) -> Result<&str, Error> {};

    pub fn add_output(&mut self, txOut: &TransactionOutput) -> &self {
        self.outputs.push(TransactionOutput(txOut));

        &self
    }

    /**
     * Adds a TransactionOutput. TransactionOutput can be given as a TransactionOutput object or as human-friendly values. This is detected by the number of arguments provided.
     * @param symbol The symbol of the asset (eg NEO or GAS).
     * @param value The value to send.
     * @param address The address to send to.
     */
    pub fn add_intent(
        mut self,
        symbol: &str,
        value: &Fixed8,
        address: &str,
    ) -> Self {
        self.outputs.push(TransactionOutput.fromIntent(symbol, value, address));
        self
    }

    /**
     * Add an attribute.
     * @param usage The usage type. Do refer to txAttrUsage enum values for all available options.
     * @param data The data as hexstring.
     */
    pub fn add_attribute(&self, usage: number, data: string): self {
    if (typeof data != = "string") {
    throw new TypeError("data should be formatted as string!");
    }

    }

    /**
     * Add a remark.
     * @param remark A remark in ASCII.
     */
    pub fn add_remark(&self, remark: string) -> &self { &self }

    /**
     * Adds an Witness to the Transaction and automatically sorts the witnesses according to scripthash.
     * @param witness The Witness object to add.
     */
    pub fn add_witness(mut self, witness: Witness) -> Self {
        self.scripts.push(witness);

        self.scripts = self.scripts.sort(
            (w1, w2) => parseInt(w1.scriptHash, 16) - parseInt(w2.scriptHash, 16)
        );
        self
    }

    /**
     * Calculate the inputs required based on existing outputs provided. Also takes into account the fees required through the gas property.
     * @param balance Balance to retrieve inputs from.
     * @param strategy
     * @param fees Additional network fees. Invocation gas and tx fees are already included automatically so self is additional fees for priority on the network.
     */
    pub fn calculate(
        &mut self,
        balance: i64,
        strategy?: calculationStrategyFunction,
        fees: Fixed8,
    ) -> &self {
        const {
            inputs, change
        } = calculateInputs(
            balance,
            self.outputs,
            new
        Fixed8(self.fees).add(fees),
        strategy
        );

        self.inputs = inputs;
        self.outputs = self.outputs.concat(change);
        log.info(
        `Calculated
        the
        inputs
        required
        for Transaction with
        Balance: $ { balance.address }`
        );
        &self
    }

    /**
     * Serialize the Transaction and return it as a hexstring.
     * @param {boolean} signed  - Whether to serialize the signatures. Signing requires it to be serialized without the signatures.
     * @return {string} Hexstring.
     */
    pub fn serialize(&self, signed: bool) -> String {
        let mut out = String::from("");
        out.push_str(num2hexstring(&self.tx_type as i64).as_str());

        out.push_str(num2hexstring(&self.version as i64).as_str());
        out.push_str(self.serialize_exclusive().as_str());
        out.push_str(serializeArrayOf(&self.attributes).as_str());
        out.push_str(serializeArrayOf(&self.inputs).as_str());
        out.push_str(serializeArrayOf(&self.outputs).as_str());

        if signed {
            out += serializeArrayOf(&self.scripts);
        }
        return out;
    }

    /**
     * Signs a Transaction.
     * @param {Account|string} signer - Account, privateKey or WIF
     * @return {Transaction} self
     */
    pub fn sign(&self, signer: &str) -> self {
        if (typeof signer == = "string") {
            signer = new
            Account(signer);
        }
        const signature = sign(self.serialize(false), signer.privateKey);
        log.info(`Signed
        Transaction
        with
        Account: $ { signer.label }`);
        self.add_witness(Witness.fromSignature(signature, signer.pubKey));
        return self;
    }

    pub fn export(&self) -> BaseTransaction {
        BaseTransaction {
            tx_type: TransactionType::ContractTransaction,
            version: self.version,
            attributes: self.attributes.map((a) => a.export()),
            inputs: self.inputs.map((a) => a.export()),
            outputs: self.outputs.map((a) => a.export()),
            scripts: self.scripts.map((a) => a.export()),
        }
    }
}