pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("inputs/day_3.txt") {
        Ok(body_string) => {
            let mut total_sum: i32 = 0;

            for line in body_string.split("\n") {
                let compartments = line.chars().collect::<Vec<char>>();
                let (compartment1, compartment2) = compartments.split_at((compartments.len()) / 2);
                let mut matching: Option<u8> = None;
                for char in compartment1 {
                    for char2 in compartment2 {
                        if char == char2 {
                            matching = Some(find_char_priority(char))
                        }
                    }
                }
                if let Some(character_value) = matching {
                    total_sum += character_value as i32;
                } else {
                    println!("no matching character found")
                }
            }

            let mut sum_badges:i32 = 0;

            let lines = body_string.split("\n").collect::<Vec<&str>>();
            for group in lines.chunks(3) {
                match group {
                    [first, second, third] => {

                        let mut first_dedup:Vec<char> = first.chars().collect::<Vec<char>>();
                        first_dedup.sort();
                        first_dedup.dedup();
                        let mut second_dedup:Vec<char> = second.chars().collect::<Vec<char>>();
                        second_dedup.sort();
                        second_dedup.dedup();
                        let mut third_dedup:Vec<char> = third.chars().collect::<Vec<char>>();
                        third_dedup.sort();
                        third_dedup.dedup();

                        for f in &first_dedup{
                            for s in &second_dedup{
                                for t in &third_dedup{
                                    if (f == t) && (t == s) && (f == s) {
                                        sum_badges += find_char_priority(f) as i32;
                                        break
                                    }
                                }
                            }
                        };
                    }
                    [""] => println!("huge booboo in day 3"),
                    _ => panic!("huge booboo in day 3"),
                }
            }

            Some(vec![total_sum.to_string(), sum_badges.to_string()])
        }
        _ => panic!("oh no - day 1 is in trouble"),
    }
}

fn find_char_priority(character: &char) -> u8 {
    let mut char_value: u8 = 0;
    let mut char_bits = character.clone() as u8;
    char_bits <<= 3;
    char_bits >>= 3;
    char_value = char_bits;
    if character.is_uppercase() {
        char_value += 26;
    }
    char_value
}
