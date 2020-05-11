use core::{str, sync::atomic};
use rand;
use std::time::{SystemTime, SystemTimeError};

const NANO_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000;

extern crate regex;
use regex::Regex;

extern crate mac_address;
use mac_address::get_mac_address;

const NIL: &str = "00000000-0000-0000-0000-000000000000";

pub type Bytes = [u8];

pub enum Format {
    Variant,
    Layout,
    Version,
    TimeStamp,
    ClockSeq,
    Node,
}

pub struct Layout {
    time_low: u32,
    time_mid: u16,
    time_high_and_version: u16,
    clock_seq_high_and_reserved: u8,
    clock_seq_low: u8,
    node: [u8; 6],
}

impl Layout {
    pub fn as_fields(&self) -> (u32, u16, u16, u16, [u8; 6]) {
        (
            self.time_low,
            self.time_mid,
            self.time_high_and_version,
            ((self.clock_seq_high_and_reserved as u16) << 8) | self.clock_seq_low as u16,
            self.node,
        )
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:x}-{:x}-{:x}-{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
            self.time_low,
            self.time_mid,
            self.time_high_and_version,
            ((self.clock_seq_high_and_reserved as u16) << 8) | self.clock_seq_low as u16,
            self.node[0],
            self.node[1],
            self.node[2],
            self.node[3],
            self.node[4],
            self.node[5],
        )
    }

    pub fn version(&self) -> Result<Version, &str> {
        match (self.time_high_and_version >> 12) & 0xf {
            0x00 => Err(NIL),
            0x01 => Ok(Version::TIME),
            0x02 => Ok(Version::DCE),
            0x03 => Ok(Version::MD5),
            0x04 => Ok(Version::RAND),
            0x05 => Ok(Version::SHA1),
            _ => Err("unknown uuid version"),
        }
    }

    pub fn variant(&self) -> Result<Variant, &str> {
        match (self.clock_seq_high_and_reserved >> 4) & 0xf {
            0x00 => Ok(Variant::NCS),
            0x01 => Ok(Variant::RFC),
            0x02 => Ok(Variant::MS),
            0x03 => Ok(Variant::FUT),
            _ => Err("unknown uuid variant"),
        }
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
    NIL = 0,
    TIME,
    DCE,
    MD5,
    RAND,
    SHA1,
}

pub struct Node(Bytes);

#[derive(Clone, Copy)]
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::AcqRel)
    }
}

#[derive(Debug)]
pub struct Uuid {
    pub bytes: Bytes,
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
        assert_eq!(uuid.version().unwrap(), Version::TIME);
        assert_eq!(uuid.variant().unwrap(), Variant::RFC);
    }

    #[test]
    fn test_format() {
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
