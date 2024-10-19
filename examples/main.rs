use std::thread;
use std::{sync::Arc, time::Duration};

extern crate simple_semaphore;

fn main() {
    let semaphore = simple_semaphore::Semaphore::new(2);

    for _ in 0..5 {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || {
            let permit = semaphore.acquire();
            thread::sleep(Duration::from_millis(500));
            drop(permit);
        });
    }
    thread::sleep(Duration::from_millis(3000));

    let cores = num_cpus::get();

    let semaphore = simple_semaphore::Semaphore::default(); // Also uses `num_cpus::get()` internally
    for _ in 0..(cores + 2) {
        let semaphore = Arc::clone(&semaphore);
        thread::spawn(move || {
            if let Some(permit) = semaphore.try_acquire() {
                thread::sleep(Duration::from_millis(500));
                drop(permit);
            } else {
                println!("Too many permits given, exiting the thread"); // Will be printed twice
            }
        });
    }
    thread::sleep(Duration::from_millis((((cores + 1) * 1000) / 2) as u64));
}
