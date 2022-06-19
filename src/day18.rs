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

    fn reduce(self, depth: usize, is_reduced: &mut bool) -> Self {
        match self.pair {
            Some(pair) => Self {
                pair: Some(Rc::new(RefCell::new(
                    pair.take().reduce(depth + 1, is_reduced),
                ))),
                num: None,
            },
            None => self,
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
    fn reduce(self, depth: usize, is_reduced: &mut bool) -> Self {
        if *is_reduced {
            self
        } else if depth == 4 {
            // TODO: Reduce after explode?
            *is_reduced = true;
            return self.explode();
        } else {
            println!("{:?}", self);

            Self {
                left: self.left.reduce(depth, is_reduced),
                right: self.right.reduce(depth, is_reduced),
                parent: self.parent,
            }
        }

        // if let Some(_) = self
        //     .left
        //     .pair
        //     .as_ref()
        //     .and_then(|left| left.borrow_mut().reduce(depth + 1))
        // {
        //     return Some(());
        // } else if let Some(_) = self
        //     .right
        //     .pair
        //     .as_ref()
        //     .and_then(|right| right.borrow_mut().reduce(depth + 1))
        // {
        //     return Some(());
        // }

        // else if let Some(left_num) = self.left.num {
        //     if left_num >= 10 {
        //         self.split(left_num)?;
        //     }
        // } else if let Some(right_num) = self.right.num {
        //     if right_num > 10 {
        //         self.split(right_num)?;
        //     }
        // }
    }

    // Note: Exploding pairs will always consist of two regular numbers.
    fn explode(self) -> Self {
        println!("Exploding {:?}...", self);

        self

        // let mut parent = self.parent.upgrade();
        // let mut found_left = false;
        // let mut found_right = false;

        // parent.unwrap().borrow();

        // todo!()

        // while let Some(curr) = parent {
        //     if let Some(ref mut left_num) = curr.borrow_mut().left.num {
        //         if !found_left {
        //             *left_num += self.left.num.unwrap();
        //             found_left = true;
        //         }
        //     }

        //     if let Some(ref mut right_num) = curr.borrow_mut().right.num {
        //         if !found_right {
        //             *right_num += self.right.num.unwrap();
        //             found_right = true;
        //         }
        //     }

        //     if found_left && found_right {
        //         break;
        //     }

        //     parent = curr.borrow().parent.upgrade();
        // }

        // let parent = self.parent.upgrade().unwrap();

        // if let Some(ref left) = parent.borrow().left.pair {
        //     if let (Some(left), Some(right)) = (left.borrow().left.num, left.borrow().right.num) {
        //         if left == self.left.num.unwrap() && right == self.right.num.unwrap() {
        //             parent.borrow_mut().left = SnailFishElem::from_num(0);
        //         }
        //     }
        // } else {
        //     parent.borrow_mut().right = SnailFishElem::from_num(0);
        // };
    }

    fn split(self, num: u8) -> Self {
        println!("Splitting {:?}...", self);
        self
        // Ok(())
        // todo!()
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
    fn new(root: Rc<RefCell<SnailFishNode>>) -> Self {
        Self { root }
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

    fn reduce(self) -> Self {
        Self::new(Rc::new(RefCell::new(
            self.root.take().reduce(0, &mut false),
        )))
    }
}

// fn part_one(fish: &[SnailFish]) -> usize {
//     todo!()
// }

fn main() -> Result<()> {
    let mut fish = util::read_input::<SnailFish>()?;
    let fish = fish.pop().unwrap();

    println!("{:?}", &fish);
    println!("{:?}", &fish.reduce());

    Ok(())
}
