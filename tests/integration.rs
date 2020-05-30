extern crate uuid_rs;

use uuid_rs::base::time::*;
use uuid_rs::*;

#[test]
fn test_macro() {
    assert!(Uuid::is_valid(&uuid_rs::uuid_v1!()));
    assert!(Uuid::is_valid(&uuid_rs::uuid_v2!(Domain::ORG)));
    assert!(Uuid::is_valid(&uuid_rs::uuid_v3!(
        "any",
        Uuid::NAMESPACE_DNS
    )));
    assert!(Uuid::is_valid(&uuid_rs::uuid_v4!()));
    assert!(Uuid::is_valid(&uuid_rs::uuid_v5!(
        "any",
        Uuid::NAMESPACE_OID
    )));
}
