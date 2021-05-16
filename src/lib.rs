//! This crate defines a uniform resource name namespace for UUIDs
//! (Universally Unique IDentifier), also known as GUIDs (Globally
//! Unique Identifier). A UUID is 128 bits long, and can guarantee
//! uniqueness across space and time.
//!
//! ```toml
//! [dependencies]
//! simple-uuid = { version = "*", features = ["random"] }
//! ```
//!
//! ```rust
//! use simple_uuid::v4;
//! println!("{}", v4!());
//! ```
#![doc(html_root_url = "https://docs.rs/simple-uuid")]

mod name;
mod random;
mod time;

use core::fmt;
use core::sync::atomic;
use std::time::SystemTime;

/// Is 100-ns ticks between UNIX and UTC epochs.
pub const UTC_EPOCH: u64 = 0x1b21_dd21_3814_000;

/// The UUID format is 16 octets.
#[derive(Debug, Default)]
pub struct Layout {
    /// The low field of the Timestamp.
    pub field_low: u32,
    /// The mid field of the Timestamp.
    pub field_mid: u16,
    /// The high field of the Timestamp multiplexed with the version number.
    pub field_high_and_version: u16,
    /// The high field of the ClockSeq multiplexed with the variant.
    pub clock_seq_high_and_reserved: u8,
    /// The low field of the ClockSeq.
    pub clock_seq_low: u8,
    /// IEEE 802 MAC-address.
    pub node: Node,
}

impl Layout {
    /// Returns the five field values of the UUID in big-endian order.
    pub fn be_fields(&self) -> (u32, u16, u16, u16, Node) {
        (
            self.field_low.to_be(),
            self.field_mid.to_be(),
            self.field_high_and_version.to_be(),
            ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16).to_be(),
            self.node,
        )
    }

    /// Returns the five field values of the UUID in little-endian order.
    pub fn as_fields(&self) -> (u32, u16, u16, u16, Node) {
        (
            self.field_low.to_le(),
            self.field_mid.to_le(),
            self.field_high_and_version.to_le(),
            ((self.clock_seq_high_and_reserved as u16) << 8 | self.clock_seq_low as u16).to_le(),
            self.node,
        )
    }

    /// Return the memory representation of the UUID in big-endian order .
    pub fn be_bytes(&self) -> UUID {
        UUID([
            self.field_low.to_be_bytes()[0],
            self.field_low.to_be_bytes()[1],
            self.field_low.to_be_bytes()[2],
            self.field_low.to_be_bytes()[3],
            self.field_mid.to_be_bytes()[0],
            self.field_mid.to_be_bytes()[1],
            self.field_high_and_version.to_be_bytes()[0],
            self.field_high_and_version.to_be_bytes()[1],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            self.node.0[0],
            self.node.0[1],
            self.node.0[2],
            self.node.0[3],
            self.node.0[4],
            self.node.0[5],
        ])
    }

    /// Return the memory representation of the UUID in little-endian order .
    pub fn as_bytes(&self) -> UUID {
        UUID([
            self.field_low.to_be_bytes()[3],
            self.field_low.to_be_bytes()[2],
            self.field_low.to_be_bytes()[1],
            self.field_low.to_be_bytes()[0],
            self.field_mid.to_be_bytes()[1],
            self.field_mid.to_be_bytes()[0],
            self.field_high_and_version.to_be_bytes()[1],
            self.field_high_and_version.to_be_bytes()[0],
            self.clock_seq_high_and_reserved,
            self.clock_seq_low,
            self.node.0[5],
            self.node.0[4],
            self.node.0[3],
            self.node.0[2],
            self.node.0[1],
            self.node.0[0],
        ])
    }

    /// Version of the current generated UUID.
    pub fn get_version(&self) -> Option<Version> {
        match (self.field_high_and_version >> 12) & 0xf {
            0x01 => Some(Version::TIME),
            0x02 => Some(Version::DCE),
            0x03 => Some(Version::MD5),
            0x04 => Some(Version::RAND),
            0x05 => Some(Version::SHA1),
            _ => None,
        }
    }

    /// Variant field of the current generated UUID.
    pub fn get_variant(&self) -> Option<Variant> {
        match (self.clock_seq_high_and_reserved >> 4) & 0xf {
            0x00 => Some(Variant::NCS),
            0x01 => Some(Variant::RFC),
            0x02 => Some(Variant::MS),
            0x03 => Some(Variant::FUT),
            _ => None,
        }
    }
}

