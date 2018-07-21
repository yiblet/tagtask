extern crate futures;
extern crate futures_cpupool;
extern crate state;

use futures_cpupool::*;

pub static POOL: state::LocalStorage<CpuPool> = state::LocalStorage::new();

pub fn init() {
    if let None = POOL.try_get() {
        let pool = CpuPool::new_num_cpus();
        POOL.set(move || pool.clone());
    }
}
