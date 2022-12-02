#[tokio::main]
pub(crate) async fn run() -> String {

    let body = std::fs::read("/src/inputs/day_1.txt").unwrap();

    println!(body);

    String::from("day_1")
}