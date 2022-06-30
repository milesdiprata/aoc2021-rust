extern crate anyhow;

use std::collections;
use std::convert;
use std::env;

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

enum TypeId {
    Sum = 0,
    Product,
    Min,
    Max,
    LiteralVal,
    GtThan,
    LeThan,
    EqTo,
}

enum LenTypeId {
    TotalLen = 0,
    NumSubPkt,
}

struct Hdr {
    ver: u8,
    type_id: TypeId,
}

struct Pkt {
    hdr: Hdr,
    literal_val: Option<usize>,
    sub_pkts: Option<Vec<Pkt>>,
}

impl convert::TryFrom<u32> for TypeId {
    type Error = anyhow::Error;

    fn try_from(val: u32) -> anyhow::Result<Self> {
        match val {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Min),
            3 => Ok(Self::Max),
            4 => Ok(Self::LiteralVal),
            5 => Ok(Self::GtThan),
            6 => Ok(Self::LeThan),
            7 => Ok(Self::EqTo),
            _ => Err(anyhow::anyhow!("Unknown type ID!")),
        }
    }
}

impl convert::TryFrom<u32> for LenTypeId {
    type Error = anyhow::Error;

    fn try_from(val: u32) -> anyhow::Result<Self> {
        match val {
            0 => Ok(Self::TotalLen),
            1 => Ok(Self::NumSubPkt),
            _ => Err(anyhow::anyhow!("Unknown length type ID!")),
        }
    }
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
            Some(type_id) => TypeId::try_from(u32::from_str_radix(&type_id, 2)?)?,
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
            TypeId::LiteralVal => pkt.literal_val = Some(Self::parse_literal_val(bin)?),
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

            literal_val.push_str(&group);

            if group_id == '0' {
                break;
            }
        }

        let literal_val = usize::from_str_radix(&literal_val, 2)?;

        Ok(literal_val)
    }

    fn parse_op(bin: &mut collections::VecDeque<char>) -> anyhow::Result<Vec<Pkt>> {
        let len_type_id = LenTypeId::try_from(
            bin.pop_front()
                .ok_or_else(|| anyhow::anyhow!(anyhow::anyhow!("Missing length type ID bit!")))?
                .to_digit(2)
                .ok_or_else(|| anyhow::anyhow!("Failed to parse length type ID!"))?,
        )?;

        match len_type_id {
            LenTypeId::TotalLen => Self::parse_total_len_op(bin),
            LenTypeId::NumSubPkt => Self::parse_num_sub_op(bin),
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

    fn eval(&self) -> Option<usize> {
        if let Some(literal_val) = self.literal_val {
            Some(literal_val)
        } else if let Some(sub_pkts) = &self.sub_pkts {
            let sub_pkts_val = sub_pkts
                .iter()
                .map(|pkt| pkt.eval())
                .collect::<Option<Vec<_>>>()?;

            let iter = sub_pkts_val.iter();

            match self.hdr.type_id {
                TypeId::Sum => Some(iter.sum()),
                TypeId::Product => Some(iter.product()),
                TypeId::Min => iter.min().copied(),
                TypeId::Max => iter.max().copied(),
                TypeId::LiteralVal => None,
                TypeId::GtThan => match sub_pkts_val[0] > sub_pkts_val[1] {
                    true => Some(1),
                    false => Some(0),
                },
                TypeId::LeThan => match sub_pkts_val[0] < sub_pkts_val[1] {
                    true => Some(1),
                    false => Some(0),
                },
                TypeId::EqTo => match sub_pkts_val[0] == sub_pkts_val[1] {
                    true => Some(1),
                    false => Some(0),
                },
            }
        } else {
            None
        }
    }
}

fn part_one(pkt: &Pkt) -> anyhow::Result<usize> {
    let mut pkts = vec![pkt];
    let mut sum = 0;

    while let Some(pkt) = pkts.pop() {
        sum += pkt.hdr.ver as usize;

        if let Some(sub_pkts) = &pkt.sub_pkts {
            pkts.extend(sub_pkts.iter());
        }
    }

    Ok(sum)
}

fn part_two(pkt: &Pkt) -> anyhow::Result<usize> {
    let val = pkt.eval().unwrap();

    Ok(val)
}

fn main() -> anyhow::Result<()> {
    let mut bin = env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Hexadecimal input missing!"))?
        .chars()
        .map(hex_to_bin)
        .map(|bin| {
            bin.ok_or_else(|| anyhow::anyhow!("Failed to parse hexadecimal input to binary!"))
        })
        .collect::<Result<String, _>>()?
        .chars()
        .collect::<collections::VecDeque<_>>();

    let pkt = Pkt::from_bin(&mut bin)?.ok_or_else(|| anyhow::anyhow!("No packet found!"))?;

    println!("Part one: {}", part_one(&pkt)?);
    println!("Part two: {}", part_two(&pkt)?);

    Ok(())
}
