use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

fn branching_u64(x: [u64; 100]) {
    let secret_array: [u64; 100] = (0..100)
        .into_iter()
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    let mut _counter = 0;
    for _ in 0..1000 {
        if x == secret_array {
            _counter += 1;
        } else {
            match x {
                x if x[0] <= 10 => _counter += 2,
                x if x[0] > 10 => _counter -= 2,
                _ => (),
            }
        }
        _counter *= 2;
    }
}

fn bench_branching_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Branching");
    let random: [u64; 100] = thread_rng()
        .sample_iter(Uniform::new(1u64, 100u64))
        .take(100)
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    for (i, arr) in [
        random,
        (0..100)
            .into_iter()
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap(),
    ]
    .iter()
    .enumerate()
    {
        group.bench_with_input(BenchmarkId::new("branch u64", i), arr, |b, arr| {
            b.iter(|| branching_u64(*arr))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_branching_u64);
criterion_main!(benches);
