use criterion::Criterion;
use euler::solution::s003;

pub fn bench_p003(c: &mut Criterion) {
    c.bench_function("p003", |b| {
        b.iter(|| s003::largest_prime_factor(600_851_475_143));
    });
}
