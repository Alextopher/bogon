use ipnetwork::{Ipv4Network, Ipv6Network};

use crate::{V4_NETWORKS, V6_NETWORKS};

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
    .collect::<Vec<Ipv4Network>>();

    // Compare to the unsafe static V4_NETWORKS.
    for (a, b) in bogus.iter().zip(V4_NETWORKS) {
        assert_eq!(a, b);
    }

    // Double check that the prefix length is less than or equal to 32.
    for network in V4_NETWORKS {
        assert!(network.prefix() <= 32);
    }
}

#[test]
fn check_v6_network() {
    let bogus = &[
        "::/128",
        "::1/128",
        "::ffff:0:0/96",
        "::/96",
        "100::/64",
        "2001:10::/28",
        "2001:db8::/32",
        "fc00::/7",
        "fe80::/10",
        "fec0::/10",
        "ff00::/8",
    ]
    .iter()
    .map(|&s| s.parse().unwrap())
    .collect::<Vec<Ipv6Network>>();

    // Compare to the unsafe static V6_NETWORKS.
    for (a, b) in bogus.iter().zip(V6_NETWORKS) {
        assert_eq!(a, b);
    }

    // Double check that the prefix length is less than or equal to 128.
    for network in V6_NETWORKS {
        assert!(network.prefix() <= 128);
    }
}
