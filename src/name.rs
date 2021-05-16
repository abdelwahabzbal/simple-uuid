#![cfg(any(feature = "hash_md5", feauture = "hash_sha1"))]

use std::convert::TryInto;

use md5;
use sha1::Sha1;

use crate::{Layout, Node, Variant, Version, UUID};

impl Layout {
    fn hash_fields(hash: [u8; 16], v: Version) -> Self {
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

impl UUID {
    /// New UUID version-3 using md5 algorithme
    #[doc(cfg(feature = "hash_md5"))]
    pub fn using_md5(data: &str, ns: UUID) -> Layout {
        let hash = md5::compute(Self::concat(data, ns)).0;
        Layout::hash_fields(hash, Version::MD5)
    }

    /// New UUID version-5 using sha1 algorithme
    #[doc(cfg(feature = "hash_sha1"))]
    pub fn using_sha1(data: &str, ns: UUID) -> Layout {
        let hash = Sha1::from(Self::concat(data, ns)).digest().bytes()[..16]
            .try_into()
            .unwrap();
        Layout::hash_fields(hash, Version::SHA1)
    }

    fn concat(data: &str, ns: UUID) -> String {
        format!("{:x}", ns) + data
    }
}

/// `UUID` version-3
#[doc(cfg(feature = "hash_md5"))]
#[macro_export]
macro_rules! v3 {
    ($data:expr, $ns:expr) => {
        format!("{:x}", $crate::UUID::using_md5($data, $ns).as_bytes())
    };
}

/// `UUID` version-5
#[doc(cfg(feature = "hash_sha1"))]
#[macro_export]
macro_rules! v5 {
    ($data:expr, $ns:expr) => {
        format!("{:x}", $crate::UUID::using_sha1($data, $ns).as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_using_md5() {
        let ns = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in ns.iter() {
            assert_eq!(
                UUID::using_md5("test_data", *s).get_version(),
                Some(Version::MD5)
            );
            assert_eq!(
                UUID::using_md5("test_data", *s).get_variant(),
                Some(Variant::RFC)
            );
        }
    }

    #[test]
    fn new_uuid_using_sha1() {
        let ns = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in ns.iter() {
            assert_eq!(
                UUID::using_sha1("test_data", *s).get_version(),
                Some(Version::SHA1)
            );
            assert_eq!(
                UUID::using_sha1("test_data", *s).get_variant(),
                Some(Variant::RFC)
            );
        }
    }
}
