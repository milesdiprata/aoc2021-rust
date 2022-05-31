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
                } else if closing_parens.is_empty() || closing_parens.pop().unwrap() != c {
                    return true;
                }

                // Don't care if string is valid or incomplete.
                false
            })
        })
        .flatten()
        .map(|illegal_paren| illegal_paren_scores[&illegal_paren])
        .map(|score| score as usize)
        .sum()
}

fn part_two(lines: &[String]) -> usize {
    let paren_completion_scores =
        collections::HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut completion_scores = lines
        .iter()
        .map(|line| {
            let mut closing_parens = Some(vec![]);

            line.chars().for_each(|c| {
                if let Some(parens) = &mut closing_parens {
                    if c == '(' {
                        parens.push(')');
                    } else if c == '[' {
                        parens.push(']');
                    } else if c == '{' {
                        parens.push('}');
                    } else if c == '<' {
                        parens.push('>');
                    } else if parens.is_empty() || parens.pop().unwrap() != c {
                        closing_parens = None;
                    }
                }
            });

            if let Some(parens) = &closing_parens {
                if parens.is_empty() {
                    closing_parens = None;
                }
            }

            closing_parens
        })
        .flatten()
        .map(|closing_parens| String::from_iter(closing_parens.into_iter().rev()))
        .map(|completion_str| {
            let mut score = 0usize;

            completion_str.chars().for_each(|c| {
                score *= 5;
                score += paren_completion_scores[&c];
            });

            score
        })
        .collect::<Vec<_>>();

    completion_scores.sort();

    completion_scores[completion_scores.len() / 2]
}

fn main() -> anyhow::Result<()> {
    let lines = read_input()?;

    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));

    Ok(())
}
