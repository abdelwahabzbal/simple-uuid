//! This crate defines a Uniform Resource Name namespace for UUIDs
//! (Universally Unique IDentifier), also known as GUIDs (Globally
//! Unique IDentifier). A UUID is 128 bits long, and can guarantee
//! uniqueness across space and time.

#![doc(html_root_url = "https://docs.rs/uuid-rs")]

use regex::Regex;

use core::fmt;
use core::str;

#[cfg(feature = "mac")]
pub mod time;

#[cfg(feature = "hash")]
pub mod name;

#[cfg(feature = "rand")]
pub mod random;

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

/// Version represents the type of UUID.
/// The version number is in the most significant 4 bits of the Timestamp.
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

    ///
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
