mod days;

use crate::days::*;

#[tokio::main]
async fn main() {
    let results = days::main::main();
    println!(results);
}
