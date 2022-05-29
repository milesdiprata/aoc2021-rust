use std::io;
use std::io::BufRead;
use std::str;

extern crate anyhow;

pub fn read_input<T: str::FromStr>() -> anyhow::Result<Vec<T>> {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let mut input = vec![];

    while let Some(Ok(Ok(line))) = lines.next().map(|line| line.map(|line| line.parse::<T>())) {
        input.push(line);
    }

    Ok(input)
}
