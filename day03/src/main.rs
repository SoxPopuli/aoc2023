#![allow(dead_code)]

use itertools::Itertools;
use std::num::Wrapping;

use common::AnyResult;

#[derive(Debug)]
struct Grid {
    data: Vec<String>,

    width: usize,
    height: usize,
}
impl Grid {
    pub fn from_string(input: &str) -> Self {
        let lines = input.lines();

        let mut data = vec![];
        let mut max = 0;

        for l in lines {
            max = max.max(l.len());

            data.push(l.into());
        }

        Self {
            height: data.len(),
            width: max,
            data,
        }
    }

    pub fn read_tokens<'a>(&'a self) -> Vec<Token<'a>> {
        let lines = self.data.iter().enumerate();

        let mut tokens = vec![];

        fn read<'a>(source: &'a Grid, tokens: &mut Vec<Token<'a>>, line: &str, line_index: usize) {
            let mut token: Option<Token<'a>> = None;
            for (j, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        if let Some(t) = token {
                            tokens.push(t.clone());
                            token = None;
                        }
                    }

                    x if x.is_digit(10) == false => {
                        match token {
                            Some(t) => {
                                tokens.push(t.clone());
                            }
                            None => {}
                        }
                        tokens.push(Token {
                            source,
                            line: line_index,
                            start: j,
                            end: j,
                        });
                        token = None;
                    }

                    _ => match token {
                        Some(ref mut t) => t.end = j,
                        None => {
                            token = Some(Token {
                                source,
                                line: line_index,
                                start: j,
                                end: j,
                            })
                        }
                    },
                }
            }

            if let Some(t) = token {
                tokens.push(t)
            }
        }

        for (idx, line) in lines {
            read(self, &mut tokens, line, idx);
        }

        tokens
    }
}

trait ClampedSub {
    fn clamped_sub(&self, x: Self) -> Self;
}
impl ClampedSub for usize {
    fn clamped_sub(&self, x: usize) -> Self {
        if *self == 0 {
            0
        } else {
            self - x
        }
    }
}

#[derive(Debug, Clone)]
struct Token<'a> {
    source: &'a Grid,

    line: usize,
    start: usize,
    ///inclusive :)
    end: usize,
}
impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.start == other.start && self.end == other.end
    }
}
impl Eq for Token<'_> {}
impl<'a> Token<'a> {
    fn as_str(&'a self) -> &'a str {
        let line = &self.source.data[self.line];
        &line[self.start..=self.end]
    }

    fn get_bordering_indices(&self) -> Vec<(usize, usize)> {
        let mut output = vec![];

        let line_range = self.line.clamped_sub(1)..=(self.line + 1);

        for idx in line_range {
            let idx = idx as usize;
            let chars = self.source.data.get(idx);

            if let Some(_) = chars {
                let mut bordering: Vec<(usize, usize)> = if idx == self.line {
                    let mut inner = vec![];

                    if self.start > 0 {
                        inner.push((idx, self.start - 1));
                    }

                    if self.end < self.source.width - 1 {
                        inner.push((idx, self.end + 1));
                    }

                    inner
                } else {
                    (self.start.clamped_sub(1)..=(self.end + 1).clamp(0, self.source.width - 1))
                        .map(|x| (idx, x))
                        .collect()
                };

                output.append(&mut bordering);
            }
        }

        output
    }

    fn get_bordering_chars(&self) -> Vec<char> {
        let indices = self.get_bordering_indices();

        indices
            .into_iter()
            .map(|(line, idx)| {
                let ch = self.source.data[line].as_bytes()[idx];

                ch as char
            })
            .filter(|x| *x != '.')
            .collect()
    }

    fn has_bordering_chars(&self) -> bool {
        self.get_bordering_chars().len() > 0
    }

    fn get_bordering_tokens(&'a self, others: &Vec<Token<'a>>) -> Vec<Token<'a>> {
        let indices = self.get_bordering_indices();

        let tokens =
            others
            .iter()
            .filter_map(|token| {
                for (line, idx) in &indices {
                    if (token.start ..= token.end).contains(&idx) && token.line == *line {
                        return Some(token.clone());
                    }

                }
                None
            });


        tokens.collect()
    }
}

fn part1(input: &str) -> i32 {
    let grid = Grid::from_string(&input);

    let tokens = grid.read_tokens();

    tokens
        .iter()
        //.map(|x| (x, x.get_bordering_chars()))
        .filter(|x| x.has_bordering_chars())
        .fold(0, |acc, x| {
            match i32::from_str_radix(x.as_str(), 10) {
                Ok(x) => acc + x,
                Err(_) => acc,
            }
        })
}

fn part2(input: &str) -> i32 {
    let grid = Grid::from_string(&input);
    let tokens = grid.read_tokens();

    let gears = 
        tokens.iter()
            .filter(|x| x.as_str() == "*")
            .map(|x| x.get_bordering_tokens(&tokens))
            .filter(|x| x.len() == 2);

    gears.fold(0, |acc, x| {
        let a = i32::from_str_radix(x[0].as_str(), 10);
        let b = i32::from_str_radix(x[1].as_str(), 10);

        match (a, b) {
            (Ok(a), Ok(b)) => {
                acc + (a * b)
            },

            _ => acc
        }
    })
}

fn main() -> AnyResult<()> {
    let input = common::read_stdin()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
