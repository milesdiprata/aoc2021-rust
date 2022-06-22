use core::fmt;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::ops::Add;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

enum SnailFishElem {
    Num(Rc<RefCell<usize>>),
    Pair(Rc<RefCell<SnailFishNode>>),
}

struct SnailFishNode {
    left: SnailFishElem,
    right: SnailFishElem,
    parent: Option<Weak<RefCell<Self>>>,
}

struct SnailFish(Rc<RefCell<SnailFishNode>>);

impl fmt::Debug for SnailFishElem {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(num) => write!(fmt, "{}", num.borrow()),
            Self::Pair(pair) => write!(fmt, "{:?}", &pair.borrow()),
        }
    }
}

impl fmt::Debug for SnailFishNode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[{:?},{:?}]", &self.left, &self.right)
    }
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{:?}", &self.0.borrow())
    }
}

impl Default for SnailFishElem {
    fn default() -> Self {
        Self::Num(Rc::new(RefCell::new(0)))
    }
}

impl Default for SnailFishNode {
    fn default() -> Self {
        Self {
            left: SnailFishElem::default(),
            right: SnailFishElem::default(),
            parent: None,
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
        Self::from_queue(None, &mut str.chars().collect())
    }
}

impl PartialEq for SnailFishElem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(lhs), Self::Num(rhs)) => *lhs.borrow() == *rhs.borrow(),
            (Self::Pair(lhs), Self::Pair(rhs)) => lhs.as_ptr() == rhs.as_ptr(),
            _ => false,
        }
    }
}

impl PartialEq for SnailFishNode {
    fn eq(&self, other: &Self) -> bool {
        let is_same_elems = self.left == other.left && self.right == other.right;

        match (
            self.parent.as_ref().and_then(|parent| parent.upgrade()),
            other.parent.as_ref().and_then(|parent| parent.upgrade()),
        ) {
            (Some(lhs), Some(rhs)) => lhs.as_ptr() == rhs.as_ptr() && is_same_elems,
            (None, None) => is_same_elems,
            _ => false,
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let fish = Self::default();
        let parent = fish.to_parent();

        let left = self.with_parent(parent.clone()).into_reduced().unwrap();
        let right = rhs.with_parent(parent).into_reduced().unwrap();

        fish.with_left(left)
            .with_right(right)
            .into_reduced()
            .unwrap()
    }
}

impl Sum for SnailFish {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut iter = iter;
        let acc = iter.next().unwrap();

        iter.fold(acc, |acc, fish| acc + fish)
    }
}

impl SnailFishElem {
    fn is_num(&self) -> bool {
        match self {
            SnailFishElem::Num(_) => true,
            SnailFishElem::Pair(_) => false,
        }
    }

    fn is_pair(&self) -> bool {
        match self {
            SnailFishElem::Num(_) => false,
            SnailFishElem::Pair(_) => true,
        }
    }

    fn peek_num(&self) -> &Rc<RefCell<usize>> {
        match self {
            SnailFishElem::Num(num) => num,
            SnailFishElem::Pair(_) => panic!("SnailFishElem is not a number!"),
        }
    }

    fn peek_pair(&self) -> &Rc<RefCell<SnailFishNode>> {
        match self {
            SnailFishElem::Num(_) => panic!("SnailFishElem is not a pair!"),
            SnailFishElem::Pair(pair) => pair,
        }
    }
}

