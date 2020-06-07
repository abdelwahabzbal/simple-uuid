//! This crate defines a uniform resource name namespace for UUIDs
//! (Universally Unique IDentifier), also known as GUIDs (Globally
//! Unique Identifier). A UUID is 128 bits long, and can guarantee
//! uniqueness across space and time.

#![doc(html_root_url = "https://docs.rs/uuid-rs")]

use mac_address as MAC;
use md5;
use rand;
use regex::Regex;
use sha1::Sha1;

use core::fmt;
use core::str;
use core::sync::atomic;
use std::time::{self, SystemTime};

/// UTC_EPOCH is 100-ns ticks between UNIX and UTC epochs.
pub const UTC_EPOCH: u64 = 0x1B21_DD21_3814_000;

/// The UUID format is 16 octets.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Layout {
    /// The low field of the Timestamp.
    pub time_low: u32,
    /// The mid field of the Timestamp.
    pub time_mid: u16,
    /// The high field of the Timestamp multiplexed with the version number.
    pub time_high_and_version: u16,
    /// The high field of the ClockSeq multiplexed with the variant.
    pub clock_seq_high_and_reserved: u8,
    /// The low field of the ClockSeq.
    pub clock_seq_low: u8,
    /// IEEE 802 MAC address.
    pub node: [u8; 6],
}

impl Layout {
    pub fn as_fields(&self) -> (u32, u16, u16, u16, u64) {
        (
            self.time_low,
            self.time_mid,
            self.time_high_and_version,
            ((self.clock_seq_high_and_reserved as u16) << 8) | self.clock_seq_low as u16,
            (self.node[0] as u64) << 40
                | (self.node[1] as u64) << 32
                | (self.node[2] as u64) << 24
                | (self.node[3] as u64) << 16
                | (self.node[4] as u64) << 8
                | (self.node[5] as u64),
        )
    }

    pub fn as_bytes(&self) -> UUID {
        UUID([
            self.time_low.to_be_bytes()[0],
            self.time_low.to_be_bytes()[1],
            self.time_low.to_be_bytes()[2],
            self.time_low.to_be_bytes()[3],
            self.time_mid.to_be_bytes()[0],
            self.time_mid.to_be_bytes()[1],
            self.time_high_and_version.to_be_bytes()[0],
            self.time_high_and_version.to_be_bytes()[1],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5],
        ])
    }

    /// Get the version of the current generated UUID.
    pub fn get_version(&self) -> Option<Version> {
        match (self.time_high_and_version >> 12) & 0xf {
            0x01 => Some(Version::TIME),
            0x02 => Some(Version::DCE),
            0x03 => Some(Version::MD5),
            0x04 => Some(Version::RAND),
            0x05 => Some(Version::SHA1),
            _ => None,
        }
    }

    /// Get the variant field of the current generated UUID.
    pub fn get_variant(&self) -> Option<Variant> {
        match (self.clock_seq_high_and_reserved >> 4) & 0xf {
            0x00 => Some(Variant::NCS),
            0x01 => Some(Variant::RFC),
            0x02 => Some(Variant::MS),
            0x03 => Some(Variant::FUT),
            _ => None,
        }
    }

    /// Get the time where the UUID generated in.
    pub fn get_time(&self) -> Timestamp {
        let time = (self.time_high_and_version as u64 & 0xfff) << 48
            | (self.time_mid as u64) << 32
            | self.time_low as u64;
        Timestamp(time)
    }
}

/// Domain is security-domain-relative name.
pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

/// Variant is a type field determines the layout of the UUID.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Variant {
    /// Reserved, NCS backward compatibility.
    NCS = 0,
    /// The variant specified in rfc4122 document.
    RFC,
    /// Reserved, Microsoft Corporation backward compatibility.
    MS,
    /// Reserved for future definition.
    FUT,
}

/// Version represents the type of UUID, and is in the most significant 4 bits of the Timestamp.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Version {
    /// The time-based version specified in this document.
    TIME = 1,
    /// DCE Security version, with embedded POSIX UIDs.
    DCE,
    /// The name-based version specified in rfc4122 document that uses MD5 hashing.
    MD5,
    /// The randomly or pseudo-randomly generated version specified in rfc4122 document.
    RAND,
    /// The name-based version specified in rfc4122 document that uses SHA-1 hashing.
    SHA1,
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

/// Is a 128-bit number used to identify information in computer systems.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct UUID([u8; 16]);

