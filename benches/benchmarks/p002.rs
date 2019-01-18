use criterion::Criterion;
use euler::solution::s002;

pub fn bench_p002(c: &mut Criterion) {
    c.bench_function("p002", |b| b.iter(|| s002::run(4000000)));
}
