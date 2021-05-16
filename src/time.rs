#![cfg(feature = "mac")]

use mac_address;

use crate::{Layout, Node, TimeStamp, Variant, Version, UUID};

impl Layout {
    /// Get timestamp where the UUID generated in.
    pub fn get_timestamp(&self) -> u64 {
        self.field_low as u64
    }

    /// Get the MAC-address where the UUID generated with.
    pub fn get_mac_addr(&self) -> Node {
        self.node
    }

    fn time_fields(utc: u64, clock_seq: (u8, u8), node: Node) -> Self {
        Self {
            field_low: (utc & 0xffff_ffff) as u32,
            field_mid: ((utc >> 32 & 0xffff) as u16),
            field_high_and_version: (utc >> 48 & 0xfff) as u16 | (Version::TIME as u16) << 12,
            clock_seq_high_and_reserved: clock_seq.0,
            clock_seq_low: clock_seq.1,
            node: node,
        }
    }
}

impl UUID {
    /// New UUID version-1
    pub fn new_from_sys_time() -> Layout {
        let clock_seq: (u8, u8) = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        let utc = TimeStamp::new();
        Layout::time_fields(utc, clock_seq, device_mac_addr())
    }

    /// New UUID with a user defined MAC-address.
    pub fn from_node(node: Node) -> Layout {
        let utc = TimeStamp::new();
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout::time_fields(utc, clock_seq, node)
    }

    /// New UUID with specific timestamp.
    pub fn from_utc(utc: u64) -> Layout {
        let clock_seq = crate::clock_seq_high_and_reserved(Variant::RFC as u8);
        Layout::time_fields(utc, clock_seq, device_mac_addr())
    }
}

fn device_mac_addr() -> Node {
    Node(mac_address::get_mac_address().unwrap().unwrap().bytes())
}

/// `UUID` version-1
#[macro_export]
macro_rules! v1 {
    () => {
        format!("{:x}", $crate::UUID::new_from_sys_time().as_bytes())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_from_timestamp() {
        let uuid = UUID::new_from_sys_time();
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC));
    }

    #[test]
    fn new_uuid_from_user_defined_mac_address() {
        let uuid = UUID::from_node(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]));
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_mac_addr().0, [0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]);
    }

    #[test]
    fn new_uuid_from_custom_time() {
        let uuid = UUID::from_utc(0x1234_u64);
        assert_eq!(uuid.get_version(), Some(Version::TIME));
        assert_eq!(uuid.get_timestamp(), 0x1234_u64);
    }
}
