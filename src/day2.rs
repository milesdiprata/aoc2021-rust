use std::str;

extern crate anyhow;

use aoc2021_rust::util;

const FORWARD: &str = "forward";
const UP: &str = "up";
const DOWN: &str = "down";

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    units: isize,
}

#[derive(Debug)]
struct Position {
    pos: isize,
    depth: isize,
    aim: isize,
}

impl str::FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        match input.to_lowercase().as_str() {
            FORWARD => Ok(Self::Forward),
            UP => Ok(Self::Up),
            DOWN => Ok(Self::Down),
            _ => Err(anyhow::anyhow!("Unknown direction '{}'!", input)),
        }
    }
}

impl str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let split = input.split(' ').collect::<Vec<_>>();
        Ok(Command {
            direction: split
                .first()
                .map(|&direction| direction.parse::<Direction>())
                .ok_or_else(|| anyhow::anyhow!("Missing command!"))??,
            units: split
                .last()
                .map(|&units| units.parse())
                .ok_or_else(|| anyhow::anyhow!("Missing units!"))??,
        })
    }
}

impl Position {
    fn new() -> Self {
        Self {
            pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn update(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => self.pos += command.units,
            Direction::Up => self.depth -= command.units,
            Direction::Down => self.depth += command.units,
        }
    }

    fn update_with_aim(&mut self, command: &Command) {
        match command.direction {
            Direction::Forward => {
                self.pos += command.units;
                self.depth += self.aim * command.units;
            }
            Direction::Up => self.aim -= command.units,
            Direction::Down => self.aim += command.units,
        }
    }

    fn result(&self) -> isize {
        self.pos * self.depth
    }
}

fn part_one(commands: &[Command]) -> anyhow::Result<isize> {
    let mut pos = Position::new();
    commands.iter().for_each(|command| pos.update(command));

    Ok(pos.result())
}

fn part_two(commands: &[Command]) -> anyhow::Result<isize> {
    let mut pos = Position::new();
    commands
        .iter()
        .for_each(|command| pos.update_with_aim(command));

    Ok(pos.result())
}

fn main() -> anyhow::Result<()> {
    let input = util::read_input::<Command>()?;

    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);

    Ok(())
}
