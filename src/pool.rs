extern crate futures;
extern crate futures_cpupool;

extern crate lazy_static;

use futures_cpupool::*;

pub fn get() -> CpuPool {
    lazy_static! {
        static ref POOL: CpuPool = CpuPool::new_num_cpus();
    }
    return POOL.clone();
}
