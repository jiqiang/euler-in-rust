use criterion::Criterion;
use euler::solution::s004;

pub fn bench_p004_1(c: &mut Criterion) {
    c.bench_function("p004_1", |b| {
        b.iter(|| s004::is_palindrome_1("1234321"));
    });
}

pub fn bench_p004_2(c: &mut Criterion) {
    c.bench_function("p004_2", |b| {
        b.iter(|| s004::is_palindrome_2("1234321"));
    });
}
