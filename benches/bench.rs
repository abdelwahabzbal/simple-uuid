#![feature(test)]

extern crate test;

use test::Bencher;
use uuid_rs::*;

#[bench]
fn bench_v1(b: &mut Bencher) {
    b.iter(|| uuid_v1!());
}

#[bench]
fn bench_v2(b: &mut Bencher) {
    b.iter(|| uuid_v2!(Domain::PERSON));
}

#[bench]
fn bench_v3(b: &mut Bencher) {
    b.iter(|| uuid_v3!("any", UUID::NAMESPACE_DNS));
}

#[bench]
fn bench_v4(b: &mut Bencher) {
    b.iter(|| uuid_v4!());
}

#[bench]
fn bench_v5(b: &mut Bencher) {
    b.iter(|| uuid_v5!("any", UUID::NAMESPACE_X500));
}

#[bench]
fn bench_is_valid(b: &mut Bencher) {
    b.iter(|| UUID::is_valid(&uuid_v1!()));
}
