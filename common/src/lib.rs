use std::io::Read;

pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_to_string(&mut buffer).map(|_| buffer)
}
