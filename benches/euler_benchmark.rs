#[macro_use]
extern crate criterion;

mod benchmarks;

criterion_group!(
    p001,
    benchmarks::p001::bench_p001_concurrently,
    benchmarks::p001::bench_p001_not_concurrently
);
criterion_main! {
    p001,
}
