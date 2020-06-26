#![cfg(feature = "mac")]

use mac_address as MAC;
use rand;

use crate::{ClockSeq, Domain, Layout, Timestamp, Variant, Version, UUID};

impl UUID {
    /// Generate a time-based and MAC-address version UUID.
    pub fn new_v1() -> Layout {
        let utc = Timestamp::new();
        let clock_seq = Self::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout {
            field_low: ((utc & 0xffff_ffff) as u32),
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: Self::MAC_address(),
        }
    }

    /// Generate a time-based, MAC-address and DCE-security version UUID.
    ///
    /// NOTE: RFC-4122 reserves version-2 for `DCE-security` UUIDs;
    /// but it does not provide any details.
    pub fn new_v2(d: Domain) -> Layout {
        let utc = Timestamp::new();
        Layout {
            field_low: (utc & 0xffff_ffff) as u32,
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::DCE as u16) << 12,
            clock_seq_high_and_reserved: Self::clock_seq_high_and_reserved(Variant::RFC as u8).0,
            clock_seq_low: d as u8,
            node: Self::MAC_address(),
        }
    }

    fn clock_seq_high_and_reserved(s: u8) -> (u8, u8) {
        let clock_seq = ClockSeq::new(rand::random::<u16>());
        (
            ((clock_seq >> 8) & 0xf) as u8 | s << 4,
            (clock_seq & 0xff) as u8,
        )
    }

    #[allow(non_snake_case)]
    fn MAC_address() -> [u8; 6] {
        MAC::get_mac_address().unwrap().unwrap().bytes()
    }
}

/// Creates a lower string for `UUID` version-1.
#[macro_export]
macro_rules! uuid_v1 {
    () => {
        format!("{}", $crate::UUID::new_v1().as_bytes())
    };
}

/// Creates a lower string for `UUID` version-2.
#[macro_export]
macro_rules! uuid_v2 {
    ($domain:expr) => {
        format!("{}", $crate::UUID::new_v2($domain).as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_v1() {
        let uuid = UUID::new_v1();

        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));

        let mac = MAC::get_mac_address().unwrap().unwrap().bytes();
        assert_eq!(
            uuid.as_fields().4,
            (mac[0] as u64) << 40
                | (mac[1] as u64) << 32
                | (mac[2] as u64) << 24
                | (mac[3] as u64) << 16
                | (mac[4] as u64) << 8
                | (mac[5] as u64),
        );
    }

    #[test]
    fn test_new_v2() {
        let domain = [Domain::PERSON, Domain::GROUP, Domain::ORG];

        for d in domain.iter() {
            assert_eq!(UUID::new_v2(*d).get_version(), Some(Version::DCE));
            assert_eq!(UUID::new_v2(*d).get_variant(), Some(Variant::RFC));
        }
    }
}