impl UUID {
    /// UUID namespace for Domain Name System (DNS).
    pub const NAMESPACE_DNS: Self = UUID([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for ISO Object Identifiers (OIDs).
    pub const NAMESPACE_OID: Self = UUID([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for Uniform Resource Locators (URLs).
    pub const NAMESPACE_URL: Self = UUID([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for X.500 Distinguished Names (DNs).
    pub const NAMESPACE_X500: Self = UUID([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// Generate a time-based and MAC-address UUID.
    pub fn v1() -> Layout {
        let utc = Timestamp::new();
        let clock_seq = Self::clock_seq_high_and_reserved(1);
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
            clock_seq_high_and_reserved: Self::clock_seq_high_and_reserved(1).0,
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

    /// Generate a UUID by hashing a namespace identifier and name uses MD5.
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

    /// Generate a UUID from truly random numbers.
    pub fn v4() -> Layout {
        let rng = rand::random::<u128>();
        let rand = rng.to_be_bytes();
        Layout {
            time_low: ((rand[0] as u32) << 24)
                | (rand[1] as u32) << 16
                | (rand[2] as u32) << 8
                | rand[3] as u32,
            time_mid: (rand[4] as u16) << 8 | (rand[5] as u16),
            time_high_and_version: ((rand[6] as u16) << 8 | (rand[7] as u16)) & 0xfff
                | (Version::RAND as u16) << 12,
            clock_seq_high_and_reserved: (rand[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: rand[9] as u8,
            node: [rand[10], rand[11], rand[12], rand[13], rand[14], rand[15]],
        }
    }

    /// Generate a UUID by hashing a namespace identifier and name uses SHA1.
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

    /// Apart from determining whether the timestamp portion of the UUID
    /// is in the future, there is no mechanism for determining whether a UUID is valid.
    pub fn is_valid(s: &str) -> bool {
        let regex = Regex::new(
            r"^(?i)(urn:uuid:)?[0-9a-f]{8}\-[0-9a-f]{4}\-[0-5]{1}[0-9a-f]{3}\-[0-9a-f]{4}\-[0-9a-f]{12}$",
        );
        regex.unwrap().is_match(s)
    }
}

impl fmt::LowerHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
        )
    }
}

impl fmt::UpperHex for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
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

#[macro_export]
macro_rules! uuid_v3 {
    ($any:expr, $namespace:expr) => {
        format!("{:x}", $crate::UUID::v3($any, $namespace).as_bytes())
    };
}

#[macro_export]
macro_rules! uuid_v4 {
    () => {
        format!("{:x}", $crate::UUID::v4().as_bytes())
    };
}

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
    fn test_v1() {
        let uuid = UUID::v1();

        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
    }

    #[test]
    fn test_v2() {
        let uuid = UUID::v2(Domain::PERSON);

        assert_eq!(uuid.get_version(), Some(Version::DCE));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
    }

    #[test]
    fn test_node() {
        let node = Node([00, 42, 53, 13, 19, 128]);
        assert_eq!(format!("{:x}", node), "00-2a-35-0d-13-80");
        assert_eq!(format!("{:X}", node), "00-2A-35-0D-13-80")
    }

    #[test]
    fn test_v3() {
        let uuid = UUID::v3("any", UUID::NAMESPACE_X500);

        assert_eq!(uuid.get_version(), Some(Version::MD5));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
    }

    #[test]
    fn test_v4() {
        let uuid = UUID::v4();

        assert_eq!(uuid.get_version(), Some(Version::RAND));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
    }

    #[test]
    fn test_v5() {
        let uuid = UUID::v5("any", UUID::NAMESPACE_X500);

        assert_eq!(uuid.get_version(), Some(Version::SHA1));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
        assert!(UUID::is_valid(&format!("{:x}", uuid.as_bytes())));
    }

    #[test]
    fn test_valid_uuid() {
        let uuid = [
            "urn:uuid:7370554e-8a0c-11ea-bc55-0242ac130003",
            "0c0bf838-9388-11ea-bb37-0242ac130002",
        ];

        for id in uuid.iter() {
            assert!(UUID::is_valid(id))
        }
    }

    #[test]
    fn test_valid_upper_uuid() {
        let uuid = [
            "urn:uuid:7370554e-8a0c-11ea-bc55-0242ac130003",
            "0c0bf838-9388-11ea-bb37-0242ac130002",
        ];

        for id in uuid.iter() {
            assert!(UUID::is_valid(&id.to_ascii_uppercase()))
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_uuid() {
        let uuid = [
            "urn:uuid:7370554e-8a0c-11ea-bc55-0242ac130003_invalid",
            "0c0bf838-9388-11ea-bb37-0242ac130002_invalid",
            "0c0bf838-9388-61ea-bb37-0242ac130002", // invalid version
        ];

        for id in uuid.iter() {
            assert!(UUID::is_valid(id))
        }
    }

    #[test]
    fn test_from_macro() {
        assert!(UUID::is_valid(&uuid_v1!()));
        assert!(UUID::is_valid(&uuid_v2!(Domain::PERSON)));
        assert!(UUID::is_valid(&uuid_v3!("any", UUID::NAMESPACE_DNS)));
        assert!(UUID::is_valid(&uuid_v4!()));
        assert!(UUID::is_valid(&uuid_v5!("any", UUID::NAMESPACE_OID)));
    }
}
