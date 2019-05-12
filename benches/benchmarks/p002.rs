use criterion::Criterion;
use euler::solution::s002;

pub fn bench_p002(c: &mut Criterion) {
    c.bench_function("p002", |b| {
        b.iter(|| s002::even_fibonacci_numbers_1(4_000_000))
    });
}
