use core::net::{Ipv4Addr, Ipv6Addr};

use bogon::BogonExt;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn generate_random_ipv4() -> Vec<Ipv4Addr> {
    (0..1_024)
        .map(|_| {
            Ipv4Addr::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
            )
        })
        .collect()
}

fn generate_random_ipv6() -> Vec<Ipv6Addr> {
    (0..1_024)
        .map(|_| {
            Ipv6Addr::new(
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
                rand::random::<u16>(),
            )
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ipv4 is_bogon", |b| {
        let ips = generate_random_ipv4();
        b.iter(|| {
            for ip in &ips {
                black_box(ip.is_bogon());
            }
        })
    });

    c.bench_function("ipv6 is_bogon", |b| {
        let ips = generate_random_ipv6();
        b.iter(|| {
            for ip in &ips {
                black_box(ip.is_bogon());
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
