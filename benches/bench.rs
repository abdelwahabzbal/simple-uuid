#![feature(test)]

extern crate test;
use test::Bencher;

use uuid_rs::{UUID, Domain};

#[bench]
fn bench_new_v1(b: &mut Bencher) {
    b.iter(|| UUID::new_v1());
}

#[bench]
fn bench_new_v2(b: &mut Bencher) {
    b.iter(|| UUID::new_v2(Domain::PERSON));
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
