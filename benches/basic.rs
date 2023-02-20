use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hint::black_box;
use std::str::FromStr;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let orig = bigdecimal::BigDecimal::from_str("1234.5678").unwrap();
    let floppa = bigfloppa::BigDecimal::from_str("1234.5678").unwrap();

    c.bench_function("original", |b| b.iter(|| black_box(&orig).hash(&mut DefaultHasher::new())));
    c.bench_function("floppa", |b| b.iter(|| black_box(&floppa).hash(&mut DefaultHasher::new())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
