use neo_core::convert::num2var_int;
use neo_core::crypto::hash160;
use neo_core::misc::reverse_hex;
use neo_core::no_std::io::Error;
use neo_core::stringstream::StringStream;
use neo_crypto::hex;
use neo_wallet::verify::verify;

use crate::txmodel::Transaction;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Witness {
    invocation_script: &'static str,
    verification_script: &'static str,
    _script_hash: Option<&'static str>,
}


impl Transaction for Witness {
    fn deserialize(&self, hex: &str) -> Result<Witness, Error> {
        let ss = StringStream.new(hex);
        self.fromStream(ss)
    }

    fn fromStream(&self, ss: &mut StringStream) -> Result<Witness, Error> {
        let invocation_script = ss.read_var_bytes().as_str();
        let verification_script = ss.read_var_bytes().as_str();

        Ok(Witness {
            invocation_script,
            verification_script,
            _script_hash: None,
        })
    }

    fn serialize(&self) -> String {
        let mut invo_length = num2var_int((self.invocation_script.len() / 2) as i64);
        let veri_length = num2var_int((self.verification_script.len() / 2) as i64);

        invo_length.push_str(&self.invocation_script);
        invo_length.push_str(veri_length.as_str());
        invo_length.push_str(&self.verification_script);

        String::from(invo_length)
    }

    fn equals(&self, other: &Witness) -> bool {
        self.invocation_script == other.invocation_script &&
            self.verification_script == other.verification_script
    }

    fn export(&self) -> Result<Witness, Error> where
        Self: Sized {
        Ok(Witness {
            invocation_script: self.invocation_script,
            verification_script: self.verification_script,
            _script_hash: None,
        })
    }
}

/**
 * A Witness is a section of VM code that is ran during the verification of the transaction.
 *
 * For example, the most common witness is the VM Script that pushes the ECDSA signature into the VM and calling CHECKSIG to prove the authority to spend the TransactionInputs in the transaction.
 */
impl Witness {
    pub fn from_signature(&self, sig: &str, publicKey: &str) -> Witness {
        let invocation_script = "40" + sig;
        let verification_script = getVerificationScriptFromPublicKey(publicKey);

        Witness { invocation_script, verification_script, _script_hash: None }
    }


    /**
     * Builds a multi-sig Witness object.
     * @param tx Hexstring to be signed.
     * @param sigs Unordered list of signatures.
     * @param acct_or_verification_script Account or verification script. Account needs to be the multi-sig account and not one of the public keys.
     */
    pub fn build_multi_sig(
        &self,
        tx: &str,
        sigs: &[Witness],
        acct_or_verification_script: &str,
    ) -> Witness {

        let verification_script = acct_or_verification_script;


        let public_keys =  getPublicKeysFromVerificationScript(verification_script);

        let orderedSigs = public_keys.len().fill("");

        sigs.forEach((element) => {
            if (typeof element == = "string") {
                let position = public_keys.findIndex((key) =>
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

                if (keys.len() != 1) {
                    throw
                    new
                    Error("Given witness contains more than 1 public key!");
                }

                let position = public_keys.indexOf(keys[0]);

                orderedSigs[position] = getSignaturesFromInvocationScript(
                    element.invocationScript
                )[0];
            } else {
                panic!("Unable to process given signature");
            }
        });

        let signingThreshold = getSigningThresholdFromVerificationScript(
            verification_script
        );

        let validSigs = orderedSigs.filter((s) => s != "");

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
            invocation_script: validSigs
                .slice(0, signingThreshold)
                .map((s) => "40" + s)
            .join(""),
            verification_script,
            _script_hash: None,
        }
    }


    pub fn get_script_hash(&mut self) -> String {
        if self._script_hash {
            return self._script_hash.unwrap().to_string();
        } else if self.verification_script {
            self.generate_script_hash();
            return self._script_hash.unwrap().to_string();
        } else {
            panic!("Unable to produce scriptHash from empty verification_script");
        }
    }

    pub fn set_script_hash(&mut self, value: &str) {
        if self.verification_script {
            panic!("Unable to set scriptHash when verification_script is not empty");
        }
        self._script_hash = Option::from(value);
    }

    fn generate_script_hash(&mut self) {
        let hash = hash160(&hex::decode(&self.verification_script).unwrap()).as_slice();
        let hx = hash.to_hex();
        self._script_hash = Option::from(reverse_hex(hx).as_str());
    }
}
