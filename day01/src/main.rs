use std::{collections::HashMap, error::Error, io::Read};

use fancy_regex::Regex;

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_to_string(&mut buffer).map(|_| buffer)
}

fn get_numbers(s: &str) -> Option<i32> {
    let digits: Vec<_> = s.chars().filter(|ch| ch.is_digit(10)).collect();

    let concat = format!("{}{}", digits.first()?, digits.last()?);

    i32::from_str_radix(&concat, 10).ok()
}

fn part1(input: &str) -> Option<i32> {
    input
        .lines()
        .map(get_numbers)
        .filter_map(|x| x)
        .reduce(|acc, x| acc + x)
}

fn get_numbers_2(re: &Regex, map: &HashMap<&str, i32>, s: &str) -> Result<i32, Box<dyn Error>> {
    fn resolve(map: &HashMap<&str, i32>, x: &str) -> i32 {
        match map.get(x) {
            Some(value) => *value,
            None => i32::from_str_radix(x, 10).unwrap(),
        }
    }

    let values = re
        .captures_iter(s)
        .filter_map(|captures| {
            let cap = 
                captures.expect("capture error").get(1)?;

            Some(resolve(&map, cap.as_str()))
        })
        .collect::<Vec<_>>();

    let concat = format!("{}{}", values.first().unwrap(), values.last().unwrap());

    Ok(i32::from_str_radix(&concat, 10)?)
}

fn part2(input: &str) -> Option<i32> {
    let re = Regex::new(r"(?=(zero|one|two|three|four|five|six|seven|eight|nine|\d))").unwrap();

    let map = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let lines = input.lines().map(|x| get_numbers_2(&re, &map, x));

    lines.filter_map(|x| x.ok()).reduce(|acc, x| acc + x)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_stdin()?;

    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input).unwrap());

    Ok(())
}
