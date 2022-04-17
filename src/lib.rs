#![allow(dead_code)]

use std::io;
use std::io::BufRead;
use std::str;

extern crate anyhow;

mod day1;

pub fn read_input<T: str::FromStr>() -> anyhow::Result<Vec<T>> {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .into_iter()
        .map(|line| line.map(|line| line.parse()))
        .collect::<Result<Result<Vec<_>, _>, _>>()?
        .map_err(|_| anyhow::anyhow!("Failed to parse line!"))
}
