use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_branching_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("Branching");
    let empty: [u64; 100] = vec![3; 100].try_into().unwrap();
    let secret = (0..100)
        .into_iter()
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();

    for (i, arr) in [empty, secret].iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("branch array", i), arr, |b, arr| {
            b.iter(|| timing_attack::branching_array(*arr))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_branching_array);
criterion_main!(benches);
