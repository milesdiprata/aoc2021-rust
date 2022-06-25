use core::fmt;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone)]
struct Rot {
    i: [isize; Self::NUM_DIM],
    j: [isize; Self::NUM_DIM],
    k: [isize; Self::NUM_DIM],
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Point>,
}

struct ScannerContext {
    id: usize,
    pos: Point,
    rot: Rot,
}

impl Default for Rot {
    fn default() -> Self {
        Self {
            i: [1, 0, 0],
            j: [0, 1, 0],
            k: [0, 0, 1],
        }
    }
}

impl fmt::Debug for Rot {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[{:?},{:?},{:?}]", self.i, self.j, self.k)
    }
}

impl Rot {
    const NUM_DIM: usize = 3;
    const NUM_ORIENTATIONS: usize = 24;

    fn from_theta(theta_x: f32, theta_y: f32, theta_z: f32) -> Self {
        let (theta_x, theta_y, theta_z) = (
            theta_x.to_radians(),
            theta_y.to_radians(),
            theta_z.to_radians(),
        );

        let (cos_x, cos_y, cos_z) = (
            theta_x.cos().round() as isize,
            theta_y.cos().round() as isize,
            theta_z.cos().round() as isize,
        );

        let (sin_x, sin_y, sin_z) = (
            theta_x.sin().round() as isize,
            theta_y.sin().round() as isize,
            theta_z.sin().round() as isize,
        );

        Self {
            i: [
                cos_y * cos_z,
                -cos_y * sin_z * cos_x + sin_y * sin_x,
                cos_y * sin_z * sin_x + sin_y * cos_x,
            ],
            j: [sin_z, cos_z * cos_x, -cos_z * sin_x],
            k: [
                -sin_y * cos_z,
                sin_y * sin_z * cos_x + cos_y * sin_x,
                -sin_y * sin_z * sin_x + cos_y * cos_x,
            ],
        }
    }

    fn all_possible() -> [Self; Self::NUM_ORIENTATIONS] {
        [
            Rot::from_theta(0.0, 0.0, 0.0),
            Rot::from_theta(0.0, 90.0, 0.0),
            Rot::from_theta(0.0, 180.0, 0.0),
            Rot::from_theta(0.0, -90.0, 0.0),
            Rot::from_theta(0.0, 0.0, 90.0),
            Rot::from_theta(0.0, 90.0, 90.0),
            Rot::from_theta(0.0, 180.0, 90.0),
            Rot::from_theta(0.0, -90.0, 90.0),
            Rot::from_theta(0.0, 0.0, -90.0),
            Rot::from_theta(0.0, 90.0, -90.0),
            Rot::from_theta(0.0, 180.0, -90.0),
            Rot::from_theta(0.0, -90.0, -90.0),
            Rot::from_theta(90.0, 0.0, 0.0),
            Rot::from_theta(90.0, 90.0, 0.0),
            Rot::from_theta(90.0, 180.0, 0.0),
            Rot::from_theta(90.0, -90.0, 0.0),
            Rot::from_theta(180.0, 0.0, 0.0),
            Rot::from_theta(180.0, 90.0, 0.0),
            Rot::from_theta(180.0, 180.0, 0.0),
            Rot::from_theta(180.0, -90.0, 0.0),
            Rot::from_theta(-90.0, 0.0, 0.0),
            Rot::from_theta(-90.0, 90.0, 0.0),
            Rot::from_theta(-90.0, 180.0, 0.0),
            Rot::from_theta(-90.0, -90.0, 0.0),
        ]
    }
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a> AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, other: &'a Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a> SubAssign<&'a Point> for Point {
    fn sub_assign(&mut self, other: &'a Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a> Sum<&'a Self> for Point {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, point| &acc + point)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        let mut split = str.split(',');

        let (x, y, z) = (
            split
                .next()
                .ok_or_else(|| anyhow!("Empty beacon x-coord!"))?
                .parse()?,
            split
                .next()
                .ok_or_else(|| anyhow!("Empty beacon y-coord!"))?
                .parse()?,
            split
                .next()
                .ok_or_else(|| anyhow!("Empty beacon z-coord!"))?
                .parse()?,
        );

        Ok(Point { x, y, z })
    }
}

impl Point {
    fn to_rotated(&self, rot: &Rot) -> Self {
        let x = [self.x, self.y, self.z]
            .iter()
            .zip(rot.i.iter())
            .map(|(x, i)| x * i)
            .sum();

        let y = [self.x, self.y, self.z]
            .iter()
            .zip(rot.j.iter())
            .map(|(y, j)| y * j)
            .sum();

        let z = [self.x, self.y, self.z]
            .iter()
            .zip(rot.k.iter())
            .map(|(z, j)| z * j)
            .sum();

        Self { x, y, z }
    }

    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }

