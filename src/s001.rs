//! Problem 1: Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we
//! get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn multiples_of_3_and_5_1(n1: u32, n2: u32, max_num: u32, num_of_workers: u32) -> u32 {
    let pool = ThreadPool::new(num_of_workers as usize);
    let (sender, receiver) = channel();
    for n in 0..max_num {
        let sender = sender.clone();
        pool.execute(move || {
            let value = if (n >= n1 && n % n1 == 0) || (n >= n2 && n % n2 == 0) {
                n
            } else {
                0
            };
            sender
                .send(value)
                .expect("channel will be there waiting for the pool");
        });
    }
    receiver.iter().take(max_num as usize).sum()
}

pub fn multiples_of_3_and_5_2(n1: u32, n2: u32, max_num: u32) -> u32 {
    let mut sum: u32 = 0;
    for n in 0..max_num {
        if (n >= n1 && n % n1 == 0) || (n >= n2 && n % n2 == 0) {
            sum += n;
        }
    }
    sum
}

pub fn multiples_of_3_and_5_3(n1: u32, n2: u32, max_num: u32) -> u32 {
    (0..max_num)
        .filter(|&n| (n >= n1 && n % n1 == 0) || (n >= n2 && n % n2 == 0))
        .sum()
}

#[cfg(test)]
mod s001_tests {
    use super::*;

    #[test]
    fn test_multiples_of_3_and_5_1() {
        assert_eq!(233_168, multiples_of_3_and_5_1(3, 5, 1000, 4));
    }

    #[test]
    fn test_multiples_of_3_and_5_2() {
        assert_eq!(233_168, multiples_of_3_and_5_2(3, 5, 1000));
    }

    #[test]
    fn test_multiples_of_3_and_5_3() {
        assert_eq!(233_168, multiples_of_3_and_5_3(3, 5, 1000));
    }
}
