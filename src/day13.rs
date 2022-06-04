use std::collections;
use std::io::{self, BufRead};

extern crate anyhow;

#[derive(Clone, PartialEq)]
enum Axis {
    X,
    Y,
}

#[derive(Clone)]
struct Fold {
    axis: Axis,
    line: usize,
}

#[derive(Clone)]
struct Manual {
    points: collections::HashSet<(usize, usize)>,
    folds: Vec<Fold>,
}

impl Manual {
    pub fn read() -> anyhow::Result<Self> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let manual = Manual {
            points: Self::read_points(&mut lines)?,
            folds: Self::read_folds(&mut lines)?,
        };

        Ok(manual)
    }

    pub fn into_grid(self) -> Vec<Vec<char>> {
        let max_x = self.points.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = self.points.iter().map(|&(_, y)| y).max().unwrap();

        (0..=max_y)
            .map(|y| {
                (0..=max_x)
                    .map(|x| {
                        if self.points.contains(&(x, y)) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn fold(self) -> Self {
        let mut points = self.points;
        let mut folds = self.folds.into_iter();

        if let Some(fold) = folds.next() {
            points = if fold.axis == Axis::X {
                Self::fold_x_axis(points, fold.line)
            } else {
                Self::fold_y_axis(points, fold.line)
            };
        }

        Manual {
            points,
            folds: folds.collect::<Vec<_>>(),
        }
    }

    pub fn fold_all(self) -> Self {
        let mut points = self.points;
        let mut folds = self.folds.into_iter();

        while let Some(fold) = folds.next() {
            points = if fold.axis == Axis::X {
                Self::fold_x_axis(points, fold.line)
            } else {
                Self::fold_y_axis(points, fold.line)
            };
        }

        Manual {
            points,
            folds: folds.collect::<Vec<_>>(),
        }
    }

    fn fold_x_axis(
        points: collections::HashSet<(usize, usize)>,
        line: usize,
    ) -> collections::HashSet<(usize, usize)> {
        let max = points.iter().map(|&(x, _)| x).max().unwrap();

        points
            .into_iter()
            .map(|(x, y)| if x > line { (max - x, y) } else { (x, y) })
            .collect()
    }

    fn fold_y_axis(
        points: collections::HashSet<(usize, usize)>,
        line: usize,
    ) -> collections::HashSet<(usize, usize)> {
        let max = points.iter().map(|&(_, y)| y).max().unwrap();

        points
            .into_iter()
            .map(|(x, y)| if y > line { (x, max - y) } else { (x, y) })
            .collect()
    }

    fn read_points(
        lines: &mut io::Lines<io::StdinLock>,
    ) -> anyhow::Result<collections::HashSet<(usize, usize)>> {
        let mut points = collections::HashSet::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            let line = line
                .split(',')
                .map(|coord| coord.parse())
                .collect::<Result<Vec<_>, _>>()?;

            points.insert((line[0], line[1]));
        }

        Ok(points)
    }

    fn read_folds(lines: &mut io::Lines<io::StdinLock>) -> anyhow::Result<Vec<Fold>> {
        const LEN: usize = "fold along ".len();

        let mut folds = vec![];

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            let line = line[LEN..].split('=').collect::<Vec<_>>();

            let axis = if line[0] == "x" { Axis::X } else { Axis::Y };
            let line = line[1].parse()?;

            folds.push(Fold { axis, line });
        }

        Ok(folds)
    }
}

fn part_one(manual: Manual) -> usize {
    manual.fold().len()
}

fn part_two(manual: Manual) -> Vec<Vec<char>> {
    manual.fold_all().into_grid()
}

fn main() -> anyhow::Result<()> {
    let manual = Manual::read()?;

    println!("Part one: {}", part_one(manual.clone()));
    println!("Part two:");
    part_two(manual).iter().for_each(|row| {
        row.iter().for_each(|point| print!("{}", point));
        println!()
    });

    Ok(())
}
