pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("src/days/inputs/day_1.txt") {
        Ok(body_string) => {
            let mut all_elves: Vec<i32> = vec![];
            let mut this_elf: Vec<i32> = vec![];
            for line in body_string.split("\n") {
                match line.parse::<i32>() {
                    Ok(number) => {
                        this_elf.push(number);
                    }
                    Err(_) => {
                        all_elves.push(this_elf.iter().sum());
                        this_elf.clear();
                    }
                }
            }
            all_elves.sort();
            let sum_of_top_3: i32 = all_elves.iter().rev().take(3).sum();
            Some(vec![
                all_elves.iter().max()?.to_string(),
                sum_of_top_3.to_string(),
            ])
        }
        _ => panic!("oh no - day 1 is in trouble"),
    }
}
