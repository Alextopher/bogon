use ipnetwork::Ipv4Network;

use crate::{network::FourByteNetwork, V4_BOGON_NETWORKS};

#[test]
fn check_v4_networks() {
    let bogus = &[
        "0.0.0.0/8",
        "10.0.0.0/8",
        "100.64.0.0/10",
        "127.0.0.0/8",
        "169.254.0.0/16",
        "172.16.0.0/12",
        "192.0.0.0/24",
        "192.0.2.0/24",
        "192.168.0.0/16",
        "198.18.0.0/15",
        "198.51.100.0/24",
        "203.0.113.0/24",
        "224.0.0.0/4",
        "240.0.0.0/4",
        "255.255.255.255/32",
    ]
    .iter()
    .map(|&s| s.parse().unwrap())
    .map(|n: Ipv4Network| FourByteNetwork::new(n.network().to_bits(), n.prefix()))
    .collect::<Vec<_>>();

    // Compare to the unsafe static V4_NETWORKS.
    for (a, b) in bogus.iter().zip(V4_BOGON_NETWORKS) {
        assert_eq!(a, &b);
    }

    // Double check that the prefix length is less than or equal to 32.
    for network in V4_BOGON_NETWORKS {
        assert!(network.prefix() <= 32);
    }
}
