extern crate threadpool;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn run_concurrently(n1: usize, n2: usize, max_num: usize) -> usize {
    let num_of_workers = 4;
    let pool = ThreadPool::new(num_of_workers);
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
    receiver.iter().take(max_num).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p001() { 
        assert_eq!(233168, run_concurrently(3, 5, 1000));
    }
}
