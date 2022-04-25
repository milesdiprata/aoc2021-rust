use std::str;

extern crate anyhow;

use aoc2021_rust::util;

struct Crabs(Vec<usize>);

impl str::FromStr for Crabs {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        Ok(Crabs(
            input
                .split(',')
                .map(|pos| pos.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn part_one(crabs: &Crabs) -> anyhow::Result<usize> {
    todo!()
}

fn part_two(crabs: &Crabs) -> anyhow::Result<usize> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let crabs = util::read_input::<Crabs>()?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of crab positions!"))?;

    println!("Part one: {}", part_one(&crabs)?);
    println!("Part two: {}", part_two(&crabs)?);

    Ok(())
}
