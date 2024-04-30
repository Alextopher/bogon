# Bogon

Bogon is a Rust library for checking whether an IP address is considered "bogus" or "bogon", meaning it's not valid for use on the public internet. This includes private IP addresses, loopback addresses, and other reserved addresses.

## Features

- Supports both IPv4 and IPv6 addresses.
- Includes methods for checking bogus IP addresses using extension traits.
- Efficiently handles a predefined list of known bogus IP address ranges.
- IPv6 ranges are generated from the [IANA reserved address registry](https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml).
