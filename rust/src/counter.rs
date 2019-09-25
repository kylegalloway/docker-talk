use std::sync::atomic::{AtomicIsize, Ordering};

pub struct Counter(AtomicIsize);

impl Counter {
    pub fn new(init: isize) -> Counter {
        Counter(AtomicIsize::new(init))
    }

    pub fn increment(&self) -> isize {
        let old = self.0.fetch_add(1, Ordering::SeqCst);
        old + 1
    }

    pub fn decrement(&self) -> isize {
        let old = self.0.fetch_sub(1, Ordering::SeqCst);
        old - 1
    }

    pub fn get(&self) -> isize {
        self.0.load(Ordering::SeqCst)
    }
}
