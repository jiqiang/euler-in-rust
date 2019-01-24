use criterion::Criterion;
use euler::solution::s004;

pub fn bench_p004(c: &mut Criterion) {
    c.bench_function("p004", |b| {
        b.iter(|| s004::largest_palindrome_product(100, 999));
    });
}
