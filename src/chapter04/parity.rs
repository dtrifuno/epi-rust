const PRECOMPUTED_PARITY_SIZE: usize = 1 + (u16::MAX as usize);
const BIT_MASK: u64 = 0xFFFF;

pub struct ParityCalculator {
    precomputed_parity: [u8; PRECOMPUTED_PARITY_SIZE],
}

impl ParityCalculator {
    pub fn new() -> ParityCalculator {
        let mut precomputed_parity = [0; PRECOMPUTED_PARITY_SIZE];

        for i in 0..PRECOMPUTED_PARITY_SIZE {
            let mut j = i;
            let mut parity = 0;
            while j != 0 {
                parity = parity ^ 1;
                j = j & (j - 1);
            }
            precomputed_parity[i] = parity;
        }
        return ParityCalculator { precomputed_parity };
    }

    pub fn calculate_parity(&self, n: u64) -> u8 {
        self.precomputed_parity[(n & BIT_MASK) as usize]
            ^ self.precomputed_parity[(n >> 16 & BIT_MASK) as usize]
            ^ self.precomputed_parity[(n >> 32 & BIT_MASK) as usize]
            ^ self.precomputed_parity[(n >> 48 & BIT_MASK) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(0b_0100_1000_0010, 1)]
    #[case(0b_1111_1111_1111_1111, 0)]
    fn test_calculate_parity(#[case] input: u64, #[case] expected: u8) {
        let pc = ParityCalculator::new();
        assert_eq!(expected, pc.calculate_parity(input));
    }
}
