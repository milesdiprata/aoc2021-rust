use std::str;

extern crate anyhow;

use aoc2021_rust::util;

#[derive(Debug, Clone)]
struct BinNo(Vec<u8>);

impl BinNo {
    fn into_decimal(self) -> anyhow::Result<usize> {
        usize::from_str_radix(
            self.0
                .into_iter()
                .map(|bit| {
                    char::from_digit(bit as u32, 2)
                        .ok_or_else(|| anyhow::anyhow!("Invalid binary digit!"))
                })
                .collect::<Result<String, _>>()?
                .as_str(),
            2,
        )
        .map_err(|err| anyhow::anyhow!("{}", err))
    }
}

impl str::FromStr for BinNo {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        if input.is_empty() {
            return Err(anyhow::anyhow!("Unexpected empty binary number input!"));
        }

        Ok(BinNo(
            input
                .chars()
                .map(|c| c.to_digit(2))
                .map(|digit| digit.map(|digit| digit as u8))
                .map(|digit| digit.ok_or_else(|| anyhow::anyhow!("Invalid binary digit!")))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn part_one(diagnostic_report: &[BinNo]) -> anyhow::Result<usize> {
    let bin_no_len = diagnostic_report
        .first()
        .map(|report| report.0.len())
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty diagnostic report!"))?;

    let report_len = diagnostic_report.len();

    let mut gamma_rate = BinNo(vec![0; bin_no_len]);
    let mut epsilon_rate = BinNo(vec![0; bin_no_len]);

    (0..bin_no_len).for_each(|i| {
        let num_zeros = diagnostic_report
            .iter()
            .map(|report| report.0[i])
            .filter(|&bit| bit == 0)
            .collect::<Vec<_>>()
            .len();

        if num_zeros > report_len - num_zeros {
            gamma_rate.0[i] = 0;
            epsilon_rate.0[i] = 1;
        } else {
            gamma_rate.0[i] = 1;
            epsilon_rate.0[i] = 0;
        }
    });

    Ok(gamma_rate.into_decimal()? * epsilon_rate.into_decimal()?)
}

fn part_two(diagnostic_report: &[BinNo]) -> anyhow::Result<usize> {
    let bin_no_len = diagnostic_report
        .first()
        .map(|report| report.0.len())
        .ok_or_else(|| anyhow::anyhow!("Unexpected empty diagnostic report!"))?;

    let mut o2_rating = diagnostic_report.iter().cloned().collect::<Vec<_>>();
    let mut co2_rating = diagnostic_report.iter().cloned().collect::<Vec<_>>();

    (0..bin_no_len).for_each(|i| {
        filter_rating(&mut o2_rating, i, true);
        filter_rating(&mut co2_rating, i, false);
    });

    Ok(o2_rating
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No answer found!"))?
        .into_decimal()?
        * co2_rating
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No answer found!"))?
            .into_decimal()?)
}

fn filter_rating(report: &mut Vec<BinNo>, bit_idx: usize, most_common_bit: bool) -> () {
    let report_len = report.len();

    if report_len == 1 {
        return;
    }

    let num_zeros = report
        .iter()
        .map(|report| report.0[bit_idx])
        .filter(|&bit| bit == 0)
        .collect::<Vec<_>>()
        .len();

    let bit = if num_zeros > report_len - num_zeros {
        if most_common_bit {
            0
        } else {
            1
        }
    } else {
        if most_common_bit {
            1
        } else {
            0
        }
    };

    *report = report
        .drain(..)
        .filter(|report| report.0[bit_idx] == bit)
        .collect::<Vec<_>>()
}

fn main() -> anyhow::Result<()> {
    let input = util::read_input::<BinNo>()?;

    println!("Part one: {}", part_one(&input)?);
    println!("Part two: {}", part_two(&input)?);

    Ok(())
}
