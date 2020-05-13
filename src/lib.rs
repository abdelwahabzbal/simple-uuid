use core::{fmt, str, sync::atomic};
use std::time::SystemTime;

use mac_address::get_mac_address;
use rand;
use regex::Regex;

pub const NANO_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000;

#[derive(Debug)]
pub enum Format {
    Layout,
    Variant,
    Version,
    TimeStamp,
    ClockSeq,
    Node,
}

#[derive(Debug)]
pub struct Layout {
    time_low: u32,
    time_mid: u16,
    time_high_and_version: u16,
    clock_seq_high_and_reserved: u8,
    clock_seq_low: u8,
    node: Id,
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

    pub fn as_bytes(&self) -> Uuid {
        Uuid([
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

    pub fn version(&self) -> Option<Version> {
        match (self.time_high_and_version >> 12) & 0xf {
            0x01 => Some(Version::TIME),
            0x02 => Some(Version::DCE),
            0x03 => Some(Version::MD5),
            0x04 => Some(Version::RAND),
            0x05 => Some(Version::SHA1),
            _ => None,
        }
    }

    pub fn variant(&self) -> Option<Variant> {
        match (self.clock_seq_high_and_reserved >> 4) & 0xf {
            0x00 => Some(Variant::NCS),
            0x01 => Some(Variant::RFC),
            0x02 => Some(Variant::MS),
            0x03 => Some(Variant::FUT),
            _ => None,
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:x}-{:x}-{:x}-{:x}-{:x}",
            self.as_fields().0,
            self.as_fields().1,
            self.as_fields().2,
            self.as_fields().3,
            self.as_fields().4,
        )
    }
}

pub struct Timestamp {
    pub tick: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Variant {
    NCS = 0,
    RFC,
    MS,
    FUT,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    TIME = 1,
    DCE,
    MD5,
    RAND,
    SHA1,
}

pub type Id = [u8; 6];

pub struct Node(Id);

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

impl fmt::LowerHex for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = (self.0[0] as u128) << 40
            | (self.0[1] as u128) << 32
            | (self.0[2] as u128) << 24
            | (self.0[3] as u128) << 16
            | (self.0[4] as u128) << 8
            | (self.0[5] as u128);
        fmt::LowerHex::fmt(&n, fmt)
    }
}

impl fmt::UpperHex for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = (self.0[0] as u64) << 40
            | (self.0[1] as u64) << 32
            | (self.0[2] as u64) << 24
            | (self.0[3] as u64) << 16
            | (self.0[4] as u64) << 8
            | (self.0[5] as u64);
        fmt::UpperHex::fmt(&n, fmt)
    }
}

#[derive(Clone, Copy)]
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::AcqRel)
    }
}

pub type Bytes = [u8; 16];

#[derive(Debug)]
pub struct Uuid(Bytes);

impl Uuid {
    pub fn v1() -> Layout {
        let tick = SystemTime::now().elapsed().unwrap();
        let utc = ((tick.as_nanos() & 0xffff_ffff_ffff_ffff) as u64) + NANO_TICKS_BETWEEN_EPOCHS;
        let clock_seq = ClockSeq::new(rand::random::<u16>());

        Layout {
            time_low: ((utc & 0xffff_ffff) as u32),
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: ((clock_seq >> 8) & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: (clock_seq & 0xff) as u8,
            node: get_mac_address().unwrap().unwrap().bytes(),
        }
    }

    pub fn is_valid(s: &str) -> bool {
        let regex = Regex::new(
            r"^(?i)(urn:uuid:)?[0-9a-f]{8}\-[0-9a-f]{4}\-[0-9a-f]{4}\-[0-9a-f]{4}\-[0-9a-f]{12}$",
        );
        regex.unwrap().is_match(s)
    }
}

impl fmt::LowerHex for Uuid {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = (self.0[0] as u128) << 120
            | (self.0[1] as u128) << 112
            | (self.0[2] as u128) << 104
            | (self.0[3] as u128) << 96
            | (self.0[4] as u128) << 88
            | (self.0[5] as u128) << 80
            | (self.0[6] as u128) << 72
            | (self.0[7] as u128) << 64
            | (self.0[8] as u128) << 56
            | (self.0[9] as u128) << 48
            | (self.0[10] as u128) << 40
            | (self.0[11] as u128) << 32
            | (self.0[12] as u128) << 24
            | (self.0[13] as u128) << 16
            | (self.0[14] as u128) << 8
            | (self.0[15] as u128);
        fmt::LowerHex::fmt(&bytes, fmt)
    }
}

impl fmt::UpperHex for Uuid {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = (self.0[0] as u128) << 120
            | (self.0[1] as u128) << 112
            | (self.0[2] as u128) << 104
            | (self.0[3] as u128) << 96
            | (self.0[4] as u128) << 88
            | (self.0[5] as u128) << 80
            | (self.0[6] as u128) << 72
            | (self.0[7] as u128) << 64
            | (self.0[8] as u128) << 56
            | (self.0[9] as u128) << 48
            | (self.0[10] as u128) << 40
            | (self.0[11] as u128) << 32
            | (self.0[12] as u128) << 24
            | (self.0[13] as u128) << 16
            | (self.0[14] as u128) << 8
            | (self.0[15] as u128);
        fmt::UpperHex::fmt(&bytes, fmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let uuid = Uuid::v1();

        assert_eq!(uuid.version(), Some(Version::TIME));
        assert_eq!(uuid.variant(), Some(Variant::RFC));

        assert!(Uuid::is_valid(&format!("{}", uuid)));
    }

    #[test]
    fn test_node() {
        let node = Node([121, 42, 53, 13, 19, 34]);
        assert_eq!(format!("{}", node), "79-2a-35-0d-13-22");
        assert_eq!(format!("{:x}", node), "792a350d1322");
        assert_eq!(format!("{:X}", node), "792A350D1322")
    }

    #[test]
    fn test_valid_uuid() {
        let uuid = [
            "urn:uuid:7370554e-8a0c-11ea-bc55-0242ac130003",
            "URN:UUID:0C0BF838-9388-11EA-BB37-0242AC130002",
            "0c0bf838-9388-11ea-bb37-0242ac130002",
            "0C0BF838-9388-11EA-BB37-0242AC130002",
        ];

        for id in uuid.iter() {
            assert!(Uuid::is_valid(id))
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_uuid() {
        let uuid = [
            "urn:uuid:7370554e-8a0c-11ea-bc55-0242ac130003_invalid",
            "URN:UUID:0C0BF838-9388-11EA-BB37-0242AC130002_INVALID",
            "0c0bf838-9388-11ea-bb37-0242ac130002_invalid",
            "0C0BF838-9388-11EA-BB37-0242AC130002_INVALID",
        ];

        for id in uuid.iter() {
            assert!(Uuid::is_valid(id))
        }
    }
}
