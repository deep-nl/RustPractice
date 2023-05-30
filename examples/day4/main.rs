use anyhow::{anyhow, bail};
use std::ops::RangeInclusive;

fn main() {
    func1();
    func2();
    func3();
}

pub fn func1() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(',').unwrap();
                let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
                (
                    a.parse::<u8>().unwrap(),
                    b.parse::<u8>().unwrap(),
                    c.parse::<u8>().unwrap(),
                    d.parse::<u8>().unwrap(),
                )
            })
            .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
            .count()
    );
}

pub fn func2() {
    println!(
        "{}",
        include_str!("input.txt")
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(',').unwrap();
                let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
                (
                    a.parse::<u8>().unwrap(),
                    b.parse::<u8>().unwrap(),
                    c.parse::<u8>().unwrap(),
                    d.parse::<u8>().unwrap(),
                )
            })
            .filter(|(a, b, c, d)| a <= d && c <= b)
            .count()
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
    let assignments = input
        .lines()
        .enumerate()
        .map(|(line, pair)| {
            let line = line + 1;
            let ranges = pair
                // 每个小组的两个清扫范围用是 ',' 隔开的
                .split(',')
                .map(|assignment| {
                    let range = assignment
                        // 清扫范围的两个区域 ID 是用 '-' 隔开的
                        .split('-')
                        .map(|id| {
                            id.parse::<usize>()
                                .map_err(|_| anyhow!("Invalid section id `{id}` on line {line}"))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    
                    if range.len() != 2 {
                        bail!("Invalid number of section IDs on line {line}");
                    }
                    
                    // 用 (start..=end) 就可以生成一个 RangeInclusive 
                    Ok(range[0]..=range[1])
                })
                .collect::<Result<Vec<_>, _>>()?;
            if ranges.len() != 2 {
                bail!("Invalid number of ranges on line {line}");
            }
            Ok(ranges)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 有包含关系的任务的数量
    let part1_answer = assignments
        .iter()
        // 过滤出有任务包含关系的小组
        .filter(|ranges| ranges[0].contains_or_is_contained(&ranges[1]))
        .count();

    println!("Part 1 Answer: {}", part1_answer);

    // 有多少个小组的清理任务区域中有重叠？
    let part2_answer = assignments
        .into_iter()
        .filter(|ranges| ranges[0].overlaps(&ranges[1]))
        .count();

    println!("Part 2 Answer: {}", part2_answer);

    Ok(())
}

pub trait RangeInclusiveExt {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains_or_is_contained(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl RangeInclusiveExt for RangeInclusive<usize> {
    fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.end() - self.start() + 1
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains_or_is_contained(&self, other: &Self) -> bool {
        // 大的范围包含小的范围
        if self.len() >= other.len() {
            self.contains(other.start()) && self.contains(other.end())
        } else {
            other.contains(self.start()) && other.contains(self.end())
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }
}
