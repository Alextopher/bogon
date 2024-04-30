/// An extension trait for checking if an IP address is in a bogon network.
///
/// This trait is [sealed](https://rust-lang.github.io/api-guidelines/future-proofing.html) and cannot be implemented for types outside of this crate.
pub trait BogonExt: sealed::Sealed {
    /// Returns a boolean indicating whether an IP address is bogus.
    ///
    /// Returns `true` if the IP address is bogus.
    /// Returns `false` if the IP address is good.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    /// use bogon::BogonExt;
    ///
    /// assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_bogon(), true);
    /// assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)).is_bogon(), true);
    ///
    /// assert_eq!(Ipv4Addr::new(8, 8, 8, 8).is_bogon(), false);
    /// assert_eq!(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0x1111, 0, 0, 0, 2).is_bogon(), false);
    /// ```
    fn is_bogon(&self) -> bool;
}

impl BogonExt for std::net::IpAddr {
    #[inline]
    fn is_bogon(&self) -> bool {
        crate::is_bogon(*self)
    }
}

impl BogonExt for std::net::Ipv4Addr {
    #[inline]
    fn is_bogon(&self) -> bool {
        crate::is_bogon_v4(*self)
    }
}

impl BogonExt for std::net::Ipv6Addr {
    #[inline]
    fn is_bogon(&self) -> bool {
        crate::is_bogon_v6(*self)
    }
}

mod sealed {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    pub trait Sealed {}

    impl Sealed for IpAddr {}
    impl Sealed for Ipv4Addr {}
    impl Sealed for Ipv6Addr {}
}
