use std::{
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::available_parallelism,
};

/// A Semaphore maintains the number of permits it is still allowed to give.
///
/// * When `acquire()` is called and the semaphore still has permits to give, it will return a `Permit`. If there are no permits that can be given, it will wait for one permit to be given back from a thread so that it can return a new `Permit`.
/// * When `try_acquire()` is called and the semaphore still has permits to give, it will return `Some(Permit)`. If there are no permits that can be given, it will return `None`.
#[derive(Debug)]
pub struct Semaphore {
    permits: Arc<AtomicUsize>,
    condvar: Condvar,
    mutex: Mutex<()>,
}

impl Semaphore {
    /// Returns a new `Arc<Semaphore>` with the limit of permits chosen by you.
    pub fn new(permits: usize) -> Arc<Self> {
        Arc::new(Semaphore {
            permits: Arc::new(AtomicUsize::new(permits)),
            condvar: Condvar::new(),
            mutex: Mutex::new(()),
        })
    }

    /// Returns a new `Arc<Semaphore>` with the limit of permits set to the machine's parallelism value, usually CPU cores.
    pub fn new_available_parallelism() -> Result<Arc<Self>, String> {
        match available_parallelism() {
            Ok(parallelism) => Ok(Arc::new(Semaphore {
                permits: Arc::new(AtomicUsize::new(parallelism.get())),
                condvar: Condvar::new(),
                mutex: Mutex::new(()),
            })),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Returns the number of available permits
    pub fn available_permits(self: &Arc<Self>) -> usize {
        self.permits.load(Ordering::Relaxed)
    }

    /// Tries to get a `Permit`. If no more permits can be given, it will wait for one permit to be given back from a thread so that it can return a new `Permit`.
    #[allow(unused_must_use)]
    pub fn acquire(self: &Arc<Self>) -> Permit {
        loop {
            if self.permits.load(Ordering::Acquire) != 0 {
                self.permits.fetch_sub(1, Ordering::AcqRel);
                return Permit {
                    semaphore: Arc::clone(self),
                };
            }
            let guard = self.mutex.lock().unwrap();
            self.condvar.wait(guard).unwrap();
        }
    }

    /// Tries to get a `Option<Permit>`. If no more permits can be given, it will return `None`.
    pub fn try_acquire(self: &Arc<Self>) -> Option<Permit> {
        if self.permits.load(Ordering::Acquire) != 0 {
            self.permits.fetch_sub(1, Ordering::Release);
            return Some(Permit {
                semaphore: Arc::clone(self),
            });
        }
        None
    }

    /// Releases a permit. This is what `drop()` on `Permit` calls, ideally use `drop(permit)`.
    pub fn release(&self) {
        self.permits.fetch_add(1, Ordering::Release);
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
