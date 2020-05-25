use md5;
use sha1::Sha1;

use crate::Layout;
use crate::Uuid;
use crate::Variant;
use crate::Version;

impl Uuid {
    pub fn v3(any: &str, ns: Uuid) -> Layout {
        let data = format!("{:x}", ns) + any;
        let hash = md5::compute(&data).0;

        Layout {
            time_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            time_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            time_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::MD5 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: [hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]],
        }
    }

    pub fn v5(any: &str, nspace: Uuid) -> Layout {
        let data = format!("{:x}", nspace) + any;
        let hash = Sha1::from(&data).digest().bytes();

        Layout {
            time_low: ((hash[0] as u32) << 24)
                | (hash[1] as u32) << 16
                | (hash[2] as u32) << 8
                | hash[3] as u32,
            time_mid: (hash[4] as u16) << 8 | (hash[5] as u16),
            time_high_and_version: ((hash[6] as u16) << 8 | (hash[7] as u16)) & 0xfff
                | (Version::SHA1 as u16) << 12,
            clock_seq_high_and_reserved: (hash[8] & 0xf) | (Variant::RFC as u8) << 4,
            clock_seq_low: hash[9] as u8,
            node: [hash[10], hash[11], hash[12], hash[13], hash[14], hash[15]],
        }
    }
}
