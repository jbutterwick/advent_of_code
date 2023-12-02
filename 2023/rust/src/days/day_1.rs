use regex::Regex;

pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("../inputs/day1.txt") {
        Ok(body_string) => {
            let part_2_output = match std::fs::read_to_string("../inputs/day1p2.txt") {
                Ok(body_string) => {
                    let mut output_vec: Vec<u32> = vec![];
                    let exp =
                        Regex::new(r"eno|owt|eerht|rouf|evif|xis|neves|thgie|enin|\d").unwrap();
                    for line in body_string.split('\n') {
                        let rev_line = line.chars().rev().collect::<String>();
                        let captures = exp.find_iter(&rev_line);
                        let all_digits: Vec<u32> = captures
                            .map(|cap| match cap.as_str() {
                                "eno" => 1,
                                "owt" => 2,
                                "eerht" => 3,
                                "rouf" => 4,
                                "evif" => 5,
                                "xis" => 6,
                                "neves" => 7,
                                "thgie" => 8,
                                "enin" => 9,
                                digit => digit.trim().parse::<u32>().ok().unwrap(),
                            })
                            .collect();
                        println!("line {}", rev_line);
                        println!("parsed digits: {}", all_digits.len());

                        let string = all_digits.last().unwrap().to_string()
                            + &all_digits.first().unwrap().to_string();

                        println!("line {}", string);

                        if let Ok(value) = string.trim().parse::<u32>() {
                            output_vec.push(value);
                        } else {
                            println! {"eerrrs"}
                        }
                    }

                    output_vec.iter().sum::<u32>().to_string()
                }
                _ => panic!("couldn't read day 1 part 2 file"),
            };

            let mut output_vec: Vec<u32> = vec![];
            for line in body_string.split('\n') {
                let all_digits: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
                let string = all_digits.first().unwrap().to_string()
                    + &all_digits.last().unwrap().to_string();
                output_vec.push(string.parse::<u32>().ok()?);
            }

            Some(vec![
                output_vec.iter().sum::<u32>().to_string(),
                part_2_output,
            ])
        }
        _ => panic!("couldn't read day 1 file"),
    }
}
