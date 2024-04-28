//! Functions for checking whether an IP address is bogus.
//!
//! Here "bogus" or "bogon" means an IP address that is not valid for use on the
//! public internet. This includes private IP addresses, loopback addresses, and
//! other reserved addresses.
//!
//! # Example
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
//! use bogon::is_bogon_str;
//!
//! assert_eq!(is_bogon_str("127.0.0.1"), Ok(true));
//! assert_eq!(is_bogon_str("8.8.8.8"), Ok(false));
//! assert_eq!(is_bogon_str("::1"), Ok(true));
//! assert!(is_bogon_str("foo").is_err());
//!
//! assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), true);
//! assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), false);
//! assert_eq!(bogon::is_bogon(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))), true);
//!
//! assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(127, 0, 0, 1)), true);
//! assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(8, 8, 8, 8)), false);
//!
//! assert_eq!(bogon::is_bogon_v6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), true);
//! ```
use ipnetwork::{Ipv4Network, Ipv6Network};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use ext::BogonExt;

mod ext;
#[cfg(test)]
mod net_tests;

// Bogus IPv4 networks.
//
// SAFETY: Ipv4Network::new_unchecked is safe here as long as the prefix length is less than or equal to 32
static V4_NETWORKS: &[Ipv4Network] = unsafe {
    &[
        Ipv4Network::new_unchecked(Ipv4Addr::new(0, 0, 0, 0), 8),
        Ipv4Network::new_unchecked(Ipv4Addr::new(10, 0, 0, 0), 8),
        Ipv4Network::new_unchecked(Ipv4Addr::new(100, 64, 0, 0), 10),
        Ipv4Network::new_unchecked(Ipv4Addr::new(127, 0, 0, 0), 8),
        Ipv4Network::new_unchecked(Ipv4Addr::new(169, 254, 0, 0), 16),
        Ipv4Network::new_unchecked(Ipv4Addr::new(172, 16, 0, 0), 12),
        Ipv4Network::new_unchecked(Ipv4Addr::new(192, 0, 0, 0), 24),
        Ipv4Network::new_unchecked(Ipv4Addr::new(192, 0, 2, 0), 24),
        Ipv4Network::new_unchecked(Ipv4Addr::new(192, 168, 0, 0), 16),
        Ipv4Network::new_unchecked(Ipv4Addr::new(198, 18, 0, 0), 15),
        Ipv4Network::new_unchecked(Ipv4Addr::new(198, 51, 100, 0), 24),
        Ipv4Network::new_unchecked(Ipv4Addr::new(203, 0, 113, 0), 24),
        Ipv4Network::new_unchecked(Ipv4Addr::new(224, 0, 0, 0), 4),
        Ipv4Network::new_unchecked(Ipv4Addr::new(240, 0, 0, 0), 4),
        Ipv4Network::new_unchecked(Ipv4Addr::new(255, 255, 255, 255), 32),
    ]
};

// Bogus IPv6 networks.
//
// SAFETY: Ipv6Network::new_unchecked is safe here as long as the prefix length is less than or equal to 128
static V6_NETWORKS: &[Ipv6Network] = unsafe {
    &[
        Ipv6Network::new_unchecked(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 128),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 128),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0, 0), 96),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 96),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0x100, 0, 0, 0, 0, 0, 0, 0), 64),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0x2001, 0x10, 0, 0, 0, 0, 0, 0), 28),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0), 32),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 0), 7),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0xfec0, 0, 0, 0, 0, 0, 0, 0), 10),
        Ipv6Network::new_unchecked(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0), 8),
    ]
};

/// Returns a boolean indicating whether an IP address is bogus.
///
/// Returns an error if the IP address is invalid.
/// Returns `Ok(true)` if the IP address is bogus.
/// Returns `Ok(false)` if the IP address is good.
///
/// # Examples
///
/// ```
/// use bogon::is_bogon_str;
///
/// assert_eq!(is_bogon_str("127.0.0.1"), Ok(true));
/// assert_eq!(is_bogon_str("8.8.8.8"), Ok(false));
/// assert_eq!(is_bogon_str("::1"), Ok(true));
/// assert_eq!(is_bogon_str("2606:4700:4700:1111::2"), Ok(false));
///
/// assert!(is_bogon_str("foo").is_err());
/// ```
pub fn is_bogon_str(ip_address: &str) -> Result<bool, std::net::AddrParseError> {
    ip_address.parse().map(is_bogon)
}

/// Returns a boolean indicating whether an IP address is bogus.
///
/// Returns `true` if the IP address is bogus.
/// Returns `false` if the IP address is good.
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
/// use bogon::is_bogon;
///
/// assert_eq!(is_bogon(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), true);
/// assert_eq!(is_bogon(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), false);
/// assert_eq!(is_bogon(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))), true);
/// assert_eq!(is_bogon(IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0x1111, 0, 0, 0, 2))), false);
/// ```
pub fn is_bogon(ip_address: IpAddr) -> bool {
    match ip_address {
        IpAddr::V4(ip) => is_bogon_v4(ip),
        IpAddr::V6(ip) => is_bogon_v6(ip),
    }
}

/// Returns a boolean indicating whether an IPv4 address is bogus.
///
/// Returns `true` if the IP address is bogus.
/// Returns `false` if the IP address is good.
///
/// # Examples
///
/// ```
/// use std::net::Ipv4Addr;
/// use bogon::is_bogon_v4;
///
/// assert_eq!(is_bogon_v4(Ipv4Addr::new(127, 0, 0, 1)), true);
/// assert_eq!(is_bogon_v4(Ipv4Addr::new(8, 8, 8, 8)), false);
/// ```
pub fn is_bogon_v4(ip_address: Ipv4Addr) -> bool {
    V4_NETWORKS
        .iter()
        .any(|&network| network.contains(ip_address))
}

/// Returns a boolean indicating whether an IPv6 address is bogus.
///
/// Returns `true` if the IP address is bogus.
/// Returns `false` if the IP address is good.
///
/// # Examples
///
/// ```
/// use std::net::Ipv6Addr;
/// use bogon::is_bogon_v6;
///
/// assert_eq!(is_bogon_v6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), true);
/// assert_eq!(is_bogon_v6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0x1111, 0, 0, 0, 2)), false);
/// ```
pub fn is_bogon_v6(ip_address: Ipv6Addr) -> bool {
    V6_NETWORKS
        .iter()
        .any(|&network| network.contains(ip_address))
}
