
use sgx_isa::{Attributes, Miscselect};

/// Information about how the sealing key was derived. This
/// should be stored alongside the sealed data, so that the enclave
/// can rederive the same key later.
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    // Record attributes and miscselect so that we can verify that
    // we can derive the correct wrapping key, but the actual input
    // to the derivation is CPU enclave state + SW-specified masks.
    attributes: Attributes,
    miscselect: Miscselect,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
