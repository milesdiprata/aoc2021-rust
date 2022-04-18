use std::collections;
use std::io;
use std::io::BufRead;

extern crate anyhow;

const BOARD_LEN: u8 = 5;

#[derive(Debug)]
struct Cell {
    num: u8,
    is_marked: bool,
}

#[derive(Debug)]
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
        (0..BOARD_LEN).all(|i| self.get(i, 0).is_marked)
            || (0..BOARD_LEN).all(|j| self.get(0, j).is_marked)
    }

    fn update(&mut self, num: u8) -> () {
        if let Some(&idx) = self.num_idxs.get(&num) {
            self.grid[idx as usize].is_marked = true;
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

fn main() -> anyhow::Result<()> {
    let mut nums = read_nums()?.into_iter();
    let mut boards = read_boards()?;

    loop {
        match nums.next() {
            Some(num) => {
                boards.iter_mut().for_each(|board| board.update(num));
                if let Some(winner) = boards.iter().find(|&board| board.is_winner()) {
                    println!("Winning score: {}", winner.get_score(num));
                    break;
                }
            }
            None => break,
        }
    }

    Ok(())
}
