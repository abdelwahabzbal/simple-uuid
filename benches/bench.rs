#![feature(test)]
#![cfg(feature = "mac")]
#![cfg(feature = "hash")]

extern crate test;

use test::Bencher;
use uuid_rs::base::time;
use uuid_rs::uuid_v1;
use uuid_rs::uuid_v2;
use uuid_rs::uuid_v3;
use uuid_rs::uuid_v4;
use uuid_rs::uuid_v5;
use uuid_rs::Uuid;

#[bench]
fn bench_v1(b: &mut Bencher) {
    b.iter(|| uuid_v1!());
}

#[bench]
fn bench_v2(b: &mut Bencher) {
    b.iter(|| uuid_v2!(time::Domain::PERSON));
}

#[bench]
fn bench_v3(b: &mut Bencher) {
    b.iter(|| uuid_v3!("any", Uuid::NAMESPACE_DNS));
}

#[bench]
fn bench_v4(b: &mut Bencher) {
    b.iter(|| uuid_v4!());
}

#[bench]
fn bench_v5(b: &mut Bencher) {
    b.iter(|| uuid_v5!("any", Uuid::NAMESPACE_X500));
}

#[bench]
fn bench_is_valid_lower(b: &mut Bencher) {
    let uuid = Uuid::v1();
    b.iter(|| Uuid::is_valid(&format!("{:x}", uuid.as_bytes())));
}

#[bench]
fn bench_is_valid_upper(b: &mut Bencher) {
    let uuid = Uuid::v2(time::Domain::ORG);
    b.iter(|| Uuid::is_valid(&format!("{:X}", uuid.as_bytes())));
}
