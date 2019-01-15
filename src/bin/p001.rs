extern crate threadpool;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

fn main() {
    let num_of_workers = 4;
    let max_num = 1000;
    let pool = ThreadPool::new(num_of_workers);
    let (sender, receiver) = channel();
    for n in 0..max_num {
        let sender = sender.clone();
        pool.execute(move || {
            let value = if (n >= 3 && n % 3 == 0) || (n >= 5 && n % 5 == 0) {
                n
            } else {
                0
            };
            sender.send(value)
                .expect("channel will be there waiting for the pool");
        });
    }
    let sum: usize = receiver.iter().take(max_num).sum();
    println!("{}", sum);
}
