extern crate threadpool;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

fn main() {
    let num_workers = 4;
    let num_jobs = 1000;
    let pool = ThreadPool::new(num_workers);
    let (tx, rx) = channel();
    for n in 0..num_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            let num = if (n >= 3 && n % 3 == 0) || (n >= 5 && n % 5 == 0) {
                n
            } else {
                0
            };
            tx.send(num)
                .expect("channel will be there waiting for the pool");
        });
    }
    let sum: usize = rx.iter().take(num_jobs).sum();
    println!("{}", sum);
}
