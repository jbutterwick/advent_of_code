#![feature(iter_collect_into)]

mod days;

pub(crate) fn main() {
    let results = days::main::main();
    println!("{:?}",results);
}
