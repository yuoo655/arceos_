#![no_std]
#![no_main]

#[macro_use]
extern crate libax;
extern crate alloc;

#[no_mangle]
fn main() {
    libax::println!("Hello world! test for cvitek-phy");
}
