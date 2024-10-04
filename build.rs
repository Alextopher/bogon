// Prefix,Designation,Date,WHOIS,RDAP,Status,Note
// 2001:0000::/23,IANA,1999-07-01,whois.iana.org,,ALLOCATED,"2001:0000::/23 is reserved for IETF Protocol Assignments [RFC2928].
// 2001:0000::/32 is reserved for TEREDO [RFC4380].

use ipnetwork::Ipv6Network;
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
    // Parse the CSV file into a Vec<Ipv6Allocation>.
    let allocations = get_ipv6_allocations();

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
    let networks = networks
        .into_iter()
        .map(four_byte_networks)
        .collect::<Vec<_>>();

    // Write the merged ranges to a file in the build directory.
    write_file(networks).unwrap();

    // Tell Cargo to rerun the build script if the CSV file changes.
    println!("cargo:rerun-if-changed=ipv6-unicast-address-assignments.csv");
}

/// Download the CSV file from the IANA website.
#[cfg(feature = "download")]
fn download_csv() -> Result<&'static str, Box<dyn std::error::Error>> {
    let url = "https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.csv";
    let user = format!(
        "bogon/{} ({}; {}) Rust/{}",
        std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set"),
        std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS not set"),
        std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH not set"),
        rustc_version::version_meta().unwrap().semver
    );

    // Build the client, it requires a user-agent string.
    // bogon/version (platform; arch) Rust/rustc.version
    let client = reqwest::blocking::Client::builder()
        .user_agent(user)
        .build()?;
    // Send the request and get the response body.
    let body = client.get(url).send()?.error_for_status()?;

    // require a successful response
    Ok(body.text()?.leak())
}

fn get_ipv6_allocations() -> Vec<Ipv6Allocation> {
    // try to download the CSV file from the IANA website
    #[cfg(feature = "download")]
    let csv = {
        // Retry up to 3 times with 1, 2, and 4 second delays.
        let mut retries = 0;
        loop {
            match download_csv() {
                Ok(csv) => break csv,
                Err(e) => {
                    if retries >= 3 {
                        eprintln!("Failed to download CSV file: {}", e);
                        std::process::exit(1);
                    }
                    retries += 1;
                    std::thread::sleep(std::time::Duration::from_secs(2u64.pow(retries)));
                }
            }
        }
    };
    #[cfg(not(feature = "download"))]
    let csv = include_str!("ipv6-unicast-address-assignments.csv");

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    rdr.deserialize().map(|result| result.unwrap()).collect()
}

/// Merge_ranges takes a list of Ipv6Networks and combines neighboring allocations into larger blocks to make
/// filtering more efficient. The algorithm works by converting networks from their CIDR representation to a
/// (start, end) tuple. Then merging is done by iterating over the list and combining neighbors when appropriate.
/// Finally, the merged ranges are converted back to Ipv6Networks
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

    // Convert the merged ranges back to a Vec<Ipv6Network>.
    let mut all_ranges: Vec<_> = merged_ranges
        .into_iter()
        .flat_map(range_to_networks)
        .collect();

    all_ranges.sort_by_key(|network| network.prefix());

    // Filter out networks that are subsets of other networks.
    let mut super_nets: Vec<Ipv6Network> = Vec::new();
    for network in &all_ranges {
        if super_nets
            .iter()
            .any(|super_net| super_net.contains(network.network()))
        {
            continue;
        }
        super_nets.push(*network);
    }

    // Sort the networks by start address.
    super_nets.sort();

    // Sort the networks by prefix size.
    super_nets
}

/// Convert a range of IP addresses to a list of Ipv6Networks.
fn range_to_networks(range: (u128, u128)) -> Vec<Ipv6Network> {
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

/// Since all RIR allocations have at most 32-bit prefixes we can preform all of our network calculations with 32-bit integers.
fn four_byte_networks(ip: Ipv6Network) -> (u32, u8) {
    let start = (ip.network().to_bits() >> 96) as u32;
    (start, ip.prefix())
}

/// Write the FourByteNetwork structs to a file.
fn write_file(networks: Vec<(u32, u8)>) -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let path = Path::new(&out_dir).join("ipv6-unicast-address-allocations.rs");
    let mut file = std::fs::File::create(path).unwrap();

    writeln!(file, "use crate::network::FourByteNetwork;")?;
    writeln!(
        file,
        "pub(crate) static V6_ALLOCATIONS: [FourByteNetwork; {}] = [",
        networks.len()
    )?;
    for (network, prefix) in networks {
        writeln!(
            file,
            "    FourByteNetwork::new({:#x}, {}),",
            network, prefix
        )?;
    }
    writeln!(file, "];")?;

    Ok(())
}
