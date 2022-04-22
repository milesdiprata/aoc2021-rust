use std::collections;
use std::str;

extern crate anyhow;

use aoc2021_rust::util;

const FISH_TIMER_RESET: u8 = 6;
const FISH_TIMER_SPAWN: u8 = 8;

#[derive(Clone)]
struct FishSchool {
    num_timers: collections::HashMap<u8, usize>,
}

impl str::FromStr for FishSchool {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let timers = input
            .split(',')
            .map(|timer| timer.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            num_timers: (0..=FISH_TIMER_SPAWN)
                .map(|state| {
                    (
                        state,
                        timers
                            .iter()
                            .filter(|&&timer| timer == state)
                            .collect::<Vec<_>>()
                            .len(),
                    )
                })
                .collect(),
        })
    }
}

impl FishSchool {
    fn simulate(&mut self, days: usize) -> usize {
        (0..days).for_each(|_| self.simulate_day());

        self.num_timers.values().sum()
    }

    fn simulate_day(&mut self) -> () {
        let mut num_timers = collections::HashMap::new();

        self.num_timers.iter().for_each(|(&state, &num_timer)| {
            if state == 0 {
                *num_timers.entry(FISH_TIMER_RESET).or_insert(0) += num_timer;
                *num_timers.entry(FISH_TIMER_SPAWN).or_insert(0) += num_timer;
            } else {
                *num_timers.entry(state - 1).or_insert(0) += num_timer;
            }
        });

        self.num_timers = num_timers;
    }
}

fn part_one(mut school: FishSchool) -> anyhow::Result<usize> {
    Ok(school.simulate(80))
}

fn part_two(mut school: FishSchool) -> anyhow::Result<usize> {
    Ok(school.simulate(256))
}

fn main() -> anyhow::Result<()> {
    let school = util::read_input::<FishSchool>()?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of initial states!"))?;

    println!("Part one: {}", part_one(school.clone())?);
    println!("Part two: {}", part_two(school)?);

    Ok(())
}
