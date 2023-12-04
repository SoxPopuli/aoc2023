use std::{error::Error, io::Read};

use regex::Regex;

type AnyResult<T> = Result<T, Box<dyn Error>>;

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_to_string(&mut buffer).map(|_| buffer)
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Type {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("{} not recognized", value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Turn {
    pub quantity: i32,
    pub typ: Type,
}

fn read_types(s: &str) -> AnyResult<Turn> {
    let parts = s.split_once(' ').ok_or("Space not found")?;

    let quantity = i32::from_str_radix(parts.0, 10)?;
    let typ = parts.1.try_into()?;

    Ok(Turn { quantity, typ })
}

#[derive(Debug, Clone)]
struct GameState {
    reds: i32,
    greens: i32,
    blues: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            reds: 12,
            greens: 13,
            blues: 14,
        }
    }

    pub fn simulate(&self, game: &[Turn]) -> bool {
        for turn in game {
            match turn.typ {
                Type::Red if turn.quantity > self.reds => return false,
                Type::Green if turn.quantity > self.greens => return false,
                Type::Blue if turn.quantity > self.blues => return false,
                _ => continue,
            }
        }

        true
    }

    pub fn power(&self) -> i32 {
        self.reds * self.blues * self.greens
    }

    pub fn count(game: &[Turn]) -> Self {
        let empty = GameState { reds: 0, greens: 0, blues: 0 };

        game.iter()
            .fold(empty, |acc, turn| {
                match turn.typ {
                    Type::Red => GameState { reds: acc.reds.max(turn.quantity), ..acc },
                    Type::Green => GameState { greens: acc.greens.max(turn.quantity), ..acc },
                    Type::Blue => GameState { blues: acc.blues.max(turn.quantity), ..acc },
                }
            })
    }
}

trait VecExt<T, E> {
    fn sequence(self) -> Result<Vec<T>, E>;
}

impl<T, E> VecExt<T, E> for Vec<Result<T, E>> {
    fn sequence(self) -> Result<Vec<T>, E> {
        let mut out_vec = Vec::with_capacity(self.len());

        for item in self {
            match item {
                Ok(x) => out_vec.push(x),
                Err(e) => return Err(e),
            };
        }

        Ok(out_vec)
    }
}

fn part1(input: &str) -> AnyResult<i32> {
    let state = GameState::new();
    let regex = Regex::new(r"(\d+ \w+)")?;

    let (sum, _) = input.lines().fold((0, 1), |(sum, idx), x| {
        let captures = regex
            .captures_iter(x)
            .map(|x| x.get(0).unwrap().as_str())
            .map(read_types)
            .collect::<Vec<_>>();

        let result = state.simulate(&captures.sequence().unwrap());
        if result {
            (sum + idx, idx + 1)
        } else {
            (sum, idx + 1)
        }
    });

    Ok(sum)
}

fn part2(input: &str) -> AnyResult<i32> {
    let regex = Regex::new(r"(\d+ \w+)")?;

    let sum = input.lines().fold(0, |sum, x| {
        let turns = regex
            .captures_iter(x)
            .map(|x| x.get(0).unwrap().as_str())
            .map(read_types)
            .collect::<Vec<_>>()
            .sequence()
            .unwrap();

        let count = GameState::count(&turns);
        let power = count.power();

        sum + power
    });

    Ok(sum)
}

fn main() -> AnyResult<()> {
    let input = read_stdin()?;

    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input).unwrap());

    Ok(())
}
