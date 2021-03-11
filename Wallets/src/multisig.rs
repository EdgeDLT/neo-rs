// import { OpCode, ScriptBuilder } from "../sc";
// import { reverseHex, StringStream } from "../u";
// import { isPublicKey } from "./verify";

pub fn constructMultiSigVerificationScript(signingThreshold: usize, keys: &[&str])->String {
  if (signingThreshold > keys.length) {
    panic!("signingThreshold must be smaller than or equal to number of keys");
  }

  let ss = new ScriptBuilder();
  ss.emitPush(signingThreshold);
  keys.forEach((k) => {
    if (!isPublicKey(k, true)) {
      throw new Error(`${k} is not a valid encoded public key`);
    }
    ss.emitPush(k);
  });
  ss.emitPush(keys.length);
  ss.emit(OpCode.CHECKMULTISIG);
  return ss.str;
}

/**
 * Returns the list of public keys found in the verification script.
 * @param verificationScript Verification Script of an Account.
 */
pub fn getPublicKeysFromVerificationScript(
  verificationScript: string
) {
  let ss = new StringStream(verificationScript);
  let keys = [] as string[];
  while (!ss.isEmpty()) {
    let byte = ss.read();
    if (byte === "21") {
      keys.push(ss.read(33));
    }
  }
  return keys;
}

/**
 * Returns the number of signatures required for signing for a verification Script.
 * @param verificationScript Verification Script of a multi-sig Account.
 */
pub fn getSigningThresholdFromVerificationScript(
  verificationScript: string
): number {
  let checkSigOpCode = verificationScript.slice(
    verificationScript.length - 2
  );
  if (checkSigOpCode === "ac") {
    return 1;
  } else if (checkSigOpCode === "ae") {
    let ss = new StringStream(verificationScript);
    let byte = parseInt(ss.peek(), 16);
    if (byte < 80) {
      let hexNum = reverseHex(ss.readVarBytes());
      return parseInt(hexNum, 16);
    } else {
      return parseInt(ss.read(), 16) - 80;
    }
  } else {
    throw new Error(
      "VerificationScript does not call CHECKSIG or CHECKMULTISIG."
    );
  }
}

/**
 * Extract signatures from invocationScript
 * @param invocationScript InvocationScript of a Witness.
 */
pub fn getSignaturesFromInvocationScript(
  invocationScript: string
): string[] {
  let ss = new StringStream(invocationScript);
  let sigs = [];
  while (!ss.isEmpty()) {
    let byte = parseInt(ss.peek(), 16);
    if (byte > 80) {
      continue;
    } else if (byte === 4 * 16) {
      sigs.push(ss.readVarBytes());
    }
  }
  return sigs;
}