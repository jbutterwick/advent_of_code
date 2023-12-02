pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("../../inputs/day1.txt") {
        Ok(body_string) => {
            let mut output_vec: Vec<u32> = vec![];
            for line in body_string.split('\n') {
                let all_digits: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
                let string = all_digits.first().unwrap().to_string()
                    + &all_digits.last().unwrap().to_string();
                output_vec.push(string.parse::<u32>().ok()?);
            }

            Some(vec![output_vec.iter().sum::<u32>().to_string()])
        }
        _ => panic!("couldn't read day 1 file"),
    }
}
