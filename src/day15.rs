use std::{
    cmp, collections,
    io::{self, BufRead},
};

extern crate anyhow;

struct ChitonCave(Vec<Vec<u8>>);

#[derive(PartialEq, Eq)]
struct Node {
    point: (usize, usize),
    score: usize,
}

impl ChitonCave {
    pub fn read() -> anyhow::Result<Self> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let mut cave = vec![];

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            cave.push(
                line.chars()
                    .map(|c| c.to_digit(10))
                    .map(|risk_lvl| risk_lvl.map(|risk_lvl| risk_lvl as u8))
                    .map(|risk_lvl| {
                        risk_lvl.ok_or_else(|| anyhow::anyhow!("Failed to parse risk level!"))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }

        Ok(Self(cave))
    }

    pub fn find_exit_a_star(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut g_score = collections::HashMap::new();
        let mut f_score = collections::HashMap::new();

        let len = self.0[0].len();
        (0..len)
            .flat_map(|i| (0..len).map(move |j| (i, j)))
            .for_each(|point| {
                g_score.insert(point, usize::MAX);
                f_score.insert(point, usize::MAX);
            });

        g_score.insert(start, 0);
        f_score.insert(start, 0);

        let mut visited = collections::HashSet::new();
        visited.insert(start);

        let mut frontier = collections::BinaryHeap::new();
        frontier.push(cmp::Reverse(Node {
            point: start,
            score: f_score[&start],
        }));

        while let Some(node) = frontier.pop() {
            if node.0.point == end {
                break;
            }

            Self::get_adj(node.0.point, len)
                .into_iter()
                .for_each(|adj| {
                    let tentative_g_score = g_score[&node.0.point] + self.0[adj.0][adj.1] as usize;

                    if tentative_g_score < g_score[&adj] {
                        g_score.insert(adj, tentative_g_score);
                        f_score.insert(adj, tentative_g_score + Self::get_h_score(adj, end));

                        if !visited.contains(&adj) {
                            visited.insert(adj);
                            frontier.push(cmp::Reverse(Node {
                                point: adj,
                                score: f_score[&adj],
                            }));
                        }
                    }
                });
        }

        g_score[&end]
    }

    fn get_adj((x, y): (usize, usize), len: usize) -> Vec<(usize, usize)> {
        (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .map(|(i, j)| (i as isize, j as isize))
            .filter(|&(i, j)| i.abs() != j.abs())
            .map(|(i, j)| (x as isize + i, y as isize + j))
            .filter(|&(i, j)| i >= 0 && i < len as isize && j >= 0 && j < len as isize)
            .map(|(i, j)| (i as usize, j as usize))
            .collect()
    }

    fn get_h_score(point: (usize, usize), end: (usize, usize)) -> usize {
        (end.0 - point.0) + (end.1 - point.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn part_one(cave: &ChitonCave) -> usize {
    let len = cave.0[0].len() - 1;

    cave.find_exit_a_star((0, 0), (len, len))
}

fn main() -> anyhow::Result<()> {
    let cave = ChitonCave::read()?;

    println!("Part one: {}", part_one(&cave));

    Ok(())
}
