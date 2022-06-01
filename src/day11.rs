use std::io::{self, BufRead};

extern crate anyhow;

const GRID_LEN: usize = 10;

fn read_grid() -> anyhow::Result<Vec<Vec<u8>>> {
    io::stdin()
        .lock()
        .lines()
        .take(GRID_LEN)
        .map(|line| {
            line.map(|line| {
                line.chars()
                    .take(GRID_LEN)
                    .map(|c| {
                        c.to_digit(10)
                            .map(|octopus| octopus as u8)
                            .ok_or_else(|| anyhow::anyhow!("Failed to parse digit!"))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
        })
        .collect::<Result<Result<Vec<_>, _>, _>>()?
}

fn get_adj_octopuses((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .map(|(i, j)| (x as isize + i, y as isize + j))
        .filter(|&(i, j)| i >= 0 && i < GRID_LEN as isize && j >= 0 && j < GRID_LEN as isize)
        .map(|(i, j)| (i as usize, j as usize))
        .filter(|&point| point != (x, y))
        .collect()
}

fn run_step(octopuses: &mut [Vec<u8>]) -> usize {
    octopuses.iter_mut().for_each(|octopuses| {
        octopuses.iter_mut().for_each(|octopus| {
            *octopus += 1;

            if *octopus > 9 {
                *octopus = 0;
            }
        })
    });

    let mut flashes = (0..GRID_LEN)
        .flat_map(|i| (0..GRID_LEN).map(move |j| (i, j)))
        .filter(|&(i, j)| octopuses[i][j] == 0)
        .collect::<Vec<_>>();

    let mut num_flashes = 0;

    while let Some((i, j)) = flashes.pop() {
        num_flashes += 1;

        get_adj_octopuses((i, j)).iter().for_each(|&(i, j)| {
            if octopuses[i][j] != 0 && octopuses[i][j] <= 9 {
                octopuses[i][j] += 1;
            }

            if octopuses[i][j] > 9 {
                octopuses[i][j] = 0;
                flashes.push((i, j));
            }
        });
    }

    num_flashes
}

fn part_one(mut octopuses: Vec<Vec<u8>>) -> usize {
    (0..100).map(|_| run_step(&mut octopuses)).sum()
}

fn main() -> anyhow::Result<()> {
    let octopuses = read_grid()?;

    println!("Part one: {}", part_one(octopuses));

    Ok(())
}
