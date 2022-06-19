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
    left: Option<SnailFishElem>,
    right: Option<SnailFishElem>,
    parent: Weak<RefCell<Self>>,
}

struct SnailFish {
    pair: Rc<RefCell<SnailFishNode>>,
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
            .and_then(|_| match self.left {
                Some(ref left) => {
                    let res = write!(fmt, "{}{:?}", sep, left);
                    sep = ",";
                    res
                }
                None => Ok(()),
            })
            .and_then(|_| match self.right {
                Some(ref right) => {
                    let res = write!(fmt, "{}{:?}", sep, right);
                    sep = ",";
                    res
                }
                None => Ok(()),
            })
            .and_then(|_| write!(fmt, "]"))
    }
}

impl Default for SnailFishNode {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            parent: Weak::new(),
        }
    }
}

impl SnailFishNode {
    fn with_left(self, left: Self) -> Self {
        let elem = SnailFishElem::from_pair(Rc::new(RefCell::new(left)));

        let left = match self.left {
            Some(left) => {
                let mut new_left = SnailFishNode::default();
                new_left.left = Some(left);
                new_left.right = Some(elem);

                SnailFishElem::from_pair(Rc::new(RefCell::new(new_left)))
            }
            None => elem,
        };

        Self {
            left: Some(left),
            right: self.right,
            parent: self.parent,
        }
    }

    fn with_right(self, right: Self) -> Self {
        let elem = SnailFishElem::from_pair(Rc::new(RefCell::new(right)));

        let right = match self.right {
            Some(right) => {
                let mut new_right = SnailFishNode::default();
                new_right.left = Some(right);
                new_right.right = Some(elem);

                SnailFishElem::from_pair(Rc::new(RefCell::new(new_right)))
            }
            None => elem,
        };

        Self {
            left: self.left,
            right: Some(right),
            parent: self.parent,
        }
    }

    fn with_parent(self, parent: Weak<RefCell<Self>>) -> Self {
        Self {
            left: self.left,
            right: self.right,
            parent,
        }
    }

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
        write!(fmt, "{:?}", self.pair.borrow())
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
            pair: Rc::default(),
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let fish = Self::default();

        let lhs = self.with_parent(&fish);
        let rhs = rhs.with_parent(&fish);

        fish.with_left(lhs).with_right(rhs)
    }
}

impl SnailFish {
    fn new(pair: SnailFishNode) -> Self {
        Self {
            pair: Rc::new(RefCell::new(pair)),
        }
    }

    fn with_left(self, left: Self) -> Self {
        Self::new(self.pair.take().with_left(left.pair.take()))
    }

    fn with_right(self, right: Self) -> Self {
        Self::new(self.pair.take().with_right(right.pair.take()))
    }

    fn with_parent(self, parent: &Self) -> Self {
        Self::new(self.pair.take().with_parent(Rc::downgrade(&parent.pair)))
    }

    fn from_queue(parent: Option<&Self>, input: &mut VecDeque<char>) -> Result<Self> {
        let fish = match parent {
            Some(parent) => Self::default().with_parent(parent),
            None => Self::default(),
        };

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

        if is_left && self.pair.borrow().left.is_none() {
            self.pair.borrow_mut().left = Some(SnailFishElem::default());
        } else if !is_left && self.pair.borrow().right.is_none() {
            self.pair.borrow_mut().right = Some(SnailFishElem::default());
        }

        match is_left {
            true => self.pair.borrow_mut().left.as_mut().unwrap().num = Some(num),
            false => self.pair.borrow_mut().right.as_mut().unwrap().num = Some(num),
        };

        Ok(())
    }

    fn parse_children(&self, input: &mut VecDeque<char>, is_left: bool) -> Result<()> {
        input.push_front('[');

        let fish = Self::from_queue(Some(&self), input)?;

        if is_left && self.pair.borrow().left.is_none() {
            self.pair.borrow_mut().left = Some(SnailFishElem::default());
        } else if !is_left && self.pair.borrow().right.is_none() {
            self.pair.borrow_mut().right = Some(SnailFishElem::default());
        }

        match is_left {
            true => self.pair.borrow_mut().left.as_mut().unwrap().pair = Some(fish.pair),
            false => self.pair.borrow_mut().right.as_mut().unwrap().pair = Some(fish.pair),
        };

        Ok(())
    }

    fn reduce(self) -> Result<Self> {
        Ok(Self::new(self.pair.take().reduce(0)?))
    }
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

    // println!(
    //     "{:?}",
    //     fish.pair
    //         .borrow()
    //         .left
    //         .as_ref()
    //         .unwrap()
    //         .pair
    //         .as_ref()
    //         .unwrap()
    //         .borrow()
    //         .parent
    //         .upgrade()
    //         .unwrap()
    // );

    Ok(())
}
