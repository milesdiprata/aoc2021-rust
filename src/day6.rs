use std::str;

extern crate anyhow;

use aoc2021_rust::util;

const FISH_TIMER_RESET: usize = 6;
const FISH_TIMER_SPAWN: usize = 8;

#[derive(Clone)]
struct FishSchool {
    timers: Vec<usize>,
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
    fn simulate_day(&mut self) -> () {
        let mut num_resets = 0;

        self.timers = self
            .timers
            .iter()
            .map(|&timer| {
                if timer == 0 {
                    num_resets += 1;
                    FISH_TIMER_RESET
                } else {
                    timer - 1
                }
            })
            .collect::<Vec<_>>();

        (0..num_resets).for_each(|_| self.timers.push(FISH_TIMER_SPAWN));
    }
}

fn part_one(mut school: FishSchool) -> anyhow::Result<usize> {
    (0..80).for_each(|_| school.simulate_day());
    Ok(school.timers.len())
}

fn part_two(mut school: FishSchool) -> anyhow::Result<usize> {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let school = util::read_input::<FishSchool>()?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of initial states"))?;

    println!("Part one: {}", part_one(school.clone())?);
    println!("Part two: {}", part_two(school)?);

    Ok(())
}
