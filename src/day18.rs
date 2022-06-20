use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::rc::{Rc, Weak};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use aoc2021_rust::util;

#[derive(Debug)]
enum SnailFishElem {
    Num(u8),
    Pair(Rc<RefCell<SnailFishNode>>),
}

#[derive(Debug)]
struct SnailFishNode {
    left: SnailFishElem,
    right: SnailFishElem,
    parent: Weak<RefCell<Self>>,
}

#[derive(Debug)]
struct SnailFish(Rc<RefCell<SnailFishNode>>);

impl Default for SnailFishElem {
    fn default() -> Self {
        Self::Num(0)
    }
}

impl Default for SnailFishNode {
    fn default() -> Self {
        Self {
            left: SnailFishElem::default(),
            right: SnailFishElem::default(),
            parent: Weak::default(),
        }
    }
}

impl Default for SnailFish {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl FromStr for SnailFish {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self> {
        Self::from_queue(Weak::new(), &mut str.chars().collect())
    }
}

impl SnailFish {
    fn from_stdin() -> Result<Vec<Self>> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        let mut input = vec![];

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            input.push(line);
        }

        input
            .into_iter()
            .map(|fish| fish.parse())
            .collect::<Result<_>>()
    }

    fn from_queue(
        parent: Weak<RefCell<SnailFishNode>>,
        input: &mut VecDeque<char>,
    ) -> Result<Self> {
        let fish = Self::default().with_parent(parent);

        if input.pop_front().ok_or_else(|| anyhow!("Missing '['!"))? != '[' {
            return Err(anyhow!("Expected '['!"));
        }

        let mut is_left = true;

        while let Some(c) = input.pop_front() {
            if c.is_digit(10) {
                fish.parse_num(c, is_left)?;
            } else if c == ',' {
                is_left = false;
                continue;
            } else if c == '[' {
                fish.parse_pair(input, is_left)?;
            } else if c == ']' {
                input.push_front(']');
                break;
            } else {
                return Err(anyhow!("Unknown character '{}' in snailfish input!", c));
            }
        }

        if input.pop_front().ok_or_else(|| anyhow!("Missing ']'!"))? != ']' {
            return Err(anyhow!("Expected ']'!"));
        }

        Ok(fish)
    }

    fn with_parent(self, parent: Weak<RefCell<SnailFishNode>>) -> Self {
        self.0.borrow_mut().parent = parent;
        Self(self.0)
    }

    fn parse_num(&self, num: char, is_left: bool) -> Result<()> {
        let num = SnailFishElem::Num(
            num.to_digit(10)
                .map(|num| num as u8)
                .ok_or_else(|| anyhow!("Failed to parse num!"))?,
        );

        match is_left {
            true => self.0.borrow_mut().left = num,
            false => self.0.borrow_mut().right = num,
        }

        Ok(())
    }

    fn parse_pair(&self, input: &mut VecDeque<char>, is_left: bool) -> Result<()> {
        input.push_front('[');

        let new_elem = SnailFishElem::Pair(Self::from_queue(Rc::downgrade(&self.0), input)?.0);

        match is_left {
            true => self.0.borrow_mut().left = new_elem,
            false => self.0.borrow_mut().right = new_elem,
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut fish = SnailFish::from_stdin()?;
    println!("{:?}", fish.pop().unwrap());

    // if let SnailFishElem::Pair(pair) = &fish.pop().unwrap().0.borrow().left {
    //     println!("{:?}", pair.borrow().parent.upgrade());
    // }

    Ok(())
}
