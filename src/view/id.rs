use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::*;

extern crate lazy_static;

lazy_static!  {
    static ref COUNT : AtomicUsize = AtomicUsize::new(0);
}


thread_local! {
    static LOCAL_COUNT : UnsafeCell<u64> = UnsafeCell::new((COUNT.fetch_add(1, Ordering::SeqCst) as u64) << 32);
}

pub trait Id {
    fn id(&self) -> u64;
}

impl<T: Id, D: Deref<Target = T>> Id for D {
    fn id(&self) -> u64 {
        self.deref().id()
    }
}

pub fn fetch() -> u64 {
    LOCAL_COUNT.with(
        |count : &UnsafeCell<u64>| unsafe {
            let y: *mut u64 = count.get();
            *y += 1;
            *y
        })
}
