use core::net::{Ipv4Addr, Ipv6Addr};

/// Since all of the IPv4 and IPv6 bogon networks have at most 32-bit prefixes we can preform
/// all of our network calculations with 32-bit integers.
///
/// When it comes time to check if an IP address is contained within a slice of networks
/// the compiler will generate SIMD instructions to check multiple networks at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FourByteNetwork {
    network: u32,
    mask: u32,
}

impl FourByteNetwork {
    pub(crate) const fn new(network: u32, prefix: u8) -> Self {
        let mask = u32::MAX << (32 - prefix);
        Self { network, mask }
    }

    pub(crate) const fn contains_v4(&self, ip: Ipv4Addr) -> bool {
        (ip.to_bits() & self.mask) == self.network
    }

    pub(crate) const fn contains_v6(&self, ip: Ipv6Addr) -> bool {
        let ip = ip.to_bits() >> 96;
        (ip as u32 & self.mask) == self.network
    }

    #[cfg(test)]
    pub const fn prefix(&self) -> u8 {
        32 - self.mask.leading_zeros() as u8
    }
}
