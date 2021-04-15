#![feature(test)]
#![cfg(any(
    feature = "mac",
    feature = "random",
    feature = "hash_md5",
    feature = "hash_sha1",
))]

extern crate test;
use test::Bencher;

use simple_uuid::{Domain, Node, Version, UUID};

#[bench]
fn bench_new_v1(b: &mut Bencher) {
    b.iter(|| UUID::new_v1());
}

#[bench]
fn bench_new_v2(b: &mut Bencher) {
    b.iter(|| UUID::new_v2(Domain::PRN));
}

#[bench]
fn bench_new_v3(b: &mut Bencher) {
    b.iter(|| UUID::new_v3("any", UUID::NAMESPACE_DNS));
}

#[bench]
fn bench_new_v4(b: &mut Bencher) {
    b.iter(|| UUID::new_v4());
}

#[bench]
fn bench_new_v5(b: &mut Bencher) {
    b.iter(|| UUID::new_v5("any", UUID::NAMESPACE_X500));
}

#[bench]
fn bench_from_mac(b: &mut Bencher) {
    b.iter(|| UUID::from_mac(Node([0x03, 0x2a, 0x35, 0x0d, 0x13, 0x80]), Version::TIME));
}

#[bench]
fn bench_from_utc(b: &mut Bencher) {
    b.iter(|| UUID::from_utc(0x1234, Version::TIME));
}
