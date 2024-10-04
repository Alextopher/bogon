# Bogon

[![Rust](https://github.com/Alextopher/bogon/actions/workflows/rust.yaml/badge.svg)](https://github.com/Alextopher/bogon/actions/workflows/rust.yaml)
[![Crates.io](https://img.shields.io/crates/v/bogon)](https://crates.io/crates/bogon)
[![Docs.rs](https://docs.rs/bogon/badge.svg)](https://docs.rs/bogon)

Bogon is a Rust library for checking whether an IP address is considered "bogus" or "bogon", meaning it's not valid for use on the public internet. This includes private IP addresses, loopback addresses, and other reserved addresses.

## Features

- Supports both IPv4 and IPv6 addresses.
- Includes methods for checking bogus IP addresses using extension traits.
- Strives to be as fast as possible. The compiler generates SIMD instructions for both IPv4 and IPv6 address checks.
- IPv6 ranges are generated at build time from the [IANA reserved address registry](https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml).
- The `download` feature can be enabled to download the latest reserved address ranges from the IANA registry at build time.
- No dependencies, `no_std` compatible.

### TODO

- [ ] Updating address ranges at runtime.

## MSRV

Rust 1.80 is the minimum supported rust version due to the use of `to_bits()`. It's unlikely that the MSRV will be increased in the future.

## Examples

```rust
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

assert_eq!(bogon::is_bogon_str("127.0.0.1"), Ok(true));
assert_eq!(bogon::is_bogon_str("8.8.8.8"), Ok(false));
assert_eq!(bogon::is_bogon_str("::1"), Ok(true));
assert!(bogon::is_bogon_str("foo").is_err());

assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(127, 0, 0, 1)), true);
assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(8, 8, 8, 8)), false);
assert_eq!(bogon::is_bogon_v6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), true);

assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), true);
assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), false);
assert_eq!(bogon::is_bogon(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))), true);

use bogon::BogonExt;

assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_bogon(), true);
assert_eq!(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)).is_bogon(), false);
assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)).is_bogon(), true);
```
