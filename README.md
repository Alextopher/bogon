# Bogon

[![Rust](https://github.com/Alextopher/bogon/actions/workflows/rust.yaml/badge.svg)](https://github.com/Alextopher/bogon/actions/workflows/rust.yaml)

Bogon is a Rust library for checking whether an IP address is considered "bogus" or "bogon", meaning it's not valid for use on the public internet. This includes private IP addresses, loopback addresses, and other reserved addresses.

## Features

- Supports both IPv4 and IPv6 addresses.
- Includes methods for checking bogus IP addresses using extension traits.
- Efficiently handles a predefined list of known bogus IP address ranges.
- IPv6 ranges are generated at build time from the [IANA reserved address registry](https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml).

## Examples

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use bogon::is_bogon_str;

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
```
