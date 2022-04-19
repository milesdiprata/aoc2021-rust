use std::collections;
use std::io;
use std::io::BufRead;

extern crate anyhow;

const BOARD_LEN: u8 = 5;

#[derive(Debug, Clone)]
struct Cell {
    num: u8,
    is_marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Cell>,
    num_idxs: collections::HashMap<u8, u8>,
}

impl Board {
    fn get(&self, i: u8, j: u8) -> &Cell {
        &self.grid[((i * BOARD_LEN) + j) as usize]
    }

    fn get_score(&self, winning_num: u8) -> u32 {
        self.grid
            .iter()
            .filter(|cell| !cell.is_marked)
            .map(|cell| cell.num as u32)
            .sum::<u32>()
            * winning_num as u32
    }

    fn is_winner(&self) -> bool {
        (0..BOARD_LEN).any(|i| (0..BOARD_LEN).all(|j| self.get(i, j).is_marked))
            || (0..BOARD_LEN).any(|j| (0..BOARD_LEN).all(|i| self.get(i, j).is_marked))
    }

    fn update(self, num: u8) -> Self {
        let mut grid = self.grid;

        if let Some(&idx) = self.num_idxs.get(&num) {
            grid[idx as usize].is_marked = true;
        }

        Board {
            grid,
            num_idxs: self.num_idxs,
        }
    }
}

fn read_nums() -> anyhow::Result<Vec<u8>> {
    io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Empty drawn numbers input!"))??
        .split(',')
        .into_iter()
        .map(|num| num.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| anyhow::anyhow!("Parse error!"))
}

fn read_board() -> anyhow::Result<Board> {
    let grid = io::stdin()
        .lock()
        .lines()
        .skip(1)
        .take(BOARD_LEN as usize)
        .into_iter()
        .map(|line| {
            line.map(|line| {
                if line.is_empty() {
                    return Err(anyhow::anyhow!("Unexpected empty board input line!"));
                }

                Ok(line
                    .split_whitespace()
                    .map(|num| u8::from_str_radix(num, 10))
                    .collect::<Result<Vec<_>, _>>())
            })
        })
        .collect::<Result<Result<Result<Vec<_>, _>, _>, _>>()???
        .into_iter()
        .flatten()
        .map(|num| Cell {
            num,
            is_marked: false,
        })
        .collect::<Vec<_>>();

    let num_idxs = grid
        .iter()
        .enumerate()
        .map(|(i, cell)| (cell.num, i as u8))
        .collect::<collections::HashMap<_, _>>();

    Ok(Board { grid, num_idxs })
}

fn read_boards() -> anyhow::Result<Vec<Board>> {
    let mut boards = vec![];
    while let Ok(board) = read_board() {
        boards.push(board);
    }

    Ok(boards)
}

fn part_one(nums: &[u8], mut boards: Vec<Board>) -> anyhow::Result<u32> {
    let mut nums = nums.iter();

    loop {
        match nums.next() {
            Some(&num) => {
                boards = boards
                    .into_iter()
                    .map(|board| board.update(num))
                    .collect::<Vec<_>>();

                if let Some(winner) = boards.iter().find(|&board| board.is_winner()) {
                    return Ok(winner.get_score(num));
                }
            }
            None => return Err(anyhow::anyhow!("No winner!")),
        }
    }
}

fn part_two(nums: &[u8], mut boards: Vec<Board>) -> anyhow::Result<u32> {
    let mut nums = nums.iter();

    loop {
        match nums.next() {
            Some(&num) => {
                boards = boards
                    .into_iter()
                    .map(|board| board.update(num))
                    .collect::<Vec<_>>();

                let last_winner = boards
                    .iter()
                    .cloned()
                    .filter(|board| board.is_winner())
                    .last();

                boards.retain(|board| !board.is_winner());

                if boards.is_empty() && last_winner.is_some() {
                    return Ok(last_winner.unwrap().get_score(num));
                }
            }
            None => return Err(anyhow::anyhow!("No winner!")),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let nums = read_nums()?;
    let boards = read_boards()?;

    println!("Part one {}", part_one(&nums, boards.clone())?);
    println!("Part two {}", part_two(&nums, boards)?);

    Ok(())
}
