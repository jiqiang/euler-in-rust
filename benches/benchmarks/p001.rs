use criterion::Criterion;
use euler::solution::s001;

pub fn bench_p001_1(c: &mut Criterion) {
    c.bench_function("p001_1", |b| b.iter(|| s001::multiples_of_3_and_5_1(3, 5, 1000, 4)));
}

pub fn bench_p001_2(c: &mut Criterion) {
    c.bench_function("p001_2", |b| {
        b.iter(|| s001::multiples_of_3_and_5_2(3, 5, 1000))
    });
}
