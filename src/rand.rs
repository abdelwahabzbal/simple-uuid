#![doc(cfg(feature = "rand_num"))]
#![cfg(feature = "rand_num")]

use crate::{Layout, Node, Variant, Version, UUID};
use rand_core::{OsRng, RngCore};

impl UUID {
    /// New UUID version-4 from truly-random number
    pub fn new_from_rand() -> Layout {
        let mut key = [0u8; 128];
        OsRng.fill_bytes(&mut key);

        let random_u64_round_1 = OsRng.next_u64();
        let round_1 = random_u64_round_1.to_le_bytes();

        let random_u64_round_2 = OsRng.next_u64();
        let round_2 = random_u64_round_2.to_le_bytes();

        Layout {
            field_low: ((round_1[0] as u32) << 24)
                | (round_1[1] as u32) << 16
                | (round_1[2] as u32) << 8
                | round_1[3] as u32,
            field_mid: (round_1[4] as u16) << 8 | (round_1[5] as u16),
            field_high_and_version: ((round_1[6] as u16) << 8 | (round_1[7] as u16)) & 0xfff
                | (Version::RAND as u16) << 12,
            clock_seq_high_and_reserved: (round_2[0] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: round_2[1] as u8,
            node: Node([
                round_2[2], round_2[3], round_2[4], round_2[5], round_2[6], round_2[7],
            ]),
        }
    }
}

/// `UUID` version-4
#[doc(cfg(feature = "rand_num"))]
#[macro_export]
macro_rules! v4 {
    () => {
        format!("{:x}", $crate::UUID::new_from_rand().as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_from_rand() {
        let uuid = UUID::new_from_rand();
        assert_eq!(uuid.get_version(), Some(Version::RAND));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
    }
}
