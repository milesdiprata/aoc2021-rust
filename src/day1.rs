extern crate anyhow;

use aoc2021_rust::util;

const PART_ONE_WINDOW_LEN: usize = 2;
const PART_TWO_WINDOW_LEN: usize = 3;

fn part_one(depths: &[isize]) -> anyhow::Result<usize> {
    if depths.is_empty() {
        return Err(anyhow::anyhow!("No depth readings given!"));
    }

    if depths.len() < PART_ONE_WINDOW_LEN {
        return Ok(0);
    }

    Ok(depths
        .windows(PART_ONE_WINDOW_LEN)
        .filter(|&window| window[0] < window[1])
        .collect::<Vec<_>>()
        .len())
}

fn part_two(depths: &[isize]) -> anyhow::Result<usize> {
    if depths.is_empty() {
        return Err(anyhow::anyhow!("No depth readings given!"));
    }

    if depths.len() < PART_TWO_WINDOW_LEN * PART_ONE_WINDOW_LEN {
        return Ok(0);
    }

    Ok(depths
        .windows(PART_TWO_WINDOW_LEN)
        .map(|window| window.iter().sum::<isize>())
        .collect::<Vec<_>>()
        .windows(PART_ONE_WINDOW_LEN)
        .filter(|&window| window[0] < window[1])
        .collect::<Vec<_>>()
        .len())
}

fn main() -> anyhow::Result<()> {
    let input = util::read_input::<isize>()?;

    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);

    Ok(())
}
