## UUID ![](https://github.com/wohabz/simid/workflows/uuid-rs/badge.svg)
A universally unique identifier (UUID) is a 128-bit number used to identify
information in computer systems. The term globally unique identifier (GUID)
is also used.

This crate generates and inspects UUIDs based on
 * [RFC 4122](http://tools.ietf.org/html/rfc4122)
 * [DCE 1.1](https://pubs.opengroup.org/onlinepubs/9696989899/chap5.htm#tagcjh_08_02_01_01)

## Install
```TOML
[dependencies]
simid = { version = "0.1.0", features = ["random"] }
```

## Usage
```Rust
use simid::v4;

println!("{}", v4!())
```

## Security

Do not assume that UUIDs are hard to guess; they should not be used as security capabilities.
