extern crate euler;
use euler::solution::s001;

fn main() {
    println!("{:?}", s001::run_concurrently(3, 5, 1000, 4));
}
