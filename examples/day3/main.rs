use std::str::FromStr;
use anyhow::{anyhow, bail};
use std::collections::BTreeSet;

fn main() {
    func1();
    func2();
    func3();
}

pub fn func1() {
    println!(
        "{}",
        include_bytes!("./input.txt")
            .split(|b| *b == b'\n')
            // split at
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| b
                .iter()
                .filter(|b| a.contains(b))
                .map(|b| if *b >= b'a' {
                    (b - b'a') as i16 + 1
                } else {
                    (b - b'A') as i16 + 27
                })
                .next()
                .unwrap())
            .sum::<i16>(),
    );
}

pub fn func2() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|set| set[0]
                .iter()
                .find(|b| set[1].contains(b) && set[2].contains(b))
                .unwrap())
            .map(|b| if *b >= b'a' {
                (b - b'a') as i16 + 1
            } else {
                (b - b'A') as i16 + 27
            })
            .sum::<i16>(),
    );
}

pub fn func3() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}


fn run() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = std::fs::read_to_string(&path)?;
    let rucksacks = input
        .lines()
        .enumerate()
        .map(|(line, items)| {
            let line = line + 1;
            items
                .parse::<Rucksack>()
                .map_err(|e| anyhow!("{} (line: {line})", e))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let part1_answer = rucksacks
        .iter()
        .cloned()
        .map(|rucksack| rucksack.common_item_priority())
        .sum::<u32>();
    println!("Part 1 Answer: {}", part1_answer);
    Ok(())
}


#[derive(Debug, Clone)]
pub struct Rucksack {
    pub first_compartment: Vec<u32>,
    pub second_compartment: Vec<u32>,
}
impl FromStr for Rucksack {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        if len % 2 != 0 {
            bail!("Two compartments have different number of items.");
        }
        let (first, second) = s.split_at(len / 2);
        let first_compartment = first
            .chars()
            .map(item_type_to_priority)
            .collect::<Result<Vec<_>, _>>()?;
        let second_compartment = second
            .chars()
            .map(item_type_to_priority)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            first_compartment,
            second_compartment,
        })
    }
}
pub fn item_type_to_priority(item_type: char) -> anyhow::Result<u32> {
    match item_type {
        'A'..='Z' => Ok((item_type as u8 - b'A') as u32 + 27),
        'a'..='z' => Ok((item_type as u8 - b'a') as u32 + 1),
        _ => bail!("Invalid item type `{item_type}`."),
    }
}

impl Rucksack {
    pub fn common_item_priority(&self) -> u32 {
        let first = BTreeSet::from_iter(self.first_compartment.iter());
        let second = BTreeSet::from_iter(self.second_compartment.iter());
        first.intersection(&second).copied().sum::<u32>()
    }
}