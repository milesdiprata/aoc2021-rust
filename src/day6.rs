use std::str;

extern crate anyhow;

use aoc2021_rust::util;

const FISH_TIMER_RESET: usize = 6;
const FISH_TIMER_SPAWN: usize = 8;

#[derive(Clone)]
struct FishSchool {
    timers: Vec<u8>,
}

impl str::FromStr for FishSchool {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            timers: input
                .split(',')
                .map(|timer| timer.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FishSchool {
    fn simulate(&self, days: usize) -> usize {
        self.timers
            .iter()
            .map(|&timer| timer as usize)
            .map(|timer| Self::simulate_one(days, timer))
            .sum()
    }

    fn simulate_one(days: usize, timer: usize) -> usize {
        if days <= timer {
            return 1;
        }

        let days = days - timer - 1;

        Self::simulate_one(days, FISH_TIMER_RESET) + Self::simulate_one(days, FISH_TIMER_SPAWN)
    }
}

fn part_one(school: &FishSchool) -> anyhow::Result<usize> {
    Ok(school.simulate(80))
}

fn part_two(school: &FishSchool) -> anyhow::Result<usize> {
    Ok(school.simulate(256))
}

fn main() -> anyhow::Result<()> {
    let school = util::read_input::<FishSchool>()?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of initial states!"))?;

    println!("Part one: {}", part_one(&school)?);
    println!("Part two: {}", part_two(&school)?);

    Ok(())
}
