use common::{read_stdin, AnyResult};

use regex::Regex;
use std::collections::hash_set::HashSet;

struct Card {
    index: i32,
    winning: Vec<i32>,
    owned: Vec<i32>,
}
impl Card {
    fn count_winning(&self) -> i32 {
        let map: HashSet<&i32> = HashSet::from_iter(self.winning.iter());

        self.owned.iter().filter(|x| map.contains(x)).count() as i32
    }
}

fn read_numbers(line: &str) -> Card {
    let re = Regex::new(r"^Card\s*(\d+):\s*([\d+\s*]+)\s*\|\s*([\d+\s*]+)$").expect("Regex error");

    let captures: Vec<_> = re.captures(line).unwrap().iter().skip(1).collect();

    match captures.as_slice() {
        &[Some(index), Some(winning), Some(owned)] => {
            let index = i32::from_str_radix(index.as_str(), 10).unwrap();

            let winning = winning
                .as_str()
                .split(' ')
                .filter_map(|x| i32::from_str_radix(x, 10).ok())
                .collect();
            let owned = owned
                .as_str()
                .split(' ')
                .filter_map(|x| i32::from_str_radix(x, 10).ok())
                .collect();

            Card {
                index,
                winning,
                owned,
            }
        }

        _ => panic!(),
    }
}

fn count_matches(line: &str) -> i32 {
    let card = read_numbers(line);

    let matches = card.count_winning();

    if matches > 0 {
        2_i32.pow(matches as u32 - 1)
    } else {
        0
    }
}

fn part1(input: &str) -> i32 {
    input.lines().map(count_matches).fold(0, |acc, x| acc + x)
}

fn part2(input: &str) -> i32 {
    let cards: Vec<_> = input.lines().map(read_numbers).collect();

    todo!()
}

fn main() -> AnyResult<()> {
    let input = read_stdin()?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}
