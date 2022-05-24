use std::collections;
use std::io::{self, BufRead};
use std::str;

extern crate anyhow;

extern crate aoc2021_rust;

use aoc2021_rust::util;

const SIGNAL_LEN: usize = 7;
const DIGS_LEN: usize = 10;

const SEVEN_SEG_DIGS: [[u8; SIGNAL_LEN]; DIGS_LEN] = [
    // a, b, c, d, e, f, g
    [1, 1, 1, 0, 1, 1, 1], // 0
    [0, 0, 1, 0, 0, 1, 0], // 1
    [1, 0, 1, 1, 1, 0, 1], // 2
    [1, 0, 1, 1, 0, 1, 1], // 3
    [0, 1, 1, 1, 0, 1, 0], // 4
    [1, 1, 0, 1, 0, 1, 1], // 5
    [1, 1, 0, 1, 1, 1, 1], // 6
    [1, 0, 1, 0, 0, 1, 0], // 7
    [1, 1, 1, 1, 1, 1, 1], // 8
    [1, 1, 1, 1, 0, 1, 1], // 9
];

struct Sig {
    patterns: [String; 10],
    output: [String; 4],
}

impl str::FromStr for Sig {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let input = input.split(" | ").collect::<Vec<_>>();

        if input.len() != 2 {
            return Err(anyhow::anyhow!("Invalid signal sequence!"));
        }

        let patterns = input
            .first()
            .ok_or_else(|| anyhow::anyhow!("Invalid unique patterns!"))?
            .split(' ')
            .map(|value| value.to_owned())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Length of patterns not equal to 10!"))?;

        let output = input
            .last()
            .ok_or_else(|| anyhow::anyhow!("Invalid output values!"))?
            .split(' ')
            .map(|value| value.to_owned())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Length of output not equal to 4!"))?;

        Ok(Sig { patterns, output })
    }
}

fn sig_len(sig: usize) -> Option<usize> {
    if sig >= DIGS_LEN {
        return None;
    }

    Some(
        SEVEN_SEG_DIGS[sig]
            .iter()
            .filter(|&&sig| sig == 1)
            .collect::<Vec<_>>()
            .len(),
    )
}

fn part_one(sigs: &[Sig]) -> Option<usize> {
    let unique_dig_sig_lens =
        collections::HashSet::from([sig_len(1)?, sig_len(4)?, sig_len(7)?, sig_len(8)?]);

    let num_unique_digs = sigs
        .iter()
        .flat_map(|sig| {
            sig.output
                .iter()
                .filter(|&output| unique_dig_sig_lens.contains(&output.len()))
        })
        .collect::<Vec<_>>()
        .len();

    Some(num_unique_digs)
}

fn part_two(sigs: &[Sig]) -> usize {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let sigs = util::read_input::<Sig>()?;

    println!(
        "Part one: {}",
        part_one(&sigs).ok_or_else(|| anyhow::anyhow!("No answer found!"))?
    );

    Ok(())
}
