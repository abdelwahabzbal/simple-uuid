#![cfg(feature = "mac")]

use mac_address;

use crate::{Layout, Node, TimeStamp, Variant, Version, UUID};

impl Layout {
    pub fn new(utc: u64, clock_seq: (u8, u8)) -> Self {
        Self {
            field_low: (utc & 0xffff_ffff) as u32,
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: Node(mac_address::get_mac_address().unwrap().unwrap().bytes()),
        }
    }
}

impl TimeStamp {
    pub fn v1() -> Layout {
        let clock_seq: (u8, u8) = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        let utc = TimeStamp::as_nano_sec();
        Layout::new(utc, clock_seq)
    }
}

impl UUID {
    pub fn from_node(node: Node) -> Layout {
        let utc = TimeStamp::as_nano_sec();
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);

        let mut layout = Layout::new(utc, clock_seq);
        layout.node = node;
        layout
    }

    pub fn from_utc(utc: u64) -> Layout {
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout::new(utc, clock_seq)
    }
}

impl Layout {
    /// Get timestamp where the UUID generated in.
    pub fn get_time_stamp(&self) -> TimeStamp {
        TimeStamp(self.field_low as u64)
    }

    /// Get the MAC-address where the UUID generated with.
    pub fn get_mac_address(&self) -> Node {
        self.node
    }
}

/// Quick `UUID` version-1
#[macro_export]
macro_rules! v1 {
    () => {
        format!("{:x}", $crate::TimeStamp::v1().as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_from_timestamp() {
        let uuid = TimeStamp::v1();
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
    }

    #[test]
    fn new_uuid_from_user_defined_mac_address() {
        let uuid = UUID::from_node(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]));
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(
            uuid.get_mac_address().0,
            [0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]
        );
    }

    #[test]
    fn new_uuid_from_custom_time() {
        let uuid = UUID::from_utc(0x1234_u64);
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_time_stamp(), TimeStamp(0x1234_u64));
    }
}
