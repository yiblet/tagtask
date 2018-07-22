extern crate futures;
extern crate futures_cpupool;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

mod pool;
mod view;

use futures::Future;

fn test() -> impl futures::Future<Item = (), Error = &'static str> {
    println!("testing testing");
    futures::future::ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let pool = pool::get();
    println!("Hello, world!");
    pool.spawn_fn(test).wait()?;
    Ok(())
}
