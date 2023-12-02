use core::str::from_utf8;

pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("../inputs/day1.txt") {
        Ok(body_string) => {
            let part_2_output = match std::fs::read_to_string("../inputs/day1p2.txt") {
                Ok(body_string) => {
                    let mut res: i32 = 0;
                    for line in body_string.lines() {
                        let mut str = line.to_string();
                        let bytes: &mut [u8] = unsafe { str.as_bytes_mut() };
                        let first = get_first_digit(bytes).unwrap().clone();
                        bytes.reverse();
                        let second = get_last_digit(bytes).unwrap().clone();
                        let calibration =
                            from_utf8(&[first, second]).unwrap().parse::<i32>().unwrap();
                        res += calibration;
                    }
                    res.to_string()
                }
                _ => panic!("couldn't read day 1 part 2 file"),
            };

            let mut output_vec: Vec<u32> = vec![];
            for line in body_string.lines() {
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

fn get_first_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => panic!("Digit not found!"),
        [byte, ..] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [b'o', b'n', b'e', ..] => Some(&b'1'),
        [b't', b'w', b'o', ..] => Some(&b'2'),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(&b'3'),
        [b'f', b'o', b'u', b'r', ..] => Some(&b'4'),
        [b'f', b'i', b'v', b'e', ..] => Some(&b'5'),
        [b's', b'i', b'x', ..] => Some(&b'6'),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(&b'7'),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(&b'8'),
        [b'n', b'i', b'n', b'e', ..] => Some(&b'9'),
        [_, tail @ ..] => get_first_digit(tail),
    }
}

fn get_last_digit(bytes: &[u8]) -> Option<&u8> {
    match bytes {
        [] => panic!("Digit not found!"),
        [byte, ..] if (byte >= &b'0' && byte <= &b'9') => Some(byte),
        [b'e', b'n', b'o', ..] => Some(&b'1'),
        [b'o', b'w', b't', ..] => Some(&b'2'),
        [b'e', b'e', b'r', b'h', b't', ..] => Some(&b'3'),
        [b'r', b'u', b'o', b'f', ..] => Some(&b'4'),
        [b'e', b'v', b'i', b'f', ..] => Some(&b'5'),
        [b'x', b'i', b's', ..] => Some(&b'6'),
        [b'n', b'e', b'v', b'e', b's', ..] => Some(&b'7'),
        [b't', b'h', b'g', b'i', b'e', ..] => Some(&b'8'),
        [b'e', b'n', b'i', b'n', ..] => Some(&b'9'),
        [_, tail @ ..] => get_last_digit(tail),
    }
}
