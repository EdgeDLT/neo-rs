use neo_core::convert::hex2int;
use neo_core::fixed8::Fixed8;
use neo_core::misc::reverse_hex;
use neo_core::stringstream::StringStream;

pub fn construct_multi_sig_verification_script(signingThreshold: usize, keys: &[str]) -> String {
    if signingThreshold > keys.len() {
        panic!("signingThreshold must be smaller than or equal to number of keys");
    }

    let ss = ScriptBuilder.new();
    ss.emitPush(signingThreshold);
    for k in keys {
        if !is_public_keylicKey(k, true) {
            panic!("{} is not a valid encoded public ke", k);
        }
        ss.emitPush(k);
    }
    ss.emitPush(keys.len());
    ss.emit(OpCode.CHECKMULTISIG);
    return ss.str;
}

/**
 * Returns the list of public keys found in the verification script.
 * @param verification_script Verification Script of an Account.
 */
pub fn getPublicKeysFromVerificationScript(
    verification_script: &str
) -> Vec<String> {
    let mut ss = StringStream::new(verification_script);
    let mut keys: Vec<String> = Vec::new();
    while !ss.isEmpty() {
        let byte = ss.read(1);
        if byte == "21" {
            keys.push(ss.read(33));
        }
    }

    keys
}

/**
 * Returns the number of signatures required for signing for a verification Script.
 * @param verificationScript Verification Script of a multi-sig Account.
 */
pub fn getSigningThresholdFromVerificationScript(
    verificationScript: &str
) -> Fixed8 {
    let check_sig_op_code = &verificationScript[
        verificationScript.len() - 2..verificationScript.len()];
    match check_sig_op_code {
        "ac" => Fixed8(1),
        "ae" => {
            let mut ss = StringStream::new(verificationScript);
            let byte = hex2int(&ss.peek(1).as_str()).unwrap();

            return if byte < 80 {
                let hex_num = reverse_hex(ss.readVarBytes());
                Fixed8(hex2int(hex_num.as_str()).unwrap())
            } else {
                Fixed8(ss.read()) - 80
            };
        }
        _ => panic!("VerificationScript does not call CHECKSIG or CHECKMULTISIG.");
    }
}

/**
 * Extract signatures from invocationScript
 * @param invocationScript InvocationScript of a Witness.
 */
pub fn getSignaturesFromInvocationScript(
    invocationScript: &str
) -> Vec<String> {
    let ss = StringStream::new(invocationScript);
    let mut sigs = Vec::new();
    while !ss.isEmpty() {
        let byte = hex2int(ss.peek(1).as_str()).unwrap();
        if byte > 80 {
            continue;
        } else if byte == 4 * 16 {
            sigs.push(ss.readVarBytes());
        }
    }
    return sigs;
}

#[derive(Debug, Fail)]
pub enum MultiSigError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "unsupported format")]
    UnsupportedFormat,
}
