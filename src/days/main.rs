use crate::days::*;

pub(crate) fn main() -> Vec<String> {
    let mut day_1 = String::from("day 1 results : ");
    let (part1, part2) = &day_1::run().unwrap();
    day_1.push_str("Part 1: \n");
    day_1.push_str(part1);
    day_1.push_str("Part 2: \n");
    day_1.push_str(part2);
    let mut day_2 = String::from("day 2 result : ");
    let (day2_part1,day2_part2) = &day_2::run().unwrap();
    day_2.push_str("Part 1: \n");
    day_2.push_str(day2_part1);
    day_2.push_str("Part 2: \n");
    day_2.push_str(day2_part2);
    return vec![day_1, day_2]
}