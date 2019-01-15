#[macro_use]
extern crate criterion;
extern crate euler;

use criterion::Criterion;
use euler::solution::s001;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p001", |b| b.iter(|| s001::run_concurrently(3, 5, 1000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
