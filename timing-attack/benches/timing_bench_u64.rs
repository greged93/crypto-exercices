use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use timing_attack::SECRET;

fn bench_branching_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Branching");
    for i in [20u64, SECRET].iter() {
        group.bench_with_input(BenchmarkId::new("branch u64", i), i, |b, i| {
            b.iter(|| timing_attack::branching_u64(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_branching_u64);
criterion_main!(benches);
