use bin_utils::{ReadFixed, WriteFixed};
use criterion::{Criterion, criterion_group, criterion_main};
use ethernet::Ethernet2Header;

fn criterion_benchmark(c: &mut Criterion) {
    let ethernet_header_bytes = [
        0x00, 0x80, 0x41, 0xff, 0xf0, 0x0d,
        0x00, 0x80, 0x41, 0xba, 0xbe, 0xff,
        0x86, 0xdd
    ];
    c.bench_function("read_ethernet_header", |b| b.iter(|| {
        let _ = Ethernet2Header::from_bytes(&ethernet_header_bytes).unwrap();
    }));
    let ethernet_header = Ethernet2Header::from_bytes(&ethernet_header_bytes).unwrap();
    c.bench_function("write_ethernet_header", |b| b.iter(|| {
        let _ = ethernet_header.to_bytes();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);