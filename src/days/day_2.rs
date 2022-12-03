enum RPSResult {
    LOSS,
    DRAW,
    WIN,
}

impl From<(bool, bool)> for RPSResult {
    fn from(move_tuple: (bool, bool)) -> Self {
        match move_tuple {
            (false, true) => RPSResult::LOSS,
            (true, false) => RPSResult::WIN,
            (false, false) => RPSResult::DRAW,
            _ => panic!("didn't recognize RPS Move matrix or impossible result (true,true)"),
        }
    }
}

enum RPSMove {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RPSMove {
    fn get_result(&self, other: &RPSMove) -> bool {
        match (self, other) {
            (RPSMove::ROCK, RPSMove::SCISSORS) => true,
            (RPSMove::SCISSORS, RPSMove::PAPER) => true,
            (RPSMove::PAPER, RPSMove::ROCK) => true,
            _ => false,
        }
    }

    fn get_move(&self, result: &RPSResult) -> RPSMove {
        match (self, result) {
            (RPSMove::ROCK, RPSResult::WIN) => RPSMove::PAPER,
            (RPSMove::PAPER, RPSResult::WIN) => RPSMove::SCISSORS,
            (RPSMove::SCISSORS, RPSResult::WIN) => RPSMove::ROCK,
            (RPSMove::ROCK, RPSResult::LOSS) => RPSMove::SCISSORS,
            (RPSMove::PAPER, RPSResult::LOSS) => RPSMove::ROCK,
            (RPSMove::SCISSORS, RPSResult::LOSS) => RPSMove::PAPER,
            (RPSMove::ROCK, RPSResult::DRAW) => RPSMove::ROCK,
            (RPSMove::PAPER, RPSResult::DRAW) => RPSMove::PAPER,
            (RPSMove::SCISSORS, RPSResult::DRAW) => RPSMove::SCISSORS,
        }
    }
}

impl From<&str> for RPSMove {
    fn from(string: &str) -> Self {
        match string {
            "A" => RPSMove::ROCK,
            "B" => RPSMove::PAPER,
            "C" => RPSMove::SCISSORS,
            "X" => RPSMove::ROCK,
            "Y" => RPSMove::PAPER,
            "Z" => RPSMove::SCISSORS,
            _ => panic!("didn't recognize RPS Move"),
        }
    }
}

impl From<&RPSMove> for RPSResult {
    fn from(rps_move: &RPSMove) -> Self {
        match rps_move {
            RPSMove::ROCK => RPSResult::LOSS,
            RPSMove::PAPER => RPSResult::DRAW,
            RPSMove::SCISSORS => RPSResult::WIN,
        }
    }
}

struct RPSGameP1 {
    you: RPSMove,
    result: RPSResult,
}
struct RPSGameP2 {
    you: RPSMove,
    result: RPSResult,
}

impl RPSGameP1 {
    fn from_game(game: &str) -> Option<RPSGameP1> {
        let split = game.split(" ");
        match split.collect::<Vec<&str>>()[..] {
            [elf, you] => {
                let elf_move = RPSMove::from(elf);
                let your_move = RPSMove::from(you);
                let result = (
                    your_move.get_result(&elf_move),
                    elf_move.get_result(&your_move),
                );
                Some(RPSGameP1 {
                    you: your_move,
                    result: RPSResult::from(result),
                })
            }
            [""] => None,
            _ => panic!("weird line found"),
        }
    }
}
impl RPSGameP2 {
    fn from_game(game: &str) -> Option<RPSGameP2> {
        let split = game.split(" ");
        match split.collect::<Vec<&str>>()[..] {
            [elf, you] => {
                let result = RPSResult::from(&RPSMove::from(you));
                let elf_move = RPSMove::from(elf);
                let your_move = elf_move.get_move(&result);

                Some(RPSGameP2 {
                    you: your_move,
                    result: RPSResult::from(result),
                })
            }
            [""] => None,
            _ => panic!("weird line found"),
        }
    }
}

pub(crate) fn run() -> Option<Vec<String>> {
    let mut part1_total_score: i32 = 0;
    let mut part2_total_score: i32 = 0;

    match std::fs::read_to_string("inputs/day_2.txt") {
        Ok(body_string) => {
            for line in body_string.split("\n") {
                match RPSGameP1::from_game(line) {
                    Some(game) => {
                        let result_score = match game.result {
                            RPSResult::WIN => 6,
                            RPSResult::DRAW => 3,
                            RPSResult::LOSS => 0,
                        };
                        let move_score = match game.you {
                            RPSMove::ROCK => 1,
                            RPSMove::PAPER => 2,
                            RPSMove::SCISSORS => 3,
                        };
                        part1_total_score += move_score + result_score;
                    }
                    None => println!("found a nothing"),
                }
                match RPSGameP2::from_game(line) {
                    Some(game) => {
                        let result_score = match game.result {
                            RPSResult::WIN => 6,
                            RPSResult::DRAW => 3,
                            RPSResult::LOSS => 0,
                        };
                        let move_score = match game.you {
                            RPSMove::ROCK => 1,
                            RPSMove::PAPER => 2,
                            RPSMove::SCISSORS => 3,
                        };
                        part2_total_score += move_score + result_score;
                    }
                    None => println!("found a nothing"),
                }
            }
        }
        _ => panic!("big oops on day 2"),
    }
    Some(vec![
        part1_total_score.to_string(),
        part2_total_score.to_string(),
    ])
}
