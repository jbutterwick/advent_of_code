#![feature(slice_take)]
extern crate core;

mod days;

pub(crate) fn main() {
    println!("{}", days::main::main());
}
