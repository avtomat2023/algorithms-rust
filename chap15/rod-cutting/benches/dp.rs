use rod_cutting::*;
use criterion::{Criterion, criterion_group, criterion_main};

const PRICES: [u32; 40] = [
     0,  1,  5,  8,  9, 10, 17, 17, 20, 24,
    30, 31, 35, 35, 35, 37, 40, 42, 43, 50,
    51, 55, 56, 57, 60, 63, 68, 69, 71, 75,
    77, 78, 80, 85, 85, 85, 85, 90, 91, 91
];

fn bench(c: &mut Criterion) {
    c.bench_function("bottom_up_dp",
                     |b| b.iter(|| bottom_up_dp(39, &PRICES)));
    c.bench_function("top_down_dp_hash_map",
                     |b| b.iter(|| top_down_dp_hash_map(39, &PRICES)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