    fn manhattan_dist(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Scanner {
    const MAX_RNG: isize = 1000;
    const MIN_COMMON_BEACONS: usize = 12;

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

        let beacons = lines
            .map(|&line| line.parse())
            .collect::<Result<Vec<_>>>()?;

        match beacons.is_empty() {
            true => Err(anyhow!("Empty scanner readings!")),
            false => Ok(Self { id, beacons }),
        }
    }

    fn has_min_common_beacons(&self, other: &Self, offset: &Point, rot: &Rot) -> bool {
        self.beacons
            .iter()
            .flat_map(|i| other.beacons.iter().map(move |j| (i, j)))
            .filter(|&(i, j)| &(&i.to_rotated(rot) + offset) == j)
            .collect::<Vec<_>>()
            .len()
            >= 3
    }
}

impl Default for ScannerContext {
    fn default() -> Self {
        Self {
            id: Default::default(),
            pos: Default::default(),
            rot: Default::default(),
        }
    }
}

impl ScannerContext {
    const SENTINEL_ID: usize = 0;

    fn from_scanners(id: usize, scanners: &[Scanner]) -> Self {
        Self::default().with_id(id).with_scanners(scanners)
    }

    fn with_id(self, id: usize) -> Self {
        Self {
            id,
            pos: self.pos,
            rot: self.rot,
        }
    }

    fn with_scanners(self, scanners: &[Scanner]) -> Self {
        if self.id == Self::SENTINEL_ID {
            return self;
        }

        let pairs = self.scanner_pairs(scanners);
        println!("pairs for {}: {:?}", self.id, pairs);

        let rel_vecs = Self::rel_vecs(&pairs);
        println!("rel_vecs for {}: {:?}", self.id, rel_vecs);

        let (abs_pos, abs_rot) = Self::abs_vec(&scanners[Self::SENTINEL_ID], &rel_vecs);

        todo!()
    }

    fn scanner_pairs<'a>(&'a self, scanners: &'a [Scanner]) -> Vec<(&'a Scanner, &'a Scanner)> {
        scanners
            .iter()
            .filter(|&scanner| self.id != scanner.id)
            .map(|scanner| (&scanners[self.id], scanner))
            .collect()
    }

    fn rel_vecs(pairs: &[(&Scanner, &Scanner)]) -> HashMap<usize, (Point, Rot)> {
        let vecs = (0..=Scanner::MAX_RNG)
            .flat_map(|x| {
                (0..=Scanner::MAX_RNG)
                    .flat_map(move |y| (0..=Scanner::MAX_RNG).map(move |z| (x, y, z)))
            })
            .map(|offset| Point::from(offset))
            .flat_map(|offset| Rot::all_possible().map(|rot| (offset.clone(), rot)))
            .collect::<Vec<_>>();

        let a = pairs
            .iter()
            .flat_map(|&(i, j)| vecs.iter().map(move |(offset, rot)| (i, j, offset, rot)))
            .filter(|&(i, j, offset, rot)| i.has_min_common_beacons(j, offset, rot))
            .map(|(_, j, offset, rot)| (j.id, (offset, rot)))
            .collect::<HashMap<_, _>>();

        println!("{:?}", a);

        todo!()
    }

    fn abs_vec(sentinel: &Scanner, rel_vecs: &HashMap<usize, (Point, Rot)>) -> (Point, Rot) {
        todo!()
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
    let contexts = scanners
        .iter()
        .map(|scanner| ScannerContext::from_scanners(scanner.id, scanners))
        .collect::<Vec<_>>();

    todo!()
}

fn main() -> Result<()> {
    let mut scanners = read_scanners()?;

    part_one(&scanners);

    Ok(())
}
