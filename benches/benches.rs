#![feature(test)]
#![cfg(any(
    feature = "mac",
    feature = "random",
    feature = "hash_md5",
    feature = "hash_sha1",
))]

extern crate test;
use test::Bencher;

use simple_uuid::{Node, Random, TimeStamp, UUID};

#[bench]
fn bench_new_v1(b: &mut Bencher) {
    b.iter(|| TimeStamp::new());
}

#[bench]
fn bench_new_v3(b: &mut Bencher) {
    b.iter(|| UUID::new_v3("any", UUID::NAMESPACE_DNS));
}

#[bench]
fn bench_new_v4(b: &mut Bencher) {
    b.iter(|| Random::new());
}

#[bench]
fn bench_new_v5(b: &mut Bencher) {
    b.iter(|| UUID::new_v5("any", UUID::NAMESPACE_X500));
}

#[bench]
fn bench_from_mac_address(b: &mut Bencher) {
    b.iter(|| Node::from(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80])));
}

#[bench]
fn bench_from_utc(b: &mut Bencher) {
    b.iter(|| TimeStamp::from(0x1234));
}
