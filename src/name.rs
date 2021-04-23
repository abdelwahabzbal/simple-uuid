#![cfg(any(feature = "hash_md5", feauture = "hash_sha1"))]

use std::convert::TryInto;

use md5;
use sha1::Sha1;

use crate::{Hash, Layout, Node, Variant, Version, UUID};

impl Layout {
    fn from_hashed(hash: [u8; 16], v: Version) -> Self {
        Self {
            field_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            field_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            field_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (v as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: Node([hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]]),
        }
    }
}

impl Hash {
    /// Generate new UUID using MD5 algorithm.
    pub fn v3(any: &str, ns: UUID) -> Layout {
        let hash = md5::compute(Self::concat(any, ns)).0;
        Layout::from_hashed(hash, Version::MD5)
    }

    /// Generate new UUID using SHA1 algorithm.
    pub fn v5(any: &str, ns: UUID) -> Layout {
        let hash = Sha1::from(Self::concat(any, ns)).digest().bytes()[..16]
            .try_into()
            .unwrap();
        Layout::from_hashed(hash, Version::SHA1)
    }

    fn concat(any: &str, ns: UUID) -> String {
        format!("{:x}", ns) + any
    }
}

/// Quick `UUID` version-3
#[macro_export]
macro_rules! v3 {
    ($any:expr, $ns:expr) => {
        format!("{:x}", $crate::Hash::v3($any, $ns).as_bytes())
    };
}

/// Quick `UUID` version-5
#[macro_export]
macro_rules! v5 {
    ($any:expr, $ns:expr) => {
        format!("{:x}", $crate::Hash::v5($any, $ns).as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_v3() {
        let ns = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in ns.iter() {
            assert_eq!(Hash::v3("any", *s).get_version(), Some(Version::MD5));
            assert_eq!(Hash::v3("any", *s).get_variant(), Some(Variant::RFC));
        }
    }

    #[test]
    fn new_v5() {
        let ns = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in ns.iter() {
            assert_eq!(Hash::v5("any", *s).get_version(), Some(Version::SHA1));
            assert_eq!(Hash::v5("any", *s).get_variant(), Some(Variant::RFC));
        }
    }
}
