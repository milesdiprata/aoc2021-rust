extern crate anyhow;

use std::io::{self, BufRead};
use std::ops;

struct Area {
    x_rng: ops::RangeInclusive<isize>,
    y_rng: ops::RangeInclusive<isize>,
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

        Ok(Area { x_rng, y_rng })
    }
}

fn trick_shot(mut vel: (isize, isize), tgt_area: &Area) -> bool {
    let mut pos = (0, 0);

    while !tgt_area.x_rng.contains(&pos.0) || !tgt_area.y_rng.contains(&pos.1) {
        pos.0 += vel.0;
        pos.1 += vel.1;

        if vel.0 > 0 {
            vel.0 -= 1;
        } else if vel.0 < 0 {
            vel.0 += 1;
        }

        vel.1 -= 1;

        if pos.1 < *tgt_area.y_rng.start() {
            return false;
        }
    }

    true
}

fn part_one(area: &Area) -> Option<isize> {
    let y_tgt_min = area.y_rng.clone().min()?;
    let y_vel = y_tgt_min.abs() - 1;
    let y_max = y_vel * (y_vel + 1) / 2;

    Some(y_max)
}

fn main() -> anyhow::Result<()> {
    let area = Area::read()?;

    println!(
        "{}",
        part_one(&area).ok_or_else(|| anyhow::anyhow!("No answer found!"))?
    );

    Ok(())
}
