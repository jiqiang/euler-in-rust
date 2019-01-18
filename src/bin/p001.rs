extern crate euler;
use euler::solution::s001;

fn main() {
    println!("{:?}", s001::multiples_of_3_and_5_1(3, 5, 1000, 4));
    println!("{:?}", s001::multiples_of_3_and_5_2(3, 5, 1000));
}
