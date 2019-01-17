use criterion::Criterion;
use euler::solution::s001;

pub fn bench_p001_concurrently(c: &mut Criterion) {
    c.bench_function("p001", |b| b.iter(|| s001::run_concurrently(3, 5, 1000, 4)));
}

pub fn bench_p001_not_concurrently(c: &mut Criterion) {
    c.bench_function("p001_not_concurrently", |b| {
        b.iter(|| s001::run(3, 5, 1000))
    });
}