/// Variant is a type field determines the layout of the UUID.
#[derive(Debug, Eq, PartialEq)]
pub enum Variant {
    /// Reserved, NCS backward compatibility.
    NCS = 0,
    /// The variant specified in `rfc4122` document.
    RFC,
    /// Reserved, Microsoft Corporation backward compatibility.
    MS,
    /// Reserved for future definition.
    FUT,
}

/// Version represents the type of UUID, and is in the most significant 4 bits of the Timestamp.
#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    /// The time-based version specified in `rfc4122` document.
    TIME = 1,
    /// DCE Security version, with embedded POSIX UIDs.
    DCE,
    /// The name-based version specified in `rfc4122` document that uses MD5 hashing.
    MD5,
    /// The randomly or pseudo-randomly generated version specified in `rfc4122` document.
    RAND,
    /// The name-based version specified in `rfc4122`document that uses SHA-1 hashing.
    SHA1,
}

/// Represented by Coordinated Universal Time (UTC) as a count
/// of 100-ns intervals from the system-time.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct TimeStamp(u64);

impl TimeStamp {
    /// Generate new UTC timestamp.
    pub fn new() -> u64 {
        let utc = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .checked_add(std::time::Duration::from_nanos(UTC_EPOCH))
            .unwrap()
            .as_nanos();
        (utc & 0xffff_ffff_ffff_fff) as u64
    }
}

/// Is a 128-bit number used to identify information in computer systems.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct UUID([u8; 16]);

impl UUID {
    /// UUID namespace for domain name system (DNS).
    pub const NAMESPACE_DNS: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for ISO object identifiers (OIDs).
    pub const NAMESPACE_OID: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for uniform resource locators (URLs).
    pub const NAMESPACE_URL: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);

    /// UUID namespace for X.500 distinguished names (DNs).
    pub const NAMESPACE_X500: UUID = UUID([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
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

impl ToString for UUID {
    fn to_string(&self) -> String {
        format!(
            "{:02}{:02}{:02}{:02}-{:02}{:02}-{:02}{:02}-{:02}{:02}-{:02}{:02}{:02}{:02}{:02}{:02}",
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

/// Used to avoid duplicates that could arise when the clock is set backwards in time.
pub struct ClockSeq(u16);

impl ClockSeq {
    /// New atomic random value.
    pub fn new(r: u16) -> u16 {
        atomic::AtomicU16::new(r).fetch_add(1, atomic::Ordering::SeqCst)
    }
}

fn clock_seq_high_and_reserved(s: u8) -> (u8, u8) {
    let clock_seq = ClockSeq::new(rand::random::<u16>());
    (
        ((clock_seq >> 8) & 0xf) as u8 | s << 4,
        (clock_seq & 0xff) as u8,
    )
}
/// The clock sequence is used to help avoid duplicates that could arise
/// when the clock is set backwards in time or if the node ID changes.
#[derive(Debug, PartialEq, Default, Copy, Clone)]
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

impl ToString for Node {
    fn to_string(&self) -> String {
        format!(
            "{:02}-{:02}-{:02}-{:02}-{:02}-{:02}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

pub struct Random(u128);

pub enum Algo {
    MD5,
    SHA1,
}

pub struct Hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_value() {
        let node = Node::default();
        assert_eq!(node, Node([0; 6]));

        let uuid = UUID::default();
        assert_eq!(uuid, UUID([0; 16]));

        let time: TimeStamp = TimeStamp::default();
        assert_eq!(time.0.leading_zeros(), 64)
    }

    #[test]
    fn to_string() {
        let node = Node::default();
        assert_eq!(node.to_string(), "00-00-00-00-00-00");

        let uuid = UUID::default();
        assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
    }
}
