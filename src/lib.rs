use core::{str, sync::atomic};
use rand;
use std::time::{SystemTime, SystemTimeError};

extern crate regex;
use regex::Regex;

extern crate mac_address;
use mac_address::get_mac_address;

pub const NANO_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000;

#[derive(Debug)]
pub enum Format {
    Variant,
    Layout,
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
    node: [u8; 6],
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

    pub fn to_string(&self) -> String {
        format!(
            "{:x}-{:x}-{:x}-{:x}-{:x}",
            Self::as_fields(self).0,
            Self::as_fields(self).1,
            Self::as_fields(self).2,
            Self::as_fields(self).3,
            Self::as_fields(self).4,
        )
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

    pub fn get_mac(&self) -> Node {
        Node(self.node)
    }
}

pub struct Timestamp {
    pub tick: u64,
}

impl Timestamp {
    pub fn new() -> Result<Self, SystemTimeError> {
        match SystemTime::now().elapsed() {
            Ok(time) => Ok(Self {
                tick: ((time.as_nanos() & 0xffff_ffff_ffff_ffff) as u64)
                    + NANO_TICKS_BETWEEN_EPOCHS,
            }),
            Err(e) => Err(e),
        }
    }
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

pub struct Node([u8; 6]);

impl Node {
    pub fn to_string(&self) -> String {
        format!(
            "{:x}-{:x}-{:x}-{:x}-{:x}-{:x}-",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[derive(Clone, Copy)]
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::AcqRel)
    }
}

#[derive(Debug)]
pub struct Uuid {
    pub bytes: [u8; 16],
}

impl Uuid {
    pub fn v1() -> Layout {
        let utc = Timestamp::new().unwrap();
        let clock_seq = ClockSeq::new(rand::random::<u16>());

        Layout {
            time_low: ((utc.tick & 0xffff_ffff) as u32),
            time_mid: ((utc.tick >> 32 & 0xffff) as u16),
            time_high_and_version: (utc.tick >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let uuid = Uuid::v1();
        assert_eq!(uuid.version(), Some(Version::TIME));
        assert_eq!(uuid.variant(), Some(Variant::RFC));
    }

    #[test]
    fn test_to_string() {
        let uuid = Uuid::v1();
        assert!(Uuid::is_valid(&uuid.to_string()))
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
