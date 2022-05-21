const PRECOMPUTED_CHUNK_SIZE: usize = u16::MAX as usize;
const BIT_MASK: u64 = 0xFFFF;

pub struct BitReverser {
    precomputed_chunks: [u16; PRECOMPUTED_CHUNK_SIZE],
}

impl BitReverser {
    pub fn new() -> BitReverser {
        let mut precomputed_chunks = [0; PRECOMPUTED_CHUNK_SIZE];

        for i in 0..PRECOMPUTED_CHUNK_SIZE {
            let mut current = i;
            let mut reversed = 0;

            for _ in 0..16 {
                reversed <<= 1;
                reversed |= current & 0x1;
                current >>= 1;
            }

            precomputed_chunks[i] = reversed as u16;
        }

        BitReverser { precomputed_chunks }
    }

    fn reverse_chunk(&self, n: u64) -> u64 {
        self.precomputed_chunks[n as usize] as u64
    }

    pub fn reverse_bits(&self, n: u64) -> u64 {
        self.reverse_chunk(n >> 48 & BIT_MASK)
            | self.reverse_chunk(n >> 32 & BIT_MASK) << 16
            | self.reverse_chunk(n >> 16 & BIT_MASK) << 32
            | self.reverse_chunk(n & BIT_MASK) << 48
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 2u64.pow(63))]
    #[case(2u64.pow(31), 2u64.pow(32))]
    fn reverse_bits(#[case] input: u64, #[case] expected: u64) {
        let br = BitReverser::new();
        assert_eq!(expected, br.reverse_bits(input));
    }
}
