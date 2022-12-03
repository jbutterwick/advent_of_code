#![feature(slice_as_chunks)]
#![feature(slice_partition_dedup)]

mod days;

pub(crate) fn main() {
    println!("{}", days::main::main());
}
