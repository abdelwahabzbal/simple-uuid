#![cfg(feature = "mac")]

use mac_address as MAC;
use rand;

use crate::{ClockSeq, Domain, Layout, Timestamp, Variant, Version, UUID};

impl UUID {
    /// Generate a time-based and MAC-address UUID.
    pub fn v1() -> Layout {
        let utc = Timestamp::new();
        let clock_seq = Self::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout {
            time_low: ((utc & 0xffff_ffff) as u32),
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: Self::mac_address(),
        }
    }

    /// Generate a time-based, MAC address and DCE security version UUID.
    ///
    /// NOTE: RFC 4122 reserves version 2 for "DCE security" UUIDs;
    /// but it does not provide any details.
    ///
    /// REF: https://pubs.opengroup.org/onlinepubs/9696989899/chap5.htm#tagcjh_08_02_01_01
    pub fn v2(d: Domain) -> Layout {
        let utc = Timestamp::new();
        Layout {
            time_low: (utc & 0xffff_ffff) as u32,
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::DCE as u16) << 12,
            clock_seq_high_and_reserved: Self::clock_seq_high_and_reserved(Variant::RFC as u8).0,
            clock_seq_low: d as u8,
            node: Self::mac_address(),
        }
    }

    fn clock_seq_high_and_reserved(s: u8) -> (u8, u8) {
        let clock_seq = ClockSeq::new(rand::random::<u16>());
        (
            ((clock_seq >> 8) & 0xf) as u8 | s << 4,
            (clock_seq & 0xff) as u8,
        )
    }

    fn mac_address() -> [u8; 6] {
        MAC::get_mac_address().unwrap().unwrap().bytes()
    }
}

/// Creates a lower version-1 `UUID` String.
#[macro_export]
macro_rules! uuid_v1 {
    () => {
        format!("{:x}", $crate::UUID::v1().as_bytes())
    };
}

/// Creates a lower version-2 `UUID` String.
#[macro_export]
macro_rules! uuid_v2 {
    ($domain:expr) => {
        format!("{:x}", $crate::UUID::v2($domain).as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let uuid = UUID::v1();

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
    fn test_v2() {
        let domain = [Domain::PERSON, Domain::GROUP, Domain::ORG];

        for d in domain.iter() {
            assert_eq!(UUID::v2(*d).get_version(), Some(Version::DCE));
            assert_eq!(UUID::v2(*d).get_variant(), Some(Variant::RFC));
        }
    }
}
