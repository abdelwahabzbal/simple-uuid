#![cfg(feature = "mac")]

use mac_address;

use crate::{Layout, Node, TimeStamp, Variant, Version};

impl TimeStamp {
    pub fn new() -> Layout {
        let clock_seq: (u8, u8) = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        let utc = TimeStamp::as_nano();
        Layout {
            field_low: (utc & 0xffff_ffff) as u32,
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: Node(mac_address::get_mac_address().unwrap().unwrap().bytes()),
        }
    }

    pub fn from(utc: u64) -> Layout {
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout {
            field_low: ((utc & 0xffff_ffff) as u32),
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: Node(mac_address::get_mac_address().unwrap().unwrap().bytes()),
        }
    }
}

impl Node {
    pub fn from(node: Node) -> Layout {
        let utc = TimeStamp::as_nano();
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout {
            field_low: ((utc & 0xffff_ffff) as u32),
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: node,
        }
    }
}

/// Quick `UUID` version-1
#[macro_export]
macro_rules! v1 {
    () => {
        format!("{:x}", $crate::TimeStamp::new().as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_from_timestamp() {
        let uuid = TimeStamp::new();
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
    }

    #[test]
    fn new_uuid_from_user_defined_mac_address() {
        let uuid = Node::from(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]));
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_mac().0, [0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]);
    }

    #[test]
    fn new_uuid_from_custom_time() {
        let uuid = TimeStamp::from(0x1234_u64);
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_time(), 0x1234_u64);
    }
}
