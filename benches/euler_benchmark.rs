#[macro_use]
extern crate criterion;

mod benchmarks;

criterion_group!(
    p001,
    benchmarks::p001::bench_p001_1,
    benchmarks::p001::bench_p001_2
);

criterion_group!(p002, benchmarks::p002::bench_p002);

criterion_group!(p003, benchmarks::p003::bench_p003);

criterion_main! {
    p001,
    p002,
    p003,
}
