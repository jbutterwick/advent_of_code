use std::cmp::max;
use std::convert::identity;

pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("../inputs/day2.txt") {
        Ok(body_string) => {
            let sum: u32 = body_string
                .lines()
                .zip(1..)
                .filter_map(|(line, id)| {
                    line.split_once(':')
                        .unwrap()
                        .1
                        .split(|ch| ch == ';' || ch == ',')
                        .map(|cube| {
                            let (value, color) = cube.trim().split_once(' ').unwrap();
                            value.parse::<u32>().unwrap()
                                <= match color {
                                    "red" => 12,
                                    "green" => 13,
                                    "blue" => 14,
                                    _ => unreachable!(),
                                }
                        })
                        .all(identity)
                        .then_some(id)
                })
                .sum();

            match std::fs::read_to_string("../inputs/day2p2.txt") {
                Ok(body_string) => {
                    let sum2: u32 = body_string
                        .lines()
                        .map(|line| {
                            line.split_once(':')
                                .unwrap()
                                .1
                                .split(|ch| ch == ',' || ch == ';')
                                .fold([0, 0, 0], |mut acc, cube| {
                                    let (value, color) = cube.trim().split_once(' ').unwrap();
                                    let index = match color {
                                        "red" => 0,
                                        "green" => 1,
                                        "blue" => 2,
                                        _ => unreachable!(),
                                    };
                                    acc[index] = max(acc[index], value.parse().unwrap());
                                    acc
                                })
                                .iter()
                                .product::<u32>()
                        })
                        .sum();
                    Some(vec![sum.to_string(), sum2.to_string()])
                }

                _ => panic!("couldn't read day 2 part 2 file"),
            }
        }
        _ => panic!("couldn't read day 2 file"),
    }
}
