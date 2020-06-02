extern crate uuid_rs;

use uuid_rs::base::time::*;
use uuid_rs::*;

#[test]
fn test_time_based_macros() {
    assert!(UUID::is_valid(&uuid_rs::uuid_v1!()));
    assert!(UUID::is_valid(&uuid_rs::uuid_v2!(Domain::ORG)));
}

#[test]
fn test_name_based_macros() {
    assert!(UUID::is_valid(&uuid_rs::uuid_v3!(
        "any",
        UUID::NAMESPACE_DNS
    )));
    assert!(UUID::is_valid(&uuid_rs::uuid_v5!(
        "any",
        UUID::NAMESPACE_OID
    )));
}

#[test]
fn test_random_based_macros() {
    assert!(UUID::is_valid(&uuid_rs::uuid_v4!()));
}
