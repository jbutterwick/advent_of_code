pub(crate) fn run() -> Option<(String,String)> {
    match std::fs::read_to_string("/home/jordanbutterwick/IdeaProjects/advent_22/src/days/inputs/day_1.txt") {
        Ok(body_string) => {
            let mut all_elves:Vec<i32> = vec![];
            let mut this_elf:Vec<i32> = vec![];
            for line in body_string.split("\n") {
                println!("{:?}", line);
                match line.parse::<i32>(){
                    Ok(number) => {
                        this_elf.push(number);
                    },
                    Err(_) => {
                        all_elves.push(this_elf.iter().sum());
                        this_elf.clear();
                    }
                }
            }
            all_elves.sort();
            let sum_of_top_3:i32 = all_elves.iter().rev().take(3).sum();
            Some((all_elves.iter().max()?.to_string(), sum_of_top_3.to_string()))
        },
        _ => panic!("oh no")
    }
}