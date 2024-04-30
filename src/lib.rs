//! Functions for checking whether an IP address is bogus.
//!
//! Here "bogus" or "bogon" means an IP address that is not valid for use on the
//! public internet. This includes private IP addresses, loopback addresses, and
//! other reserved addresses. The primary goal of this crate is to provide
//! efficient filters for use in geo-ip tools.
//!
//! # Example
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
//! use bogon::is_bogon_str;
//!
//! assert_eq!(bogon::is_bogon_str("127.0.0.1"), Ok(true));
//! assert_eq!(bogon::is_bogon_str("8.8.8.8"), Ok(false));
//! assert_eq!(bogon::is_bogon_str("::1"), Ok(true));
//! assert!(bogon::is_bogon_str("foo").is_err());
//!
//! assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(127, 0, 0, 1)), true);
//! assert_eq!(bogon::is_bogon_v4(Ipv4Addr::new(8, 8, 8, 8)), false);
//! assert_eq!(bogon::is_bogon_v6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), true);
//!
//! assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))), true);
//! assert_eq!(bogon::is_bogon(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))), false);
//! assert_eq!(bogon::is_bogon(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))), true);
//!
//! use bogon::BogonExt;
//!
//! assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_bogon(), true);
//! assert_eq!(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)).is_bogon(), false);
//! assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)).is_bogon(), true);
//! ```
use ipnetwork::Ipv4Network;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use ext::BogonExt;

mod ext;
#[cfg(test)]
mod net_tests;

mod ipv6_unicast_address_allocations {
    include!(concat!(
        env!("OUT_DIR"),
        "/ipv6-unicast-address-allocations.rs"
    ));
}

// Bogus IPv4 networks.
//
// SAFETY: Ipv4Network::new_unchecked is safe here as long as the prefix length is less than or equal to 32
static V4_BOGON_NETWORKS: once_cell::sync::Lazy<[Ipv4Network; 15]> =
    once_cell::sync::Lazy::new(|| {
        [
            // "This Network"
            Ipv4Network::new(Ipv4Addr::new(0, 0, 0, 0), 8).unwrap(),
            // Private-Use
            Ipv4Network::new(Ipv4Addr::new(10, 0, 0, 0), 8).unwrap(),
            // Shared Address Space
            Ipv4Network::new(Ipv4Addr::new(100, 64, 0, 0), 10).unwrap(),
            // Loopback
            Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 0), 8).unwrap(),
            // Link Local
            Ipv4Network::new(Ipv4Addr::new(169, 254, 0, 0), 16).unwrap(),
            // Private-Use
            Ipv4Network::new(Ipv4Addr::new(172, 16, 0, 0), 12).unwrap(),
            // IETF Protocol Assignments
            Ipv4Network::new(Ipv4Addr::new(192, 0, 0, 0), 24).unwrap(),
            // Documentation (TEST-NET-1)
            Ipv4Network::new(Ipv4Addr::new(192, 0, 2, 0), 24).unwrap(),
            // Private-Use
            Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 0), 16).unwrap(),
            // "Benchmarking"
            Ipv4Network::new(Ipv4Addr::new(198, 18, 0, 0), 15).unwrap(),
            // TEST-NET-2
            Ipv4Network::new(Ipv4Addr::new(198, 51, 100, 0), 24).unwrap(),
            // TEST-NET-3
            Ipv4Network::new(Ipv4Addr::new(203, 0, 113, 0), 24).unwrap(),
            // Multicast
            Ipv4Network::new(Ipv4Addr::new(224, 0, 0, 0), 4).unwrap(),
            // Reserved
            Ipv4Network::new(Ipv4Addr::new(240, 0, 0, 0), 4).unwrap(),
            // Limited Broadcast
            Ipv4Network::new(Ipv4Addr::new(255, 255, 255, 255), 32).unwrap(),
        ]
    });

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
#[inline]
pub fn is_bogon(ip_address: IpAddr) -> bool {
    match ip_address {
        IpAddr::V4(ip) => is_bogon_v4(ip),
        IpAddr::V6(ip) => is_bogon_v6(ip),
    }
}

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
#[inline]
pub fn is_bogon_str(ip_address: impl AsRef<str>) -> Result<bool, std::net::AddrParseError> {
    ip_address.as_ref().parse().map(is_bogon)
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
#[inline]
pub fn is_bogon_v4(ip_address: Ipv4Addr) -> bool {
    V4_BOGON_NETWORKS
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
#[inline]
pub fn is_bogon_v6(ip_address: Ipv6Addr) -> bool {
    // If the IP is outside 2000::/3, it is not a global unicast address.
    if ip_address.segments()[0] & 0xe000 != 0x2000 {
        return true;
    }

    // Bring the IP address into the IPv4 space for comparison.
    let v4 = Ipv4Addr::from((u128::from(ip_address) >> 96) as u32);

    !ipv6_unicast_address_allocations::V6_ALLOCATIONS
        .iter()
        .any(|&network| network.contains(v4))
}
