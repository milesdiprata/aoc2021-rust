use std::str;

extern crate anyhow;
extern crate crossbeam;
extern crate num_cpus;

use aoc2021_rust::util;

const FISH_TIMER_RESET: u8 = 6;
const FISH_TIMER_SPAWN: u8 = 8;

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
    fn simulate_day(&mut self) -> () {
        let mut num_resets = 0usize;

        self.timers.iter_mut().for_each(|timer| {
            if *timer == 0 {
                *timer = FISH_TIMER_RESET;
                num_resets += 1;
            } else {
                *timer -= 1;
            }
        });

        (0..num_resets).for_each(|_| self.timers.push(FISH_TIMER_SPAWN));
    }
}

fn part_one(mut school: FishSchool) -> anyhow::Result<usize> {
    (0..80).for_each(|_| school.simulate_day());
    Ok(school.timers.len())
}

fn part_two(school: FishSchool) -> anyhow::Result<usize> {
    Ok(crossbeam::scope(|scope| {
        let threads = school
            .timers
            .chunks(num_cpus::get())
            .map(|timers| timers.to_vec())
            .map(|timers| FishSchool { timers })
            .map(|mut school| {
                scope.spawn(move |_| {
                    (0..256).for_each(|_| school.simulate_day());
                    school.timers.len()
                })
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .map(|thread| thread.join())
            .collect::<Result<Vec<_>, _>>()
    })
    .map_err(|err| anyhow::anyhow!("{:?}", err))?
    .map_err(|err| anyhow::anyhow!("{:?}", err))?
    .iter()
    .sum::<usize>())
}

fn main() -> anyhow::Result<()> {
    let school = util::read_input::<FishSchool>()?
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty set of initial states!"))?;

    println!("Part one: {}", part_one(school.clone())?);
    println!("Part two: {}", part_two(school)?);

    Ok(())
}