impl SnailFish {
    fn from_stdin() -> Result<Self> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        let mut input = vec![];

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            input.push(line);
        }

        let fish = input
            .into_iter()
            .map(|fish| fish.parse())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum();

        Ok(fish)
    }

    fn from_queue(
        parent: Option<Weak<RefCell<SnailFishNode>>>,
        input: &mut VecDeque<char>,
    ) -> Result<Self> {
        let fish = match parent {
            Some(parent) => Self::default().with_parent(parent),
            None => Self::default(),
        };

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
        self.0.borrow_mut().parent = Some(parent);
        Self(self.0)
    }

    fn with_left(self, left: Self) -> Self {
        self.0.borrow_mut().left = SnailFishElem::Pair(left.0);
        Self(self.0)
    }

    fn with_right(self, right: Self) -> Self {
        self.0.borrow_mut().right = SnailFishElem::Pair(right.0);
        Self(self.0)
    }

    fn to_parent(&self) -> Weak<RefCell<SnailFishNode>> {
        Rc::downgrade(&self.0)
    }

    fn parse_num(&self, num: char, is_left: bool) -> Result<()> {
        let num = SnailFishElem::Num(Rc::new(RefCell::new(
            num.to_digit(10)
                .map(|num| num as usize)
                .ok_or_else(|| anyhow!("Failed to parse num!"))?,
        )));

        match is_left {
            true => self.0.borrow_mut().left = num,
            false => self.0.borrow_mut().right = num,
        }

        Ok(())
    }

    fn parse_pair(&self, input: &mut VecDeque<char>, is_left: bool) -> Result<()> {
        input.push_front('[');

        let pair = SnailFishElem::Pair(Self::from_queue(Some(self.to_parent()), input)?.0);

        match is_left {
            true => self.0.borrow_mut().left = pair,
            false => self.0.borrow_mut().right = pair,
        }

        Ok(())
    }

    fn into_reduced(self) -> Result<Self> {
        loop {
            if let Some(node) = self.find_exploded(&self.0, 0, &mut false)? {
                self.explode_node(&node)?;
            } else if let Some((node, is_left)) = Self::find_split(&self.0) {
                Self::split_node(&node, is_left)?;
            } else {
                break;
            }
        }

        Ok(self)
    }

    fn find_exploded(
        &self,
        node: &Rc<RefCell<SnailFishNode>>,
        depth: usize,
        is_reduced: &mut bool,
    ) -> Result<Option<Rc<RefCell<SnailFishNode>>>> {
        if depth == 4 {
            return Ok(Some(node.clone()));
        } else if !*is_reduced {
            if let SnailFishElem::Pair(left) = &node.borrow().left {
                if let Some(node) = self.find_exploded(left, depth + 1, is_reduced)? {
                    return Ok(Some(node));
                }
            }

            if let SnailFishElem::Pair(right) = &node.borrow().right {
                if let Some(node) = self.find_exploded(right, depth + 1, is_reduced)? {
                    return Ok(Some(node));
                }
            }
        }

        Ok(None)
    }

    fn explode_node(&self, node: &Rc<RefCell<SnailFishNode>>) -> Result<()> {
        let (left_num, right_num) = match (&node.borrow().left, &node.borrow().right) {
            (SnailFishElem::Num(left_num), SnailFishElem::Num(right_num)) => {
                (left_num.borrow().to_owned(), right_num.borrow().to_owned())
            }
            _ => return Err(anyhow!("Exploding pair does not contain two numbers!")),
        };

        if let Some(num) = self.find_closest_num(node, true) {
            *num.borrow_mut() += left_num;
        }

        if let Some(num) = self.find_closest_num(node, false) {
            *num.borrow_mut() += right_num;
        }

        let parent = node
            .borrow()
            .parent
            .as_ref()
            .and_then(|parent| parent.upgrade())
            .unwrap();

        let is_left_child = if let SnailFishElem::Pair(left) = &parent.borrow().left {
            &*left.borrow() == &*node.borrow()
        } else {
            false
        };

        match is_left_child {
            true => parent.borrow_mut().left = SnailFishElem::default(),
            false => parent.borrow_mut().right = SnailFishElem::default(),
        }

        Ok(())
    }

    fn find_closest_num(
        &self,
        node: &Rc<RefCell<SnailFishNode>>,
        is_left: bool,
    ) -> Option<Rc<RefCell<usize>>> {
        let num = match is_left {
            true => node.borrow().left.peek_num().clone(),
            false => node.borrow().right.peek_num().clone(),
        };

        let flat = self.to_sorted_vec();
        let idx = flat
            .iter()
            .enumerate()
            .find(|&(_, flat_num)| flat_num.as_ptr() == num.as_ptr())
            .map(|(idx, _)| idx)
            .unwrap();

        if idx == 0 || idx == flat.len() - 1 {
            return None;
        }

        let num = match is_left {
            true => &flat[idx - 1],
            false => &flat[idx + 1],
        };

        Some(num.clone())
    }

    fn to_sorted_vec(&self) -> Vec<Rc<RefCell<usize>>> {
        let mut vec = vec![];
        Self::in_order_nums(&self.0, &mut vec);

        vec
    }

    fn to_magnitude(&self) -> usize {
        let mut magnitude = 0;
        Self::in_order_magnitude(&self.0, 1, &mut magnitude);

        magnitude
    }

    fn find_split(node: &Rc<RefCell<SnailFishNode>>) -> Option<(Rc<RefCell<SnailFishNode>>, bool)> {
        if let SnailFishElem::Pair(node) = &node.borrow().left {
            if let Some((node, is_left)) = Self::find_split(node) {
                return Some((node, is_left));
            }
        }

        if let SnailFishElem::Num(num) = &node.borrow().left {
            if *num.borrow() >= 10 {
                return Some((node.clone(), true));
            }
        }

        if let SnailFishElem::Num(num) = &node.borrow().right {
            if *num.borrow() >= 10 {
                return Some((node.clone(), false));
            }
        }

        if let SnailFishElem::Pair(node) = &node.borrow().right {
            if let Some((node, is_left)) = Self::find_split(node) {
                return Some((node, is_left));
            }
        }

        None
    }

    fn split_node(node: &Rc<RefCell<SnailFishNode>>, is_left: bool) -> Result<()> {
        let num = match is_left {
            true => match &node.borrow().left {
                SnailFishElem::Num(num) => *num.borrow(),
                SnailFishElem::Pair(_) => {
                    return Err(anyhow!("Split pair does not contain a number!"))
                }
            },
            false => match &node.borrow().right {
                SnailFishElem::Num(num) => *num.borrow(),
                SnailFishElem::Pair(_) => {
                    return Err(anyhow!("Split pair does not contain a number!"))
                }
            },
        };

        let fish = Self::default().with_parent(Rc::downgrade(node));
        let num = num as f32 / 2.0;

        fish.0.borrow_mut().left = SnailFishElem::Num(Rc::new(RefCell::new(num.floor() as usize)));
        fish.0.borrow_mut().right = SnailFishElem::Num(Rc::new(RefCell::new(num.ceil() as usize)));

        match is_left {
            true => node.borrow_mut().left = SnailFishElem::Pair(fish.0),
            false => node.borrow_mut().right = SnailFishElem::Pair(fish.0),
        };

        Ok(())
    }

    fn in_order_nums(node: &Rc<RefCell<SnailFishNode>>, nums: &mut Vec<Rc<RefCell<usize>>>) -> () {
        if let SnailFishElem::Pair(left) = &node.borrow().left {
            Self::in_order_nums(left, nums);
        }

        if let SnailFishElem::Num(num) = &node.borrow().left {
            nums.push(num.clone());
        }

        if let SnailFishElem::Num(num) = &node.borrow().right {
            nums.push(num.clone());
        }

        if let SnailFishElem::Pair(right) = &node.borrow().right {
            Self::in_order_nums(right, nums);
        }
    }

    fn in_order_magnitude(
        node: &Rc<RefCell<SnailFishNode>>,
        multi: usize,
        magnitude: &mut usize,
    ) -> () {
        if let SnailFishElem::Pair(left) = &node.borrow().left {
            Self::in_order_magnitude(left, multi * 3, magnitude);
        }

        if let SnailFishElem::Num(num) = &node.borrow().left {
            *magnitude += 3 * multi * *num.borrow();
        }

        if let SnailFishElem::Num(num) = &node.borrow().right {
            *magnitude += 2 * multi * *num.borrow();
        }

        if let SnailFishElem::Pair(right) = &node.borrow().right {
            Self::in_order_magnitude(right, multi * 2, magnitude);
        }
    }
}

fn main() -> Result<()> {
    let fish = SnailFish::from_stdin()?.to_magnitude();

    println!("{:?}", &fish);

    Ok(())
}
