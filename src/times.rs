use mac_address;

use core::fmt;
use core::sync::atomic;
use std::time::{self, SystemTime};

use crate::Layout;
use crate::Uuid;
use crate::Variant;
use crate::Version;

pub const UTC_EPOCH: u64 = 0x1B21_DD21_3814_000;

pub enum Domain {
    PERSON = 0,
    GROUP,
    ORG,
}

impl Uuid {
    pub fn v1() -> Layout {
        let utc = Timestamp::new();
        let clock_seq = ClockSeq::new(rand::random::<u16>());

        Layout {
            time_low: ((utc & 0xffff_ffff) as u32),
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: ((clock_seq >> 8) & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: (clock_seq & 0xff) as u8,
            node: mac_address::get_mac_address().unwrap().unwrap().bytes(),
        }
    }

    pub fn v2(domain: Domain) -> Layout {
        let utc = Timestamp::new();
        let clock_seq = ClockSeq::new(rand::random::<u16>());
        let local_id = ((utc & 0xffff_ffff) as u32) | 1000;

        Layout {
            time_low: local_id,
            time_mid: ((utc >> 32 & 0xffff) as u16),
            time_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::DCE as u16) << 12,
            clock_seq_high_and_reserved: ((clock_seq >> 8) & 0xf) as u8 | (Variant::RFC as u8) << 4,
            clock_seq_low: domain as u8,
            node: mac_address::get_mac_address().unwrap().unwrap().bytes(),
        }
    }
}

#[derive(Debug)]
pub struct Timestamp(pub u64);

impl Timestamp {
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

#[derive(Clone, Copy)]
pub struct ClockSeq(u16);

impl ClockSeq {
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::SeqCst)
    }
}