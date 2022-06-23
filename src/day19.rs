use core::fmt;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::{Add, Sub};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

const SENSOR_MAX_RNG: isize = 5;
const MIN_COMMON_BEACONS: usize = 3;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Beacon {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

impl fmt::Debug for Beacon {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({},{})", self.x, self.y)
    }
}

impl FromStr for Beacon {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut split = str.split(',');

        let (x, y) = (
            split
                .next()
                .ok_or_else(|| anyhow!("Empty beacon x-coord!"))?
                .parse()?,
            split
                .next()
                .ok_or_else(|| anyhow!("Empty beacon y-coord!"))?
                .parse()?,
        );

        Ok(Beacon { x, y })
    }
}

impl<'a, 'b> Add<&'b Beacon> for &'a Beacon {
    type Output = Beacon;

    fn add(self, other: &'b Beacon) -> Beacon {
        Beacon {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Beacon> for &'a Beacon {
    type Output = Beacon;

    fn sub(self, other: &'b Beacon) -> Beacon {
        Beacon {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Scanner {
    fn from_str_slice(slice: &[&str]) -> Result<Self> {
        let mut lines = slice.iter();

        let id = lines
            .next()
            .ok_or_else(|| anyhow!("Empty scanner ID input!"))?
            .split(' ')
            .skip(2)
            .next()
            .ok_or_else(|| anyhow!("Empty scanner ID!"))?
            .parse::<usize>()?;

        let beacons = lines.map(|line| line.parse()).collect::<Result<Vec<_>>>()?;

        match beacons.is_empty() {
            true => Err(anyhow!("Empty scanner readings!")),
            false => Ok(Self { id, beacons }),
        }
    }
}

fn read_from_stdin() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = vec![];
    let mut is_empty = false;

    while let Some(line) = lines.next() {
        let line = line?;

        if line.is_empty() && is_empty {
            break;
        } else if line.is_empty() {
            is_empty = true;
        } else {
            is_empty = false;
        }

        input.push(line);
    }

    Ok(input)
}

fn into_scanner_inputs(input: Vec<String>) -> Vec<Vec<String>> {
    let mut input = input.into_iter();
    let mut scanner_inputs = vec![];
    let mut scanner_input = vec![];

    while let Some(line) = input.next() {
        if line.is_empty() {
            scanner_inputs.push(scanner_input);
            scanner_input = vec![];
        } else {
            scanner_input.push(line);
        }
    }

    scanner_inputs
}

fn read_scanners() -> Result<Vec<Scanner>> {
    let scanners = into_scanner_inputs(read_from_stdin()?)
        .iter()
        .map(|input| input.iter())
        .map(|input| input.map(|line| line.as_str()))
        .map(|input| input.collect::<Vec<_>>())
        .map(|input| Scanner::from_str_slice(&input))
        .collect::<Result<Vec<_>>>()?;

    Ok(scanners)
}

fn find_min_common_beacons(i: &Scanner, j: &Scanner) -> Option<HashMap<Beacon, Beacon>> {
    let beacon_pairs = i
        .beacons
        .iter()
        .flat_map(|i| j.beacons.iter().map(move |j| (i, j)))
        .map(|(i, j)| (i.clone(), j.clone()))
        .collect::<Vec<_>>();

    (-SENSOR_MAX_RNG..=SENSOR_MAX_RNG)
        .flat_map(|x| (-SENSOR_MAX_RNG..=SENSOR_MAX_RNG).map(move |y| (x, y)))
        .map(|(x, y)| Beacon { x, y })
        .map(|offset| {
            beacon_pairs
                .iter()
                .filter(|(i, j)| &(i + &offset) == j)
                .map(|(i, j)| (i.clone(), j.clone()))
                .collect::<HashMap<_, _>>()
        })
        .find(|common| common.len() >= MIN_COMMON_BEACONS)
}

fn part_one(scanners: &[Scanner]) -> () {
    // Compute all possible pairs of scanners
    let scanner_pairs = scanners
        .iter()
        .flat_map(|i| scanners.iter().map(move |j| (i, j)))
        .filter(|&(i, j)| i.id != j.id)
        .collect::<Vec<_>>();

    // Find pairs of scanners that have 12 common beacons
    let scanner_common_beacons = scanner_pairs
        .iter()
        .map(|&(i, j)| (i, j, find_min_common_beacons(i, j)))
        .filter(|(_, _, common)| common.is_some())
        .map(|(i, j, common)| (i.id, j.id, common.unwrap()))
        .map(|(i, j, common)| (i, HashMap::from([(j, common)])))
        .collect::<HashMap<_, _>>();

    // Reconstruct beacon map one sensor at a time
    let scanner_offsets = todo!();
    todo!();

    // Express absolute beacon positions relative to scanner 0 with its orientation and coords at (0,0,0)
    todo!();

    todo!()
}

fn main() -> Result<()> {
    let mut scanners = read_scanners()?;

    // let pairs = scanners
    //     .iter()
    //     .flat_map(|i| scanners.iter().map(move |j| (i, j)))
    //     .collect::<Vec<_>>();

    part_one(&scanners);

    Ok(())
}
