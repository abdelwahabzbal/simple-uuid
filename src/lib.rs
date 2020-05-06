#![no_std]
#![allow(unused)]

use core::{cmp::Ordering, str, sync::atomic, time};

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

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
    node_id: [u8; 6],
}

impl Layout {
    pub fn as_fields(&self) -> (u32, u16, u16, u16, [u8; 6]) {
        (
            self.time_low,
            self.time_mid,
            self.time_high_and_version,
            ((self.clock_seq_high_and_reserved as u16) << 8) | self.clock_seq_low as u16,
            self.node_id,
        )
    }

    pub fn version(&self) -> Result<Version, &str> {
        match (self.time_high_and_version >> 12) & 0xf {
            0x00 => Err("00000000-0000-0000-0000-000000000000"),
            0x01 => Ok(Version::TIME),
            0x02 => Ok(Version::DCE),
            0x03 => Ok(Version::MD5),
            0x04 => Ok(Version::RAND),
            0x05 => Ok(Version::SHA1),
            _ => Err("Unknown uuid version"),
        }
    }
}

pub struct Timestamp {
    pub utc: time::Duration,
    cs: ClockSeq,
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

impl Version {
    pub const fn scalar(self) -> u8 {
        self as u8
    }
}

pub struct Node([u8; 6]);

#[derive(Clone, Copy)]
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(self) -> u16 {
        atomic::AtomicU16::new(self.0).fetch_add(1, atomic::Ordering::AcqRel)
    }
}

#[derive(Debug)]
pub struct Uuid {
    pub bytes: [u8; 16],
}

impl Uuid {
    pub fn v1(d: time::Duration, n: [u8; 6]) -> Layout {
        let ts = Timestamp {
            utc: d,
            cs: ClockSeq((d.subsec_millis() >> 16) as u16),
        };
        // let mm = ts.utc.as_secs()
        Layout {
            time_low: ((ts.utc.as_nanos() & 0xffff_ffff) as u32),
            time_mid: ((ts.utc.as_nanos() >> 32 & 0xffff) as u16),
            time_high_and_version: ((ts.utc.as_nanos() as u64) >> 48 & 0x0fff) as u16
                | ((Version::TIME as u16) << 12),
            clock_seq_high_and_reserved: (((ts.cs.new() >> 8) & 0x00ff) | Variant::RFC as u16)
                as u8,
            clock_seq_low: (ts.cs.new() & 0x00ff) as u8,
            node_id: n,
        }
    }

    pub fn is_valid_len(s: &str) -> Result<bool, &str> {
        match s {
            l if l.len() == 36 => Ok(true),
            l if l.starts_with("urn:uuid:") & (l.len() == 45) => Ok(true),
            _ => Err("Invalid uuid lenght"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_based() {
        let uuid = Uuid::v1(
            std::time::Duration::from_secs(1588784260),
            [40, 14, 22, 04, 25, 37],
        );
        assert_eq!(uuid.version().unwrap(), Version::TIME)
    }

    #[test]
    fn test_is_valid_len() {
        let uuids = [
            "urn:uuid:cd46baae-8a20-11ea-bc55-0242ac130003",
            "urn:uuid:cd46baae-8a20-11ea-bc55-0242ac130003_invalid",
            "7370554e-8a0c-11ea-bc55-0242ac130003",
            "7370554e-8a0c-11ea-bc55-0242ac130003_invalid",
        ];
        for uuid in uuids.iter() {
            if uuid.ends_with("_invalid") {
                assert_eq!(Uuid::is_valid_len(uuid).is_err(), true)
            } else {
                assert_eq!(Uuid::is_valid_len(uuid).is_ok(), true)
            }
        }
    }
}
