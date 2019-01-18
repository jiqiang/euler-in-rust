extern crate threadpool;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn run_concurrently(n1: u32, n2: u32, max_num: u32, num_of_workers: u32) -> u32 {
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

pub fn run(n1: u32, n2: u32, max_num: u32) -> u32 {
    let mut sum: u32 = 0;
    for n in 0..max_num {
        if (n >= n1 && n % n1 == 0) || (n >= n2 && n % n2 == 0) {
            sum += n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p001_concurrently() {
        assert_eq!(233168, run_concurrently(3, 5, 1000, 4));
    }

    #[test]
    fn p001_not_concurrently() {
        assert_eq!(233168, run(3, 5, 1000));
    }
}
