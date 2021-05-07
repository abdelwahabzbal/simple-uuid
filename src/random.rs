#![cfg(feature = "random")]

use crate::{Layout, Node, Random, Variant, Version};
use rand as random;

impl Random {
    /// New UUID from truly random number.
    pub fn new() -> Layout {
        let rand = random::random::<u128>().to_le_bytes();
        Layout {
            field_low: ((rand[0] as u32) << 24)
                | (rand[1] as u32) << 16
                | (rand[2] as u32) << 8
                | rand[3] as u32,
            field_mid: (rand[4] as u16) << 8 | (rand[5] as u16),
            field_high_and_version: ((rand[6] as u16) << 8 | (rand[7] as u16)) & 0xfff
                | (Version::RAND as u16) << 12,
            clock_seq_high_and_reserved: (rand[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: rand[9] as u8,
            node: Node([rand[10], rand[11], rand[12], rand[13], rand[14], rand[15]]),
        }
    }
}

/// `UUID` version-4
#[macro_export]
macro_rules! v4 {
    () => {
        format!("{:x}", $crate::Random::new().as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_v4() {
        let uuid = Random::new();
        assert_eq!(uuid.get_version(), Some(Version::RAND));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
    }
}
