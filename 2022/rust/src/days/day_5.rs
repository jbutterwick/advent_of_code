pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("inputs/day_5.txt") {
        Ok(body_string) => {
            let mut part1_answer = String::new();
            let mut part2_answer = String::new();

            let (crates, moves) = match body_string.split("\n\n").collect::<Vec<&str>>()[..] {
                [crates, moves] => (crates, moves),
                _ => panic!("wrong number of strings found"),
            };

            let mut crate_vec: Vec<&str> = crates.split("\n").collect::<Vec<&str>>();
            crate_vec.pop();
            crate_vec.reverse();

            // println!("crates : {:?} moves : {:?}", crate_vec, moves);

            let mut crate_matrix: Vec<Vec<&str>> = vec![vec![]; 9];
            let mut p1answer_matrix: Vec<Vec<&str>> = vec![vec![]; 9];
            let mut p2answer_matrix: Vec<Vec<&str>> = vec![vec![]; 9];

            for line in crate_vec {
                let vec_of_line = line.split(" ").collect::<Vec<&str>>();

                let mut empty_crate_incrementer: i8 = 0;
                let mut i: usize = 0;
                for elf_crate in vec_of_line {
                    match elf_crate {
                        "" => {
                            if empty_crate_incrementer == 3 {
                                i += 1;
                                empty_crate_incrementer = 0;
                            } else {
                                empty_crate_incrementer += 1;
                            }
                        }
                        item => {
                            crate_matrix[i].push(item);
                            if i == 9 {
                                i = 0
                            } else {
                                i += 1
                            }
                        }
                    }
                }
            }
            // println!("{:?}", crate_matrix);

            for move_line in moves.split("\n") {
                let altered_string = String::from(move_line)
                    .replace("move", "")
                    .replace("from", "")
                    .replace("to", "");
                match altered_string.split("  ").collect::<Vec<&str>>()[..] {
                    [move_instr, from_instr, to_instr] => {

                        let move_stripped = match move_instr.strip_prefix(" ") {
                            Some(value) => {
                                value
                            },
                            None => {
                                move_instr
                            }
                        };
                        let move_instr_int = move_stripped.parse::<usize>().unwrap();
                        let from_instr_int = from_instr.parse::<usize>().unwrap() - 1;
                        let to_instr_int = to_instr.parse::<usize>().unwrap() - 1;
                        for i in 0..(move_instr_int-1) {
                            let crate_length = crate_matrix[from_instr_int].len();
                            if i >= crate_length {
                                p1answer_matrix[to_instr_int].push(&crate_matrix[from_instr_int][crate_length - 1]);

                            } else {
                                p1answer_matrix[to_instr_int].push(&crate_matrix[from_instr_int][i]);
                            }
                        }
;
                        let mut pre_p2answer_vec:Vec<&str> = vec![];

                        for i in 0..(move_instr_int-1) {
                            let crate_length = crate_matrix[from_instr_int].len();
                            if i >= crate_length {
                                pre_p2answer_vec.push(&crate_matrix[from_instr_int][crate_length - 1]);
                                pre_p2answer_vec.reverse();
                                p2answer_matrix[to_instr_int].append(&mut pre_p2answer_vec)
                            } else {
                                pre_p2answer_vec.push(&crate_matrix[from_instr_int][i]);
                                pre_p2answer_vec.reverse();
                                p2answer_matrix[to_instr_int].append(&mut pre_p2answer_vec)
                            }
                        }
                    }
                    _ => panic!("wrong number of strings found"),
                };
                // println!("{:?}", altered_string);
            }


            println!("{:?}",p1answer_matrix);
            // println!("{:?}",p2answer_matrix);


            Some(vec![part1_answer, part2_answer])
        }
        _ => None,
    }
}
