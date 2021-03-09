use crate::txtype::{TransactionType, toTxType};
use crate::transaction_attribute::TransactionAttribute;
use crate::transaction_input::TransactionInput;
use crate::transaction_output::TransactionOutput;
use crate::witness::Witness;
use neo_core::misc::reverse_hex;
use neo_core::no_std::io::Error;
use neo_crypto::sha2;
use neo_core::fixed8::Fixed8;
use crate::txmodel::transaction;
use crate::usage::{toTxAttrUsage, TxAttrUsage};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub(crate) struct BaseTransaction {
    tx_type: TransactionType,
    version: u8,
    attributes: Vec<TransactionAttribute>,
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    scripts: Vec<Witness>,
}

impl transaction for BaseTransaction {
    fn hash(&self) -> Result<&str, Error> {
        reverse_hex(sha2::digest(self.serialize(false)))
    }


    fn addOutput(&mut self, txOut: &TransactionOutput) -> &self {
        self.outputs.push(txOut.clone());
        self
    }

    fn addIntent(&mut self, symbol: &str, value: Fixed8, address: &str) -> &self {
        self.outputs.push(TransactionOutput.fromIntent(symbol, value, address));
        self
    }

    fn addAttribute(&mut self, usage: usize, data: &str) -> &self {
        self.attributes.push(TransactionAttribute { usage: toTxAttrUsage(usage)?, data });
        self
    }

    fn addRemark(&mut self, remark: &str) -> &self {
        let hexRemark = str2hexstring(&remark);
        self.addAttribute(TxAttrUsage::Remark as usize, hexRemark);
        self
    }

    fn addWitness(&mut self, witness: &Witness) -> &self {
        self.scripts.push(witness.clone());

        self.scripts = self.scripts.sort(
            (w1, w2) => parseInt(w1.scriptHash, 16) - parseInt(w2.scriptHash, 16)
        );

        self
    }

    fn serialize(&self, signed: bool) -> Result<&str, Error> {
        unimplemented!()
    }

    fn sign(&self, signer: &str) -> &self {
        unimplemented!()
    }

    fn export(&self) -> dyn transaction {
        unimplemented!()
    }
}

impl BaseTransaction {
    pub const tx_type: TransactionType = toTxType(0x00)?;


    pub fn hash(&self) -> Result<&str, Error> {
        reverse_hex(sha2::digest(self.serialize(false)))
    }

    fn fees(&self) -> Fixed8 {};

    // fn exclusiveData(&self): { [key: string]: any };

    pub fn serializeExclusive(&self) -> Result<&str, Error> {};

    pub addOutput( & self , txOut: & TransactionOutput) -> & self {

    self .outputs.push(new TransactionOutput(txOut));
    return self ;
}

/**
 * Adds a TransactionOutput. TransactionOutput can be given as a TransactionOutput object or as human-friendly values. This is detected by the number of arguments provided.
 * @param symbol The symbol of the asset (eg NEO or GAS).
 * @param value The value to send.
 * @param address The address to send to.
 */
pub fn addIntent(
    &self,
    symbol: string,
    value: number | Fixed8,
address: string
): self {
self .outputs.push(TransactionOutput.fromIntent(symbol, value, address));
return self ;
}

/**
 * Add an attribute.
 * @param usage The usage type. Do refer to txAttrUsage enum values for all available options.
 * @param data The data as hexstring.
 */
pub fn addAttribute(&self, usage: number, data: string): self {
if (typeof data != = "string") {
throw new TypeError("data should be formatted as string!");
}

}

/**
 * Add a remark.
 * @param remark A remark in ASCII.
 */
pub fn addRemark(&self, remark: string): self {

}

/**
 * Adds an Witness to the Transaction and automatically sorts the witnesses according to scripthash.
 * @param witness The Witness object to add.
 */
pub fn addWitness(&self, witness: Witness): self {
if (witness.scriptHash == = "") {
throw new Error("Please define the scriptHash for self Witness!");
}
self.scripts.push(witness);
self.scripts = self.scripts.sort(
(w1, w2) => parseInt(w1.scriptHash, 16) - parseInt(w2.scriptHash, 16)
);
return self;
}

/**
 * Calculate the inputs required based on existing outputs provided. Also takes into account the fees required through the gas property.
 * @param balance Balance to retrieve inputs from.
 * @param strategy
 * @param fees Additional network fees. Invocation gas and tx fees are already included automatically so self is additional fees for priority on the network.
 */
pub fn calculate(
    &self,
    balance: Balance,
    strategy?: calculationStrategyFunction,
    fees: number | Fixed8 = 0
): self {
const { inputs, change } = calculateInputs(
balance,
self .outputs,
new Fixed8( self .fees).add(fees),
strategy
);
self .inputs = inputs;
self .outputs = self .outputs.concat(change);
log.info(
`Calculated the inputs required for Transaction with Balance: ${balance.address}`
);
return self ;
}

/**
 * Serialize the transaction and return it as a hexstring.
 * @param {boolean} signed  - Whether to serialize the signatures. Signing requires it to be serialized without the signatures.
 * @return {string} Hexstring.
 */
pub fn serialize(&self, signed: bool): string {
let out = "";
out += num2hexstring( self.type );
out += num2hexstring( self.version);
out += self.serializeExclusive();
out += serializeArrayOf( self.attributes);
out += serializeArrayOf( self.inputs);
out += serializeArrayOf( self.outputs);
if (signed) {
out += serializeArrayOf( self.scripts);
}
return out;
}

/**
 * Signs a transaction.
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
    self.addWitness(Witness.fromSignature(signature, signer.pubKey));
    return self;
}

pub fn export(&self): BaseTransaction {
return {
type: self.type,
version: self.version,
attributes: self.attributes.map((a) => a.export()),
inputs: self.inputs.map((a) => a.export()),
outputs: self.outputs.map((a) => a.export()),
scripts: self.scripts.map((a) => a.export()),
};
}
}