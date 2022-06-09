extern crate anyhow;

use std::collections;
use std::env;

const LITERAL_VAL_TYPE_ID: u8 = 4;

const TOTAL_LEN_LEN_TYPE_ID: u8 = 0;
const NUM_SUB_LEN_TYPE_ID: u8 = 1;

const fn hex_to_bin(hex: char) -> Option<&'static str> {
    match hex {
        '0' => Some("0000"),
        '1' => Some("0001"),
        '2' => Some("0010"),
        '3' => Some("0011"),
        '4' => Some("0100"),
        '5' => Some("0101"),
        '6' => Some("0110"),
        '7' => Some("0111"),
        '8' => Some("1000"),
        '9' => Some("1001"),
        'A' => Some("1010"),
        'B' => Some("1011"),
        'C' => Some("1100"),
        'D' => Some("1101"),
        'E' => Some("1110"),
        'F' => Some("1111"),
        _ => None,
    }
}

struct Hdr {
    ver: u8,
    type_id: u8,
}

struct Pkt {
    hdr: Hdr,
    literal_val: Option<usize>,
    sub_pkts: Option<Vec<Pkt>>,
}

impl Hdr {
    const VER_LEN: usize = 3;
    const TYPE_ID_LEN: usize = 3;

    fn from_bin(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Option<Self>> {
        let ver = match (0..Self::VER_LEN)
            .map(|_| bin.pop_front())
            .collect::<Option<String>>()
        {
            Some(ver) => u8::from_str_radix(&ver, 2)?,
            None => return Ok(None),
        };

        let type_id = match (0..Self::TYPE_ID_LEN)
            .map(|_| bin.pop_front())
            .collect::<Option<String>>()
        {
            Some(type_id) => u8::from_str_radix(&type_id, 2)?,
            None => return Ok(None),
        };

        Ok(Some(Self { ver, type_id }))
    }
}

impl Pkt {
    const LITERAL_VAL_GROUP_LEN: usize = 4;

    const TOTAL_LEN: usize = 15;
    const NUM_SUB: usize = 11;

    fn from_bin(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Option<Self>> {
        let mut pkt = match Hdr::from_bin(bin)? {
            Some(hdr) => Self {
                hdr,
                literal_val: None,
                sub_pkts: None,
            },
            None => return Ok(None),
        };

        match pkt.hdr.type_id {
            LITERAL_VAL_TYPE_ID => pkt.literal_val = Some(Self::parse_literal_val(bin)?),
            _ => pkt.sub_pkts = Some(Self::parse_op(bin)?),
        };

        Ok(Some(pkt))
    }

    fn parse_literal_val(bin: &mut collections::VecDeque<char>) -> anyhow::Result<usize> {
        let mut literal_val = String::new();

        loop {
            let group_id = bin
                .pop_front()
                .ok_or_else(|| anyhow::anyhow!("Missing literal value group bit!"))?;

            let group = (0..Self::LITERAL_VAL_GROUP_LEN)
                .map(|_| bin.pop_front())
                .map(|bit| {
                    bit.ok_or_else(|| {
                        anyhow::anyhow!(anyhow::anyhow!("Missing literal value bits!"))
                    })
                })
                .collect::<Result<String, _>>()?;

            literal_val.extend(group.chars());

            if group_id == '0' {
                break;
            }
        }

        let literal_val = usize::from_str_radix(&literal_val, 2)?;

        Ok(literal_val)
    }

    fn parse_op(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Vec<Pkt>> {
        let len_type_id = bin
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!(anyhow::anyhow!("Missing length type ID bit!")))?
            .to_digit(2)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse length type ID!"))?;

        match len_type_id as u8 {
            TOTAL_LEN_LEN_TYPE_ID => Self::parse_total_len_op(bin),
            NUM_SUB_LEN_TYPE_ID => Self::parse_num_sub_op(bin),
            _ => Err(anyhow::anyhow!("Unknown length type ID!")),
        }
    }

    fn parse_total_len_op(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Vec<Pkt>> {
        let mut pkts = vec![];

        let total_len = usize::from_str_radix(
            &(0..Self::TOTAL_LEN)
                .map(|_| bin.pop_front())
                .map(|bin| bin.ok_or_else(|| anyhow::anyhow!("Missing total length bits!")))
                .collect::<Result<String, _>>()?,
            2,
        )?;

        let mut new_bin = (0..total_len)
            .map(|_| bin.pop_front())
            .map(|bin| bin.ok_or_else(|| anyhow::anyhow!("Missing total length sub-packet bits!")))
            .collect::<Result<collections::VecDeque<_>, _>>()?;

        while let Some(pkt) = Pkt::from_bin(&mut new_bin)? {
            pkts.push(pkt);
        }

        Ok(pkts)
    }

    fn parse_num_sub_op(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Vec<Pkt>> {
        let num_sub = usize::from_str_radix(
            &(0..Self::NUM_SUB)
                .map(|_| bin.pop_front())
                .map(|bin| {
                    bin.ok_or_else(|| anyhow::anyhow!("Missing number of sub-packets bits!"))
                })
                .collect::<Result<String, _>>()?,
            2,
        )?;

        let pkts = (0..num_sub)
            .map(|_| Pkt::from_bin(bin))
            .map(|pkt| {
                pkt.map(|pkt| {
                    pkt.ok_or_else(|| {
                        anyhow::anyhow!("Missing number of sub-packets sub-packet bits")
                    })
                })
            })
            .collect::<Result<Result<Vec<_>, _>, _>>()??;

        Ok(pkts)
    }
}

fn part_one(bin: &mut collections::VecDeque<char>) -> anyhow::Result<usize> {
    let mut pkts = vec![];

    while let Some(pkt) = Pkt::from_bin(bin)? {
        pkts.push(pkt);
    }

    let mut sum = 0;

    while let Some(pkt) = pkts.pop() {
        sum += pkt.hdr.ver as usize;

        if let Some(sub_pkts) = pkt.sub_pkts {
            pkts.extend(sub_pkts.into_iter());
        }
    }

    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let mut bin = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| anyhow::anyhow!("Hexadecimal input missing!"))?
        .chars()
        .map(|hex| hex_to_bin(hex))
        .map(|bin| {
            bin.ok_or_else(|| anyhow::anyhow!("Failed to parse hexadecimal input to binary!"))
        })
        .collect::<Result<String, _>>()?
        .chars()
        .collect::<collections::VecDeque<_>>();

    println!("Part one: {}", part_one(&mut bin)?);

    Ok(())
}
