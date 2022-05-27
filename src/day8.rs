use std::collections;
use std::str;

extern crate anyhow;

extern crate aoc2021_rust;

use aoc2021_rust::util;

const SEVEN_SEG_DIGS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

struct Sig {
    patterns: [String; 10],
    output: [String; 4],
}

impl str::FromStr for Sig {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let input = input.split(" | ").collect::<Vec<_>>();

        if input.len() != 2 {
            return Err(anyhow::anyhow!("Invalid signal sequence!"));
        }

        let patterns = input
            .first()
            .ok_or_else(|| anyhow::anyhow!("Invalid unique patterns!"))?
            .split(' ')
            .map(|value| value.to_owned())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Length of patterns not equal to 10!"))?;

        let output = input
            .last()
            .ok_or_else(|| anyhow::anyhow!("Invalid output values!"))?
            .split(' ')
            .map(|value| value.to_owned())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Length of output not equal to 4!"))?;

        Ok(Sig { patterns, output })
    }
}

fn part_one(sigs: &[Sig]) -> usize {
    let unique_dig_sig_lens = collections::HashSet::from([
        SEVEN_SEG_DIGS[1].len(),
        SEVEN_SEG_DIGS[4].len(),
        SEVEN_SEG_DIGS[7].len(),
        SEVEN_SEG_DIGS[8].len(),
    ]);

    sigs.iter()
        .flat_map(|sig| {
            sig.output
                .iter()
                .filter(|&output| unique_dig_sig_lens.contains(&output.len()))
        })
        .collect::<Vec<_>>()
        .len()
}

fn part_two(sigs: &[Sig]) -> usize {
    let sig_freqs = get_sig_freq(&SEVEN_SEG_DIGS);

    let mut dig_scores = collections::HashMap::new();

    SEVEN_SEG_DIGS.iter().enumerate().for_each(|(no, &dig)| {
        dig.chars()
            .for_each(|c| *dig_scores.entry(no as u8).or_insert(0) += sig_freqs[&c])
    });

    // Coincidentally, the score of each digit following the above frequency-
    // based approach yields a unique value among all the digits... Use this
    // score to uniquely match the scrambled digits following the same
    // frequency-based scoring approach for the scrambled signals.

    let digs = dig_scores
        .into_iter()
        .map(|(dig, score)| (score, dig))
        .collect::<collections::HashMap<_, _>>();

    sigs.iter()
        .map(|sig| {
            let scrambled_sig_freqs = get_sig_freq(
                &sig.patterns
                    .iter()
                    .map(|pattern| pattern.as_str())
                    .collect::<Vec<_>>(),
            );

            let scrambled_digs = sig
                .output
                .iter()
                .map(|dig| dig.chars().map(|c| scrambled_sig_freqs[&c]).sum::<usize>())
                .map(|scrambled_score| digs[&scrambled_score])
                .collect::<Vec<_>>();

            (scrambled_digs[0] as usize * 1000)
                + (scrambled_digs[1] as usize * 100)
                + (scrambled_digs[2] as usize * 10)
                + scrambled_digs[3] as usize
        })
        .sum()
}

fn get_sig_freq(patterns: &[&str]) -> collections::HashMap<char, usize> {
    let mut freqs = collections::HashMap::new();

    patterns.iter().for_each(|&dig| {
        dig.chars().for_each(|c| *freqs.entry(c).or_insert(0) += 1);
    });

    freqs
}

fn main() -> anyhow::Result<()> {
    let sigs = util::read_input::<Sig>()?;

    println!("Part one: {}", part_one(&sigs));
    println!("Part two: {}", part_two(&sigs));

    Ok(())
}
