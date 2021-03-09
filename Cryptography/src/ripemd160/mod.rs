extern crate block_buffer;
extern crate opaque_debug;

#[cfg(feature = "std")]
extern crate std;

pub use digest::Digest;
use digest::{Input, BlockInput, FixedOutput, Reset};
use block_buffer::BlockBuffer;
use block_buffer::byteorder::{LE, ByteOrder};
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{U20, U64};

mod block;
use block::{process_msg_block, DIGEST_BUF_LEN, H0};

#[derive(Clone)]
pub struct Ripemd160 {
    h: [u32; DIGEST_BUF_LEN],
    len: u64,
    buffer: BlockBuffer<U64>,
}

impl Default for Ripemd160 {
    fn default() -> Self {
        Ripemd160 {
            h: H0,
            len: 0,
            buffer: Default::default(),
        }
    }
}

impl BlockInput for Ripemd160 {
    type BlockSize = U64;
}

impl Input for Ripemd160 {
    fn input<B: AsRef<[u8]>>(&mut self, input: B) {
        let input = input.as_ref();
        // Assumes that input.len() can be converted to u64 without overflow
        self.len += input.len() as u64;
        let h = &mut self.h;
        self.buffer.input(input, |b| process_msg_block(h, b));
    }
}

impl FixedOutput for Ripemd160 {
    type OutputSize = U20;

    fn fixed_result(mut self) -> GenericArray<u8, Self::OutputSize> {
        {
            let h = &mut self.h;
            let l = self.len << 3;
            self.buffer.len64_padding::<LE, _>(l, |b| process_msg_block(h, b));
        }

        let mut out = GenericArray::default();
        LE::write_u32_into(&self.h, &mut out[..]);
        out
    }
}

impl Reset for Ripemd160 {
    fn reset(&mut self) {
        self.buffer.reset();
        self.len = 0;
        self.h = H0;
    }
}

opaque_debug::implement!(Ripemd160);
digest::impl_write!(Ripemd160);
