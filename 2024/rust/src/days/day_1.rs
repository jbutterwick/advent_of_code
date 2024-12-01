use std::str::FromStr;

pub trait FindAll: Iterator + Sized {
    fn find_all<P>(&mut self, predicate: P) -> Option<Vec<usize>>
    where
        P: FnMut(&Self::Item) -> bool;
}

impl<I> FindAll for I
where
    I: Iterator,
{
    fn find_all<P>(&mut self, mut predicate: P) -> Option<Vec<usize>>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut occurences = Vec::<usize>::default();
        for (index, element) in self.enumerate() {
            if predicate(&element) {
                occurences.push(index);
            }
        }

        match occurences.len() {
            0 => None,
            _ => Some(occurences),
        }
    }
}

pub(crate) fn run() -> Option<Vec<String>> {
    match std::fs::read_to_string("2024/inputs/day1.txt") {
        Ok(body_string) => {
            let outputs: Vec<Vec<i32>> = body_string
                .split('\n')
                .map(|s| s.splitn(2,"   ").map(|x| {i32::from_str(x).unwrap()}).collect::<Vec<i32>>())
                .collect();

            let mut list_a = outputs.iter().map(|s| s.get(0).unwrap().clone()).collect::<Vec<i32>>();
            let mut list_b = outputs.iter().map(|s| s.get(1).unwrap().clone()).collect::<Vec<i32>>();

            list_a.sort();
            list_b.sort();

            let zipped_list = list_a.iter().zip(list_b.iter()).collect::<Vec<(&i32, &i32)>>();

            let output_p1 = zipped_list.iter().fold(0, |mut a, b| { a += b.0.abs_diff(*b.1); a } );

            let output_p2: usize = list_a.iter().map(|a| {
                *a as usize * list_b.iter().find_all(|b| *b == a).unwrap_or(vec![]).len()
            }).sum();

            Some(vec![
                output_p1.to_string(),
                output_p2.to_string(),
            ])
        }
        _ => panic!("couldn't read day 1 file"),
    }
}
