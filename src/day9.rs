use std::collections;
use std::io;
use std::io::BufRead;

extern crate anyhow;

fn read_heights() -> anyhow::Result<Vec<Vec<u32>>> {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let mut heights = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        heights.push(
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("Failed to parse digit!"))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    Ok(heights)
}

fn get_adj_heights(height: (usize, usize), max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let mut adj_heights = vec![];

    if height.0 > 0 {
        adj_heights.push((height.0 - 1, height.1));
    }

    if height.1 > 0 {
        adj_heights.push((height.0, height.1 - 1))
    }

    if height.0 + 1 < max_i {
        adj_heights.push((height.0 + 1, height.1));
    }

    if height.1 + 1 < max_j {
        adj_heights.push((height.0, height.1 + 1));
    }

    adj_heights
}

fn part_one(heights: &[Vec<u32>]) -> usize {
    let max_i = heights.len();
    let max_j = heights[0].len();

    let mut to_visit = vec![];
    let mut visited = collections::HashSet::<(usize, usize)>::new();

    let mut risk_level = 0usize;

    let start = (0, 0);

    to_visit.push(start);
    visited.insert(start);

    while let Some(height) = to_visit.pop() {
        let mut low_point = true;

        get_adj_heights(height, max_i, max_j)
            .iter()
            .for_each(|&adj| {
                if heights[height.0][height.1] >= heights[adj.0][adj.1] {
                    low_point = false;
                }

                if !visited.contains(&adj) {
                    to_visit.push(adj);
                    visited.insert(adj);
                }
            });

        if low_point {
            risk_level += heights[height.0][height.1] as usize + 1;
        }
    }

    risk_level
}

fn part_two(heights: &[Vec<u32>]) -> usize {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let heights = read_heights()?;

    println!("Part one: {}", part_one(&heights));
    // println!("Part two: {}", part_two(&heights));

    Ok(())
}
