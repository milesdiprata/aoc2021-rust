use std::collections;
use std::io;
use std::io::BufRead;

extern crate anyhow;

struct Manual {
    template: Vec<char>,
    insert_rules: collections::HashMap<[char; 2], char>,
}

impl Manual {
    pub fn read() -> anyhow::Result<Self> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let template = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing polymer template!"))??
            .chars()
            .collect();

        lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing empty line following polymer template!"))??;

        let mut insert_rules = collections::HashMap::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut split = line.split(" -> ");

            let mut pair = split
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing pair in insertion rule!"))?
                .chars();

            let pair = [
                pair.next()
                    .ok_or_else(|| anyhow::anyhow!("Missing first element of insertion rule!"))?,
                pair.next()
                    .ok_or_else(|| anyhow::anyhow!("Missing second element of insertion rule!"))?,
            ];

            let insert = split
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing element to-insert in insertion rule!"))?
                .chars()
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing element to-insert in insertion rule!"))?;

            insert_rules.insert(pair, insert);
        }

        Ok(Manual {
            template,
            insert_rules,
        })
    }
}

fn part_one(manual: &Manual) -> usize {
    const NUM_STEPS: usize = 10;

    let mut polymer = manual.template.clone();

    (0..NUM_STEPS).for_each(|_| {
        let pairs = polymer.iter().enumerate().collect::<Vec<_>>();

        pairs
            .windows(2)
            .map(|pair| ([*pair[0].1, *pair[1].1], pair[1].0))
            .filter(|(pair, _)| manual.insert_rules.contains_key(pair))
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
            .for_each(|(i, (pair, idx))| polymer.insert(idx + i, manual.insert_rules[&pair]));
    });

    let mut freqs = collections::HashMap::new();

    polymer
        .iter()
        .for_each(|element| *freqs.entry(element).or_insert(0usize) += 1);

    let freqs = freqs
        .values()
        .collect::<collections::BinaryHeap<_>>()
        .into_sorted_vec();

    *freqs.last().unwrap() - *freqs.first().unwrap()
}

fn part_two(manual: &Manual) -> usize {
    const NUM_STEPS: usize = 40;

    let mut pairs = collections::HashMap::new();
    let mut elems = collections::HashMap::new();

    manual
        .template
        .windows(2)
        .map(|pair| [pair[0], pair[1]])
        .for_each(|pair| *pairs.entry(pair).or_insert(0usize) += 1);

    manual
        .template
        .iter()
        .for_each(|&elem| *elems.entry(elem).or_insert(0usize) += 1);

    (0..NUM_STEPS).for_each(|_| {
        let new_pairs = pairs
            .iter_mut()
            .filter(|(pair, num_occur)| **num_occur > 0 && manual.insert_rules.contains_key(*pair))
            .flat_map(|(pair, num_occur)| {
                let elem = manual.insert_rules[pair];
                let pairs = [([pair[0], elem], *num_occur), ([elem, pair[1]], *num_occur)];

                *elems.entry(elem).or_insert(0) += *num_occur;

                *num_occur = 0;

                pairs
            })
            .collect::<Vec<_>>();

        new_pairs
            .iter()
            .for_each(|&(pair, num_occur)| *pairs.entry(pair).or_insert(0) += num_occur);
    });

    let freqs = elems
        .values()
        .collect::<collections::BinaryHeap<_>>()
        .into_sorted_vec();

    *freqs.last().unwrap() - *freqs.first().unwrap()
}

fn main() -> anyhow::Result<()> {
    let manual = Manual::read()?;

    println!("Part one: {}", part_one(&manual));
    println!("Part two: {}", part_two(&manual));

    Ok(())
}
