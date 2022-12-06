use std::ops::Range;
use std::str::{FromStr};

struct SectionIds((Range<i32>, Range<i32>));

impl FromStr for SectionIds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let to_int: fn(&str) -> Range<i32> = |str| {
            let split: Vec<i32> = str
                .split("-")
                .map(|s| s.parse().expect("parse error"))
                .collect();
            match split[..] {
                [first, second] => first..second,
                _ => panic!("not rangable"),
            }
        };

        match s.split(",").collect::<Vec<&str>>()[..] {
            [first, second] => Ok(SectionIds((to_int(first), to_int(second)))),
            _ => Err(()),
        }
    }
}

pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("inputs/day_4.txt") {
        Ok(body_string) => {
            let mut part1_answer = 0;
            let mut part2_answer = 0;

            for line in body_string.split("\n") {
                match line.parse::<SectionIds>().unwrap() {
                    SectionIds((first, second)) => {
                        if (first.start <= second.start) & (first.end >= second.end)
                            || (second.start <= first.start) & (second.end >= first.end)
                        {
                            part1_answer += 1;
                        }
                        if (first.start <= second.end) && (second.start <= first.end) {
                            part2_answer += 1;
                        }
                    }
                }
            }

            Some(vec![part1_answer.to_string(), part2_answer.to_string()])
        }
        _ => None,
    }
}
