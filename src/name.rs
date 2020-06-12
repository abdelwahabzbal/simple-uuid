#[cfg(feature = "md_5")]
use md5;
#[cfg(feature = "sha_1")]
use sha1::Sha1;

use crate::{Layout, Variant, Version, UUID};

impl UUID {
    /// Generate a UUID by hashing a namespace identifier and name uses MD5.
    #[cfg(feature = "md_5")]
    pub fn v3(any: &str, namespace: UUID) -> Layout {
        let hash = md5::compute(Self::data(any, namespace)).0;
        Layout {
            time_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            time_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            time_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::MD5 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: [hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]],
        }
    }

    /// Generate a UUID by hashing a namespace identifier and name uses SHA1.
    #[cfg(feature = "sha_1")]
    pub fn v5(any: &str, namespace: UUID) -> Layout {
        let hash = Sha1::from(Self::data(any, namespace)).digest().bytes();
        Layout {
            time_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            time_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            time_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::SHA1 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: [hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]],
        }
    }

    fn data(any: &str, namespace: UUID) -> String {
        format!("{:x}", namespace) + any
    }
}

/// Creates a lower version-3 `UUID` String.
#[cfg(feature = "md_5")]
#[macro_export]
macro_rules! uuid_v3 {
    ($any:expr, $namespace:expr) => {
        format!("{:x}", $crate::UUID::v3($any, $namespace).as_bytes())
    };
}

/// Creates a lower version-5 `UUID` String.
#[cfg(feature = "sha_1")]
#[macro_export]
macro_rules! uuid_v5 {
    ($any:expr, $namespace:expr) => {
        format!("{:x}", $crate::UUID::v5($any, $namespace).as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v3() {
        let namespace = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in namespace.iter() {
            assert_eq!(UUID::v3("any", *s).get_version(), Some(Version::MD5));
            assert_eq!(UUID::v3("any", *s).get_variant(), Some(Variant::RFC));
        }

        for s in namespace.iter() {
            assert_eq!(UUID::v5("any", *s).get_version(), Some(Version::SHA1));
            assert_eq!(UUID::v5("any", *s).get_variant(), Some(Variant::RFC));
        }
    }

    #[test]
    fn test_v5() {
        let namespace = [
            UUID::NAMESPACE_DNS,
            UUID::NAMESPACE_OID,
            UUID::NAMESPACE_URL,
            UUID::NAMESPACE_X500,
        ];

        for s in namespace.iter() {
            assert_eq!(UUID::v5("any", *s).get_version(), Some(Version::SHA1));
            assert_eq!(UUID::v5("any", *s).get_variant(), Some(Variant::RFC));
        }
    }
}
