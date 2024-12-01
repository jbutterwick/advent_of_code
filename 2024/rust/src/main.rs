extern crate core;

use std::env;

mod days;

pub(crate) fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    println!("{}", days::main::main());
}
