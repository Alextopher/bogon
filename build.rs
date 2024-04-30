// Prefix,Designation,Date,WHOIS,RDAP,Status,Note
// 2001:0000::/23,IANA,1999-07-01,whois.iana.org,,ALLOCATED,"2001:0000::/23 is reserved for IETF Protocol Assignments [RFC2928].
// 2001:0000::/32 is reserved for TEREDO [RFC4380].

use ipnetwork::{Ipv4Network, Ipv6Network};
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize, Clone)]
struct Ipv6Allocation {
    #[serde(rename = "Prefix")]
    prefix: Ipv6Network,
    #[serde(rename = "Designation")]
    designation: String,
    #[serde(rename = "Date")]
    _date: String,
    #[serde(rename = "WHOIS")]
    _whois: String,
    #[serde(rename = "RDAP")]
    _rdap: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Note")]
    _note: String,
}

use std::{env, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Read the CSV file.
    let csv = include_str!("ipv6-unicast-address-assignments.csv");
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());

    // Parse the CSV file into a Vec<Ipv6Allocation>.
    let allocations = rdr
        .deserialize()
        .map(|result| result.unwrap())
        .collect::<Vec<Ipv6Allocation>>();

    // IP address ranges are only considered reachable if they are both ALLOCATED and assigned
    // to one of the 5 regional internet registries (RIRs).
    let rirs = ["AFRINIC", "APNIC", "ARIN", "LACNIC", "RIPE NCC"];
    // Filter out the unallocated ranges.
    let networks = allocations
        .iter()
        .filter(|a| a.status == "ALLOCATED" && rirs.contains(&&*a.designation))
        .map(|a| a.prefix)
        .collect::<Vec<_>>();

    // Following the filtering, we now want to merge neighboring ranges into larger blocks.
    let networks = merge_ranges(networks);

    // Convert to IPv4 networks for more efficient comparisons.
    let networks = networks.into_iter().map(ipv6_to_ipv4).collect::<Vec<_>>();

    // Write the merged ranges to a file in the build directory.
    let path = Path::new(&out_dir).join("ipv6-unicast-address-allocations.rs");
    let mut file = std::fs::File::create(path).unwrap();

    // Include ipnetwork::Ipv6Network and std::Ipv6Addr.
    writeln!(file, "use ipnetwork::Ipv4Network;").unwrap();
    writeln!(file, "use std::net::Ipv4Addr;").unwrap();

    // Write the allocations to the file as a static array.
    writeln!(
        file,
        "pub(crate) static V6_ALLOCATIONS: [ipnetwork::Ipv4Network; {}] = unsafe {{ [",
        networks.len()
    )
    .unwrap();

    for allocation in networks {
        writeln!(
            file,
            "    Ipv4Network::new_unchecked(Ipv4Addr::new({}, {}, {}, {}), {}),",
            allocation.network().octets()[0],
            allocation.network().octets()[1],
            allocation.network().octets()[2],
            allocation.network().octets()[3],
            allocation.prefix()
        )
        .unwrap()
    }

    writeln!(file, "]}};").unwrap();

    // Tell Cargo to rerun the build script if the CSV file changes.
    println!("cargo:rerun-if-changed=ipv6-unicast-address-assignments.csv");
}

fn merge_ranges(mut ranges: Vec<Ipv6Network>) -> Vec<Ipv6Network> {
    // Firstly, sort the ranges by their start address.
    ranges.sort();

    // Next, we change the representation of the ranges from a Vec<Ipv6Network> to a Vec<(u128, u128)>.
    let ranges: Vec<(u128, u128)> = ranges
        .iter()
        .map(|range| (range.network().into(), range.broadcast().into()))
        .collect::<Vec<_>>();

    // Merging is allowed if the end of the current range is equal to the start of the next range.
    let mut merged_ranges = vec![ranges[0]];

    for &(start, end) in &ranges[1..] {
        let (_prev_start, prev_end) = merged_ranges.last_mut().unwrap();

        if start <= *prev_end + 1 {
            // Merge the ranges
            *prev_end = (*prev_end).max(end);
        } else {
            // Add a new range
            merged_ranges.push((start, end));
        }
    }

    // Finally, convert the merged ranges back to a Vec<Ipv6Network>.
    let mut all_ranges: Vec<_> = merged_ranges
        .into_iter()
        .flat_map(range_to_network)
        .collect();

    all_ranges.sort_by_key(|network| network.prefix());

    let mut super_nets: Vec<Ipv6Network> = Vec::new();
    for network in &all_ranges {
        // If any of the super networks contain the current network, skip it.
        if super_nets
            .iter()
            .any(|super_net| super_net.contains(network.network()))
        {
            continue;
        }

        // Otherwise, add the network to the list of super networks.
        super_nets.push(*network);
    }

    // Sort the networks by start address.
    super_nets.sort();

    // Sort the networks by prefix size.
    super_nets
}

/// Convert a range of IP addresses to a list of Ipv6Networks.
fn range_to_network(range: (u128, u128)) -> Vec<Ipv6Network> {
    // implemented by recursively taking the largest prefix length that fits the range
    let mut networks = Vec::new();
    let mut start = range.0;
    let end = range.1;

    while start <= end {
        // Find the largest power of 2 in the length of the range.
        let prefix_length = (end - start).leading_zeros();
        // Next, create the network.
        let network = Ipv6Network::new(start.into(), prefix_length as u8).unwrap();
        // Add the network to the list.
        networks.push(network);
        // Move the start address to the next network.
        start = u128::from(network.broadcast()) + 1;
    }

    // Do a second pass removing any networks that are subnets of other networks.
    networks
}

/// Since all RIR allocations are at least /32, we can safely treat them as ipv4 addresses for more efficient comparisons
fn ipv6_to_ipv4(ip: Ipv6Network) -> Ipv4Network {
    let start = (u128::from(ip.network()) >> 96) as u32;
    Ipv4Network::new(start.into(), ip.prefix()).unwrap()
}
