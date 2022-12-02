use crate::days::*;
#[tokio::main]
pub(crate) async fn main() -> Vec<String>{
    let day_1 = String::from("day 1 result : ") + day_1::run();

    return vec![day_1]
}