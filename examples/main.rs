use std::thread::{self, available_parallelism};
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

    let available_machine_parallelism = available_parallelism().unwrap().get();

    let semaphore = simple_semaphore::Semaphore::new_available_parallelism().unwrap(); // Also uses `available_parallelism()` internally
    for _ in 0..(available_machine_parallelism + 2) {
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
    thread::sleep(Duration::from_millis(
        (((available_machine_parallelism + 1) * 1000) / 2) as u64,
    ));
}
