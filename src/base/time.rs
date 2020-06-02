//! Is version-1, version-2 UUIDs based on time and MAC-addresse.

use mac_address as mac;

use core::fmt;
use core::sync::atomic;
use std::time::{self, SystemTime};

use crate::*;

/// UTC_EPOCH is 100-ns ticks between UNIX and UTC epochs.
pub const UTC_EPOCH: u64 = 0x1B21_DD21_3814_000;

/// Domain is security-domain-relative name.
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl UUID {
    /// Generate a time-based and MAC address UUID.
    pub fn v1() -> Layout {
        let utc = Timestamp::new();
        let clock_seq = ClockSeq::new(rand::random::<u16>());

        Layout {
            time_low: ((utc & 0xffff_ffff) as u32),
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: ((clock_seq >> 8) & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: (clock_seq & 0xff) as u8,
            node: Self::get_mac_address(),
        }
    }

    /// Generate a time-based, MAC address and DCE security version UUID.
    ///
    /// NOTE: RFC 4122 reserves version 2 for "DCE security" UUIDs;
    /// but it does not provide any details.
    ///
    /// REF: https://pubs.opengroup.org/onlinepubs/9696989899/chap5.htm#tagcjh_08_02_01_01
    pub fn v2(domain: Domain) -> Layout {
        let utc = Timestamp::new();
        let clock_seq = ClockSeq::new(rand::random::<u16>());

        Layout {
            time_low: (utc & 0xffff_ffff) as u32,
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::DCE as u16) << 12,
            clock_seq_high_and_reserved: ((clock_seq >> 8) & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: domain as u8,
            node: Self::get_mac_address(),
        }
    }

    fn get_mac_address() -> [u8; 6] {
        mac::get_mac_address().unwrap().unwrap().bytes()
    }
}

impl Layout {
    /// Get the time where the UUID generated in.
    pub fn get_time(&self) -> Timestamp {
        let time = (self.time_high_and_version as u64 & 0xfff) << 48
            | (self.time_mid as u64) << 32
            | self.time_low as u64;
        Timestamp(time)
    }
}

/// Timestamp represented by Coordinated Universal Time (UTC)
/// as a count of 100-ns intervals from the system-time.
#[derive(Debug)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Generate new 60-bit value from the system-time.
    pub fn new() -> u64 {
        let nano = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .checked_add(std::time::Duration::from_nanos(UTC_EPOCH))
            .unwrap()
            .as_nanos();

        (nano & 0xffff_ffff_ffff_ffff) as u64
    }

    pub fn duration(&self) -> time::Duration {
        time::Duration::from_nanos(self.0)
    }
}

/// the clock sequence is used to help avoid duplicates that could arise
/// when the clock is set backwards in time or if the node ID changes.
pub struct Node(pub [u8; 6]);

impl fmt::LowerHex for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

impl fmt::UpperHex for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

/// ClockSeq is used to avoid duplicates that could arise when the clock
/// is set backwards in time.
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::SeqCst)
    }
}

#[macro_export]
macro_rules! uuid_v1 {
    () => {
        format!("{:x}", $crate::UUID::v1().as_bytes())
    };
}

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

        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
        assert!(UUID::is_valid(&format!("{:X}", uuid.as_bytes())));
    }

    #[test]
    fn test_v2() {
        let uuid = UUID::v2(Domain::PERSON);

        assert_eq!(uuid.get_version(), Some(Version::DCE));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));

        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
        assert!(UUID::is_valid(&format!("{:X}", uuid.as_bytes())));
    }

    #[test]
    fn test_node() {
        let node = Node([00, 42, 53, 13, 19, 128]);
        assert_eq!(format!("{:x}", node), "00-2a-35-0d-13-80");
        assert_eq!(format!("{:X}", node), "00-2A-35-0D-13-80")
    }

    #[test]
    fn test_from_macro() {
        assert!(UUID::is_valid(&uuid_v1!()));
        assert!(UUID::is_valid(&uuid_v2!(Domain::PERSON)));
    }
}
