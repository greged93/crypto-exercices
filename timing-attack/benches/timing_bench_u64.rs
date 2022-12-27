use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const SECRET: u64 = 123456789012345678;
fn branching_u64(x: u64) {
    let mut _counter = 0;
    for _ in 0..1000 {
        if x == SECRET {
            _counter += 1;
        } else {
            match x {
                x if x <= SECRET => _counter += 2,
                x if x > SECRET => _counter -= 2,
                _ => (),
            }
        }
        _counter *= 2;
    }
}

fn bench_branching_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("Branching");
    for i in [20u64, SECRET].iter() {
        group.bench_with_input(BenchmarkId::new("branch u64", i), i, |b, i| {
            b.iter(|| branching_u64(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_branching_u64);
criterion_main!(benches);
