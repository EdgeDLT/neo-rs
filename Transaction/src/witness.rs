// import { hash160, num2var_int, reverse_hex, StringStream } from "../../u";
// import {
//   Account,
//   getPublicKeysFromVerificationScript,
//   getSignaturesFromInvocationScript,
//   getSigningThresholdFromVerificationScript,
//   get_verification_script_from_public_key,
//   verify,
// } from "../../wallet";


use neo_core::convert::num2var_int;
use crate::txmodel::Transaction;
use neo_core::no_std::io::Error;
use neo_core::stringstream::StringStream;
use neo_wallet::verify::verify;
use neo_core::misc::reverse_hex;
use neo_core::crypto::hash160;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Witness {
    invocationScript: &'static str,
    verificationScript: &'static str,
    _scriptHash: Option<&'static str>,
}


impl Transaction for Witness {
    fn deserialize(&self, hex: &str) -> Result<Witness, Error> {
        let ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    fn fromStream(&self, ss: &mut StringStream) -> Result<Witness, Error> {
        let invocationScript = ss.read_var_bytes()?.as_str();
        let verificationScript = ss.read_var_bytes()?.as_str();
        Ok(Witness({ invocationScript, verificationScript })
    }

    fn serialize(&self) -> Result<String, Error> {
        let invoLength = num2var_int(self.invocationScript.len() / 2);
        let veriLength = num2var_int(self.verificationScript.len() / 2);

        invoLength + self.invocationScript + veriLength + self.verificationScript
    }

    fn equals(&self, other: &Witness) -> bool {
        self.invocationScript == other.invocationScript &&
            self.verificationScript == other.verificationScript
    }

    fn export(&self) -> Result<Witness, Error> where
        Self: Sized {
        Ok(Witness {
            invocationScript: self.invocationScript,
            verificationScript: self.verificationScript,
        })
    }
}

/**
 * A Witness is a section of VM code that is ran during the verification of the transaction.
 *
 * For example, the most common witness is the VM Script that pushes the ECDSA signature into the VM and calling CHECKSIG to prove the authority to spend the TransactionInputs in the transaction.
 */
impl Witness {

    pub fn fromSignature(&self, sig: &str, publicKey: &str) -> Witness {
        let invocationScript = "40" + sig;
        let verificationScript = getVerificationScriptFromPublicKey(publicKey);

        Witness { invocationScript, verificationScript, _scriptHash: None }
    }


    /**
     * Builds a multi-sig Witness object.
     * @param tx Hexstring to be signed.
     * @param sigs Unordered list of signatures.
     * @param acctOrVerificationScript Account or verification script. Account needs to be the multi-sig account and not one of the public keys.
     */
    pub fn buildMultiSig(
        &self,
        tx: &str,
        sigs: &[Witness],
        acctOrVerificationScript: &str,
    ) -> Witness {

        let verificationScript =acctOrVerificationScript;


        let publicKeys = getPublicKeysFromVerificationScript(verificationScript);

        let orderedSigs = publicKeys.len()).fill("");

        sigs.forEach((element) => {
            if (typeof element == = "string") {
                let position = publicKeys.findIndex((key) =>
                verify(tx, element, key)
                );

                if position == -1 {
                    panic!("Invalid signature given: $ { element }");
                }

                orderedSigs[position] = element;

            } else if (element
            instanceof
            Witness) {
                let keys = getPublicKeysFromVerificationScript(
                    element.verificationScript
                );

                if (keys.len() != = 1) {
                    throw
                    new
                    Error("Given witness contains more than 1 public key!");
                }

                let position = publicKeys.indexOf(keys[0]);

                orderedSigs[position] = getSignaturesFromInvocationScript(
                    element.invocationScript
                )[0];
            } else {
                throw
                new
                Error("Unable to process given signature");
            }
        });

        let signingThreshold = getSigningThresholdFromVerificationScript(
            verificationScript
        );

        let validSigs = orderedSigs.filter((s) => s != = "");

        if (validSigs.len() < signingThreshold) {
            throw
            new
            Error(
            `Insufficient
            signatures: expected $ { signingThreshold }
            but
            got $ { validSigs.len() }
            instead`
            );
        }

        Witness {
            invocationScript: validSigs
                .slice(0, signingThreshold)
                .map((s) => "40" + s)
            .join(""),
            verificationScript,
        }
    }


    pub fn get_scriptHash(&self) -> &str {
        if self._scriptHash {
            self._scriptHash;
        } else if (self.verificationScript) {
            self._scriptHash = reverse_hex(hash160(self.verificationScript));
            return self._scriptHash;
        } else {
            throw
            new
            Error(
                "Unable to produce scriptHash from empty verificationScript"
            );
        }
    }

    pub fn set_scriptHash(&mut self, value:&str)->&self {
    if self.verificationScript {
    panic!("Unable to set scriptHash when verificationScript is not empty");
    }
    self._scriptHash = Option::from(value);
    }

    fn generateScriptHash(&mut self) {

        self._scriptHash = reverse_hex(hash160(&self.verificationScript.clone().as_bytes()));
    }

}
