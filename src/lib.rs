use std::{
    sync::{Arc, Condvar, Mutex},
    thread::available_parallelism,
};

/// A Semaphore maintains the number of permits it is still allowed to give.
///
/// * When `acquire()` is called and the semaphore still has permits to give, it will return a `Permit`. If there are no permits that can be given, it will wait for one permit to be given back from a thread so that it can return a new `Permit`.
/// * When `try_acquire()` is called and the semaphore still has permits to give, it will return `Some(Permit)`. If there are no permits that can be given, it will return `None`.
#[derive(Debug)]
pub struct Semaphore {
    permits: Mutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    /// Returns a new `Arc<Semaphore>` with the limit of permits chosen by you.
    pub fn new(permits: usize) -> Arc<Self> {
        Arc::new(Semaphore {
            permits: Mutex::new(permits),
            condvar: Condvar::new(),
        })
    }

    /// Returns a new `Arc<Semaphore>` with the limit of permits set to the machine's parallelism value, usually CPU cores.
    pub fn new_available_parallelism() -> Result<Arc<Self>, String> {
        match available_parallelism() {
            Ok(parallelism) => Ok(Arc::new(Semaphore {
                permits: Mutex::new(parallelism.get()),
                condvar: Condvar::new(),
            })),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Returns the number of available permits
    #[deprecated(
        since = "1.1.0",
        note = "Please do not use this method anymore, there will be no replacement for it either"
    )]
    pub fn available_permits(self: &Arc<Self>) -> usize {
        *self.permits.lock().unwrap()
    }

    /// Tries to get a `Permit`. If no more permits can be given, it will wait for one permit to be given back from a thread so that it can return a new `Permit`.
    pub fn acquire(self: &Arc<Self>) -> Permit {
        let mut permits = self.permits.lock().unwrap();
        while *permits == 0 {
            permits = self.condvar.wait(permits).unwrap();
        }
        *permits -= 1;
        Permit {
            semaphore: Arc::clone(self),
        }
    }

    /// Tries to get a `Option<Permit>`. If no more permits can be given, it will return `None`.
    pub fn try_acquire(self: &Arc<Self>) -> Option<Permit> {
        let mut permits = self.permits.lock().unwrap();
        if *permits > 0 {
            *permits -= 1;
            Some(Permit {
                semaphore: Arc::clone(self),
            })
        } else {
            None
        }
    }

    /// Releases a permit. This is what `drop()` on `Permit` calls, ideally use `drop(permit)`.
    pub fn release(&self) {
        let mut permits = self.permits.lock().unwrap();
        *permits += 1;
        self.condvar.notify_one();
    }
}

/// A permit that holds the Semaphore, so that `drop(permit)` can be called.
#[derive(Debug)]
pub struct Permit {
    semaphore: Arc<Semaphore>,
}

impl Drop for Permit {
    /// Releases the permit.
    fn drop(&mut self) {
        self.semaphore.release();
    }
}
