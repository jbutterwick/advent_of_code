use crate::days::*;

fn format_result_string(results: Vec<Vec<String>>) -> String {
    let mut result_string = String::new();
    for (day_index, result) in results.iter().enumerate() {
        result_string.push_str(&format!("Day {}: \n", day_index + 1));
        for (part_index, part) in result.iter().enumerate() {
            result_string.push_str(&format!("\tPart {}: \n", part_index + 1));
            result_string.push_str(&format!("\t\t{} \n", part));
        }
    }
    result_string
}

pub(crate) fn main() -> String {
    format_result_string(vec![day_1::run().unwrap(), day_2::run().unwrap()])
}
