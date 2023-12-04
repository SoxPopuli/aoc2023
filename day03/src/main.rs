use common::AnyResult;

#[derive(Debug, Clone, Copy)]
struct Subsequence {
    line: usize,
    start: usize,
    length: usize,
}

impl<'a> Subsequence {
    pub fn as_str(&self, input: &'a str) -> &'a str {
        &input[self.start..self.start + self.length]
    }

    pub fn read(input: &str) -> Vec<Self> {
        let mut output = vec![];
        let mut seq: Option<Subsequence> = None;
        let mut line = 0;

        fn pop_seq(output: &mut Vec<Subsequence>, seq: &mut Option<Subsequence>) {
            match seq {
                Some(s) => {
                    output.push(*s);
                    *seq = None;
                }
                None => {}
            }
        }

        for (idx, ch) in input.chars().enumerate() {
            match ch {
                '\n' => {
                    pop_seq(&mut output, &mut seq);
                    line = line + 1;
                }

                '.' => pop_seq(&mut output, &mut seq),

                _ => {
                    seq = match seq {
                        Some(s) => Some(Subsequence {
                            length: s.length + 1,
                            ..s
                        }),
                        None => Some(Subsequence {
                            line,
                            start: idx,
                            length: 1,
                        }),
                    }
                }
            }
        }

        output
    }

    fn get_boundary_indices(&self, line_length: usize) {
        let mut boundaries = vec![];

        let line_start = (self.start / line_length) * line_length;
        let line_end = line_start + line_length;
        
        todo!()
    }
}

fn line_length(input: &str) -> Option<usize> {
    for (idx, ch) in input.chars().enumerate() {
        if ch == '\n' {
            return Some(idx);
        } else {
        }
    }

    None
}

fn main() -> AnyResult<()> {
    let input = common::read_stdin()?;
    let line_length = line_length(&input).expect("failed to find a newline character?");

    let _ = Subsequence::read(&input)
        .into_iter()
        .map(|x| x.as_str(&input))
        .for_each(|x| println!("{}", x));

    Ok(())
}
