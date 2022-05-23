use std::env;
use std::str;

extern crate anyhow;

struct Crabs(Vec<isize>);

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

fn part_one(crabs: &Crabs) -> Option<usize> {
    let max_pos = *crabs.0.iter().max()?;

    (0..=max_pos)
        .map(|pos| crabs.0.iter().map(|crab| (pos - crab).abs() as usize).sum())
        .min()
}

fn part_two(crabs: &Crabs) -> anyhow::Result<usize> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let crabs = env::args()
        .collect::<Vec<_>>()
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of crab positions!"))?
        .parse::<Crabs>()?;

    println!(
        "Part one: {}",
        part_one(&crabs).ok_or_else(|| anyhow::anyhow!("No answer found!"))?
    );

    // println!("Part two: {}", part_two(&crabs)?);

    Ok(())
}
