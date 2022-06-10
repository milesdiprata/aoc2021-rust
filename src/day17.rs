extern crate anyhow;

use std::cmp;
use std::io::{self, BufRead};
use std::ops;

struct Area {
    x_rng: ops::RangeInclusive<isize>,
    y_rng: ops::RangeInclusive<isize>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Area {
    fn read() -> anyhow::Result<Self> {
        let line = io::stdin()
            .lock()
            .lines()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing target area input!"))??;

        let mut split = line["target area: ".len()..].split(", ");

        let mut x_split = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing x-direction target range!"))?["x=".len()..]
            .split("..");

        let mut y_split = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing y-direction target range!"))?["y=".len()..]
            .split("..");

        let x_rng = x_split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing x-direction target range lower bound!"))?
            .parse()?
            ..=x_split
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing x-direction target range upper bound!"))?
                .parse()?;

        let y_rng = y_split
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing y-direction target range lower bound!"))?
            .parse()?
            ..=y_split
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing y-direction target range upper bound!"))?
                .parse()?;

        let (x_min, x_max) = (x_rng.clone().min().unwrap(), x_rng.clone().max().unwrap());
        let (y_min, y_max) = (y_rng.clone().min().unwrap(), y_rng.clone().max().unwrap());

        Ok(Area {
            x_rng,
            y_rng,
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

fn trick_shot(mut vel: (isize, isize), area: &Area) -> bool {
    let mut pos = (0, 0);

    while !area.x_rng.contains(&pos.0) || !area.y_rng.contains(&pos.1) {
        if pos.0 > area.x_max
            || (vel.0 == 0 && (!area.x_rng.contains(&pos.0) || pos.1 < area.y_min))
        {
            return false;
        }

        pos.0 += vel.0;
        pos.1 += vel.1;

        if vel.0 > 0 {
            vel.0 -= 1;
        } else if vel.0 < 0 {
            vel.0 += 1;
        }

        vel.1 -= 1;
    }

    true
}

fn part_one(area: &Area) -> Option<isize> {
    let y_vel = area.y_min.abs() - 1;
    let y_max = y_vel * (y_vel + 1) / 2;

    Some(y_max)
}

fn part_two(area: &Area) -> usize {
    let y_abs_max = cmp::max(area.y_min.abs(), area.y_max.abs());

    (0..=area.x_max)
        .flat_map(|x_vel| (-y_abs_max..=y_abs_max).map(move |y_vel| (x_vel, y_vel)))
        .filter(|&vel| trick_shot(vel, area))
        .collect::<Vec<_>>()
        .len()
}

fn main() -> anyhow::Result<()> {
    let area = Area::read()?;

    println!(
        "Part one: {}",
        part_one(&area).ok_or_else(|| anyhow::anyhow!("No answer found!"))?
    );

    println!("Part two: {}", part_two(&area));

    Ok(())
}
