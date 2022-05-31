use std::io::BufRead;
use std::{collections, io};

extern crate anyhow;

fn read_input() -> anyhow::Result<Vec<String>> {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let mut input = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        input.push(line)
    }

    Ok(input)
}

fn part_one(lines: &[String]) -> usize {
    let illegal_paren_scores =
        collections::HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    lines
        .iter()
        .map(|line| {
            let mut closing_parens = vec![];

            line.chars().find(|&c| {
                if c == '(' {
                    closing_parens.push(')');
                } else if c == '[' {
                    closing_parens.push(']');
                } else if c == '{' {
                    closing_parens.push('}');
                } else if c == '<' {
                    closing_parens.push('>');
                } else if closing_parens.is_empty() {
                    return false;
                } else if closing_parens.pop().unwrap() != c {
                    return true;
                }

                false
            })
        })
        .flatten()
        .map(|illegal_paren| illegal_paren_scores[&illegal_paren])
        .map(|score| score as usize)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let lines = read_input()?;

    println!("Part one: {}", part_one(&lines));

    Ok(())
}
