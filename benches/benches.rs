#![feature(test)]
#![cfg(any(
    feature = "mac",
    feature = "random",
    feature = "hash_md5",
    feature = "hash_sha1",
))]

extern crate test;
use test::Bencher;

use simple_uuid::{self, Node, UUID};

#[bench]
fn new_v1_from_system_time(b: &mut Bencher) {
    b.iter(|| simple_uuid::v1!());
}

#[bench]
fn new_v1_from_mac_address(b: &mut Bencher) {
    b.iter(|| Node::from(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80])));
}

#[bench]
fn new_v1_from_utc(b: &mut Bencher) {
    b.iter(|| UUID::from_utc(0x1234));
}

#[bench]
fn new_v3_using_md5(b: &mut Bencher) {
    b.iter(|| simple_uuid::v3!("test_data", UUID::NAMESPACE_DNS));
}

#[bench]
fn new_v4_with_random_number(b: &mut Bencher) {
    b.iter(|| simple_uuid::v4!());
}

#[bench]
fn new_v5_using_sha1(b: &mut Bencher) {
    b.iter(|| simple_uuid::v5!("test_data", UUID::NAMESPACE_X500));
}
