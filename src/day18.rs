use std::cell::RefCell;
use std::collections::VecDeque;
use std::default::Default;
use std::fmt;
use std::ops::Add;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use aoc2021_rust::util;

struct SnailFishElem {
    num: Option<u8>,
    pair: Option<Rc<RefCell<SnailFishNode>>>,
}

struct SnailFishNode {
    left: SnailFishElem,
    right: SnailFishElem,
    parent: Weak<RefCell<Self>>,
}

struct SnailFish {
    root: Rc<RefCell<SnailFishNode>>,
}

impl fmt::Debug for SnailFishElem {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(num) = self.num {
            write!(fmt, "{}", num)
        } else if let Some(ref pair) = self.pair {
            write!(fmt, "{:?}", pair.borrow())
        } else {
            Ok(())
        }
    }
}

impl Default for SnailFishElem {
    fn default() -> Self {
        Self {
            num: None,
            pair: None,
        }
    }
}

impl SnailFishElem {
    fn from_num(num: u8) -> Self {
        Self {
            num: Some(num),
            pair: None,
        }
    }

    fn from_pair(pair: Rc<RefCell<SnailFishNode>>) -> Self {
        Self {
            num: None,
            pair: Some(pair),
        }
    }
}

impl fmt::Debug for SnailFishNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";

        write!(fmt, "[")
            .and_then(|_| {
                let res = write!(fmt, "{}{:?}", sep, &self.left);
                sep = ",";
                res
            })
            .and_then(|_| {
                let res = write!(fmt, "{}{:?}", sep, &self.right);
                sep = ",";
                res
            })
            .and_then(|_| write!(fmt, "]"))
    }
}

impl Default for SnailFishNode {
    fn default() -> Self {
        Self {
            left: SnailFishElem::default(),
            right: SnailFishElem::default(),
            parent: Weak::new(),
        }
    }
}

impl SnailFishNode {
    fn reduce(self, depth: usize) -> Result<Self> {
        if depth == 4 {
            return self.explode()?.reduce(depth + 1);
        }

        // if let Some(ref num) = self.num {
        //     if let Some(&num) = num.iter().find(|&&num| num == 10) {
        //         return self.split(num).reduce(depth + 1);
        //     }
        // }

        Ok(self)
    }

    fn explode(self) -> Result<Self> {
        let parent = self
            .parent
            .upgrade()
            .ok_or_else(|| anyhow!("Parent fish was dropped!"))?;

        todo!()
    }

    fn split(self, num: u8) -> Self {
        todo!()
    }
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", &self.root.borrow())
    }
}

impl FromStr for SnailFish {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        Self::from_queue(None, &mut input.chars().collect())
    }
}

impl Default for SnailFish {
    fn default() -> Self {
        Self {
            root: Rc::default(),
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let fish = Self::default();
        let left = self;
        let right = rhs;

        left.root.borrow_mut().parent = Rc::downgrade(&fish.root);
        right.root.borrow_mut().parent = Rc::downgrade(&fish.root);

        fish.root.borrow_mut().left = SnailFishElem::from_pair(left.root);
        fish.root.borrow_mut().right = SnailFishElem::from_pair(right.root);

        fish
    }
}

impl SnailFish {
    fn new(root: SnailFishNode) -> Self {
        Self {
            root: Rc::new(RefCell::new(root)),
        }
    }

    fn from_queue(parent: Option<&Self>, input: &mut VecDeque<char>) -> Result<Self> {
        let fish = Self::default();

        if let Some(parent) = parent {
            fish.root.borrow_mut().parent = Rc::downgrade(&parent.root);
        }

        if input
            .pop_front()
            .ok_or_else(|| anyhow!("Unexpected empty snailfish input!"))?
            != '['
        {
            return Err(anyhow!("Expected '[' when parsing snailfish!"));
        }

        let mut is_left = true;

        while let Some(c) = input.pop_front() {
            if c.is_digit(10) {
                fish.parse_num(c, is_left)?;
            } else if c == ',' {
                is_left = false;
                continue;
            } else if c == '[' {
                fish.parse_children(input, is_left)?;
            } else if c == ']' {
                break;
            } else {
                return Err(anyhow!("Unknown character '{}' in snailfish input!", c));
            }
        }

        Ok(fish)
    }

    fn parse_num(&self, num: char, is_left: bool) -> Result<()> {
        let num = num
            .to_digit(10)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse snailfish regular number!"))?
            as u8;

        match is_left {
            true => self.root.borrow_mut().left = SnailFishElem::from_num(num),
            false => self.root.borrow_mut().right = SnailFishElem::from_num(num),
        };

        Ok(())
    }

    fn parse_children(&self, input: &mut VecDeque<char>, is_left: bool) -> Result<()> {
        input.push_front('[');

        let fish = Self::from_queue(Some(&self), input)?;

        match is_left {
            true => self.root.borrow_mut().left = SnailFishElem::from_pair(fish.root),
            false => self.root.borrow_mut().right = SnailFishElem::from_pair(fish.root),
        };

        Ok(())
    }

    // fn reduce(self) -> Result<Self> {
    //     Ok(Self::new(self.root.take().reduce(0)?))
    // }
}

// fn part_one(fish: &[SnailFish]) -> usize {
//     todo!()
// }

fn main() -> Result<()> {
    let mut fish = util::read_input::<SnailFish>()?;

    let two = fish.pop().unwrap();
    let one = fish.pop().unwrap();

    let fish = one + two;

    println!("{:?}", &fish);

    Ok(())
}
