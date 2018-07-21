extern crate futures;
extern crate futures_cpupool;
extern crate state;

#[macro_use]
extern crate derive_builder;

mod pool;
mod view;



fn main() -> Result<(), Box<std::error::Error>> {
    println!("Hello, world!");
    Ok(())
}
