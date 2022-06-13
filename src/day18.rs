extern crate anyhow;

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::default::Default;
use std::fmt;
use std::ops::Add;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use aoc2021_rust::util::read_input;

struct SnailFish {
    nums: Option<BinaryHeap<Reverse<u8>>>,
    pair: Option<Vec<Rc<RefCell<Self>>>>,
    parent: Weak<RefCell<Self>>,
}

struct SnailFishNode(Rc<RefCell<SnailFish>>);

impl fmt::Debug for SnailFish {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[")
            .and_then(|_| {
                if let Some(fish) = &self.pair {
                    let mut sep = "";

                    fish.iter()
                        .map(|fish| {
                            let res = write!(fmt, "{}{:?}", sep, fish.as_ref().borrow());
                            sep = ",";

                            res
                        })
                        .collect::<Result<_, _>>()?;
                }

                Ok(())
            })
            .and_then(|_| {
                if let Some(nums) = &self.nums {
                    let mut sep = match self.pair.is_some() {
                        true => ",",
                        false => "",
                    };

                    nums.iter()
                        .map(|num| {
                            let res = write!(fmt, "{}{:?}", sep, num.0);
                            sep = ",";

                            res
                        })
                        .collect::<Result<(), _>>()?;
                }

                Ok(())
            })
            .and_then(|_| write!(fmt, "]"))
    }
}

impl fmt::Debug for SnailFishNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", self.0.as_ref().borrow())
    }
}

impl Default for SnailFish {
    fn default() -> Self {
        Self {
            nums: None,
            pair: None,
            parent: Weak::new(),
        }
    }
}

impl SnailFish {
    fn new(parent: Weak<RefCell<SnailFish>>) -> Self {
        Self {
            nums: None,
            pair: None,
            parent,
        }
    }
}

impl FromStr for SnailFishNode {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        Self::from_queue(Weak::new(), &mut input.chars().collect())
    }
}

impl Add for SnailFishNode {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let pair = vec![rhs.0, self.0];
        let fish = Self::default();
        fish.0.borrow_mut().pair = Some(pair);

        fish
    }
}

impl Default for SnailFishNode {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(SnailFish::default())))
    }
}

impl SnailFishNode {
    fn new(parent: Weak<RefCell<SnailFish>>) -> Self {
        Self(Rc::new(RefCell::new(SnailFish::new(parent))))
    }

    fn from_queue(
        parent: Weak<RefCell<SnailFish>>,
        input: &mut VecDeque<char>,
    ) -> anyhow::Result<Self> {
        let fish = Self::new(parent);

        if input
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("Unexpected empty snailfish input!"))?
            != '['
        {
            return Err(anyhow::anyhow!("Expected '[' when parsing snailfish!"));
        }

        while let Some(c) = input.pop_front() {
            if c.is_digit(10) {
                fish.parse_num(c)?;
            } else if c == ',' {
                continue;
            } else if c == '[' {
                fish.parse_pair(input)?;
            } else if c == ']' {
                break;
            } else {
                return Err(anyhow::anyhow!(
                    "Unknown character '{}' in snailfish input!",
                    c
                ));
            }
        }

        Ok(fish)
    }

    fn parse_num(&self, num: char) -> anyhow::Result<()> {
        if self.0.borrow_mut().nums.is_none() {
            self.0.borrow_mut().nums = Some(BinaryHeap::with_capacity(2));
        }

        self.0.borrow_mut().nums.as_mut().unwrap().push(Reverse(
            num.to_digit(10)
                .ok_or_else(|| anyhow::anyhow!("Failed to parse snailfish regular number!"))?
                as u8,
        ));

        Ok(())
    }

    fn parse_pair(&self, input: &mut VecDeque<char>) -> anyhow::Result<()> {
        if self.0.borrow_mut().pair.is_none() {
            self.0.borrow_mut().pair = Some(Vec::with_capacity(2));
        }

        input.push_front('[');

        self.0
            .borrow_mut()
            .pair
            .as_mut()
            .unwrap()
            .push(Self::from_queue(Rc::downgrade(&self.0), input)?.0);

        Ok(())
    }

    fn reduce(self) -> Self {
        todo!()
    }

    fn explode(self) -> Self {
        todo!()
    }
}

fn part_one(fish: &[SnailFishNode]) -> usize {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let mut fish = read_input()?;

    println!("{:?}", fish.pop().unwrap() + fish.pop().unwrap());

    println!("Part one: {}", part_one(&fish));

    Ok(())
}
