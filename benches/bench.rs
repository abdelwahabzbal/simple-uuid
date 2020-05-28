#![feature(test)]
#![cfg(feature = "v1")]
#![cfg(feature = "v2")]
#![cfg(feature = "v3")]
#![cfg(feature = "v5")]

extern crate test;

use test::Bencher;
use uuid_rs::times::*;
use uuid_rs::Uuid;

#[bench]
fn bench_v1(b: &mut Bencher) {
    let uuid = Uuid::v1();
    b.iter(|| uuid.as_bytes());
}

#[bench]
fn bench_v2(b: &mut Bencher) {
    let uuid = Uuid::v2(Domain::PERSON);
    b.iter(|| uuid.as_bytes());
}

#[bench]
fn bench_v3(b: &mut Bencher) {
    let uuid = Uuid::v3("any", Uuid::NAMESPACE_DNS);
    b.iter(|| uuid.as_bytes());
}

#[bench]
fn bench_v4(b: &mut Bencher) {
    let uuid = Uuid::v4();
    b.iter(|| uuid.as_bytes());
}

#[bench]
fn bench_v5(b: &mut Bencher) {
    let uuid = Uuid::v5("any", Uuid::NAMESPACE_X500);
    b.iter(|| uuid.as_bytes());
}

#[bench]
fn bench_is_valid_lower(b: &mut Bencher) {
    let uuid = Uuid::v1();
    b.iter(|| Uuid::is_valid(&format!("{:x}", uuid.as_bytes())));
}

#[bench]
fn bench_is_valid_upper(b: &mut Bencher) {
    let uuid = Uuid::v1();
    b.iter(|| Uuid::is_valid(&format!("{:X}", uuid.as_bytes())));
}
