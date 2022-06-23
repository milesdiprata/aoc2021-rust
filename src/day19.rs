use core::fmt;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Point>,
}

impl fmt::Debug for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({},{})", self.x, self.y)
    }
}

impl FromStr for Point {
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

        Ok(Point { x, y })
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a> AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, other: &'a Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<'a> SubAssign<&'a Point> for Point {
    fn sub_assign(&mut self, other: &'a Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<'a> Sum<&'a Self> for Point {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, point| &acc + point)
    }
}

impl Scanner {
    const MAX_RNG: isize = 5;
    const MIN_COMMON_BEACONS: usize = 3;

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

    fn find_min_common_beacons(
        &self,
        other: &Self,
    ) -> Option<HashMap<usize, HashMap<usize, HashMap<Point, Point>>>> {
        let beacon_pairs = self
            .beacons
            .iter()
            .flat_map(|i| other.beacons.iter().map(move |j| (i, j)))
            .map(|(i, j)| (i.clone(), j.clone()))
            .collect::<Vec<_>>();

        (-Self::MAX_RNG..=Self::MAX_RNG)
            .flat_map(|x| (-Self::MAX_RNG..=Self::MAX_RNG).map(move |y| (x, y)))
            .map(|(x, y)| Point { x, y })
            .map(|offset| {
                beacon_pairs
                    .iter()
                    .filter(|(i, j)| &(i + &offset) == j)
                    .cloned()
                    .collect::<HashMap<_, _>>()
            })
            .find(|common| common.len() >= Self::MIN_COMMON_BEACONS)
            .map(|common| HashMap::from([(other.id, common)]))
            .map(|common| HashMap::from([(self.id, common)]))
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

fn part_one(scanners: &[Scanner]) -> usize {
    // Compute all possible pairs of scanners
    let scanner_pairs = scanners
        .iter()
        .flat_map(|i| scanners.iter().map(move |j| (i, j)))
        .filter(|&(i, j)| i.id != j.id)
        .collect::<Vec<_>>();

    // Find pairs of scanners that have 12 common beacons
    // TODO: Account for rotations about Euclidean space
    let scanner_common_beacons = scanner_pairs
        .iter()
        .flat_map(|&(i, j)| i.find_min_common_beacons(j))
        .fold(HashMap::new(), |acc, common| {
            acc.into_iter().chain(common.into_iter()).collect()
        });

    // Determine relative sensor positions to each other
    let scanner_rel_pos = scanner_common_beacons
        .iter()
        .map(|(&id, common)| {
            (
                id,
                common
                    .iter()
                    .map(|(&id, common)| (id, common.iter().next().map(|(i, j)| i - j).unwrap()))
                    .collect::<HashMap<_, _>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    // Determine 'absolute' sensor positions to 0th sensor
    let scanner_abs_pos = scanner_rel_pos
        .iter()
        .filter(|(&id, _)| id != 0)
        .map(|(&id, common)| (id, common.values().sum::<Point>()))
        .collect::<HashMap<_, _>>();

    println!("abs {:?}", scanner_abs_pos);

    // Determine 'absolute' beacon positions to 0th sensor
    let mut scanners = scanners.iter();

    let beacons = scanners
        .next()
        .unwrap()
        .beacons
        .iter()
        .cloned()
        .chain(
            scanners
                .map(|scanner| {
                    scanner
                        .beacons
                        .iter()
                        .map(|beacon| beacon - &scanner_abs_pos[&scanner.id])
                })
                .flatten(),
        )
        .collect::<HashSet<_>>();

    println!("beacons {:?}", beacons);

    beacons.len()
}

fn main() -> Result<()> {
    let mut scanners = read_scanners()?;

    println!("Part one: {}", part_one(&scanners));

    Ok(())
}
