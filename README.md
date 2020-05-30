## UUID ![](https://github.com/awh6al/uuid-rs/workflows/uuid-rs/badge.svg)
A universally unique identifier (UUID) is a 128-bit number used to identify
information in computer systems. The term globally unique identifier (GUID)
is also used.

This crate generates and inspects UUIDs based on
 * [RFC 4122](http://tools.ietf.org/html/rfc4122)
 * [DCE 1.1](https://pubs.opengroup.org/onlinepubs/9696989899/chap5.htm#tagcjh_08_02_01_01)

## Install 
```TOML
[dependencies]
uuid-rs = "0.2.3"
```

## Usage
```Rust
use uuid_rs::uuid_v4;

println!("{}", uuid_v4!())
```
