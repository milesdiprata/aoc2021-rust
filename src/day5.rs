use std::cmp;
use std::collections;
use std::fmt;
use std::str;

extern crate anyhow;

use aoc2021_rust::util;

struct Line {
    x: (isize, isize),
    y: (isize, isize),
}

impl str::FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let dirs = input
            .split(" -> ")
            .flat_map(|pair| pair.split(',').map(|dir| dir.parse()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Line {
            x: (dirs[0], dirs[2]),
            y: (dirs[1], dirs[3]),
        })
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({},{}) -> ({},{})",
            self.x.0, self.y.0, self.x.1, self.y.1
        )
    }
}

impl cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Line {
    fn to_coords(&self, parse_diagonal: bool) -> Option<Vec<(isize, isize)>> {
        if self.x.0 == self.x.1 {
            Some(
                (cmp::min(self.y.0, self.y.1)..=cmp::max(self.y.0, self.y.1))
                    .map(|y| (self.x.0, y))
                    .collect(),
            )
        } else if self.y.0 == self.y.1 {
            Some(
                (cmp::min(self.x.0, self.x.1)..=cmp::max(self.x.0, self.x.1))
                    .map(|x| (x, self.y.0))
                    .collect(),
            )
        } else if parse_diagonal && ((self.y.1 - self.y.0) / (self.x.1 - self.x.0)).abs() == 1 {
            Some(
                Self::get_diagonal_coords((self.x.0, self.x.1))
                    .into_iter()
                    .zip(Self::get_diagonal_coords((self.y.0, self.y.1)).into_iter())
                    .collect(),
            )
        } else {
            None
        }
    }

    fn get_diagonal_coords(dir: (isize, isize)) -> Vec<isize> {
        let is_increasing = dir.0 < dir.1;

        (0..=(dir.1 - dir.0).abs())
            .map(|i| if is_increasing { dir.0 + i } else { dir.0 - i })
            .collect()
    }
}

fn part_one(lines: &[Line]) -> anyhow::Result<usize> {
    let mut coord_count = collections::HashMap::<(isize, isize), usize>::new();

    lines
        .iter()
        .flat_map(|line| line.to_coords(false))
        .flatten()
        .for_each(|coord| *coord_count.entry(coord).or_insert(0) += 1);

    Ok(coord_count
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .collect::<Vec<_>>()
        .len())
}

fn part_two(lines: &[Line]) -> anyhow::Result<usize> {
    let mut coord_count = collections::HashMap::<(isize, isize), usize>::new();

    lines
        .iter()
        .flat_map(|line| line.to_coords(true))
        .flatten()
        .for_each(|coord| *coord_count.entry(coord).or_insert(0) += 1);

    Ok(coord_count
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .collect::<Vec<_>>()
        .len())
}

fn main() -> anyhow::Result<()> {
    let lines = util::read_input::<Line>()?;

    println!("Part one {}", part_one(&lines)?);
    println!("Part two {}", part_two(&lines)?);

    Ok(())
}
