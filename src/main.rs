use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("range_collect", |b| {
        b.iter(|| black_box(0..200_000).collect::<Vec<u64>>())
    });

    c.bench_function("rev_range_collect", |b| {
        b.iter(|| black_box(0..200_000).rev().collect::<Vec<u64>>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
