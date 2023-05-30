use std::str::FromStr;
use anyhow::anyhow;
use std::collections::HashSet;

fn main() {
    func1();
    func2();
    func3();
}

// usage of any
pub fn func1() {
    println!(
        "{}",
        include_bytes!("input.txt")
            .windows(4)
            .position(|b| !(0..3).any(|i| (i + 1..4).any(|j| b[i] == b[j])))
            .unwrap()
            + 4,
    );
}


pub fn func2() {
    let mut i = 0;
    while let Some(win) = include_bytes!("input.txt").get(i..i + 14) {
        let mut seen = 0u32;
        if let Some(pos) = win.iter().rposition(|b| {
            let bit = 1 << (b % 32);
            let duplicate = seen & bit != 0;
            seen |= bit;
            duplicate
        }) {
            i += pos + 1;
        } else {
            break;
        }
    }

    println!("{}", i);
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
    let signals = input.trim().chars().collect::<Vec<_>>();
    //在检测到第一个数据包开始标记之前需要处理多少个字符？
    let marker = find_marker(&signals, 4);
    println!("Part 1 Answer: {}", marker.unwrap());
    //在检测到第一个消息开始标记之前需要处理多少个字符？
    let marker = find_marker(&signals, 14);
    println!("Part 2 Answer: {}", marker.unwrap());
    Ok(())
}

// usage of all
fn find_marker(signals: &[char], size: usize) -> Option<usize> {
    signals
        .windows(size)
        .position(|window| {
            window
                .iter()
                .take(window.len() - 1)
                .enumerate()
                .all(|(i, c)| !window[i + 1..].contains(c))
        })
        .map(|pos| pos + size)
}

fn find_marker_(signals: &[char], size: usize) -> Option<usize> {
    signals
        .windows(size)
        .position(|window| {
            window.iter().collect::<HashSet<_>>().len() == window.len()
        })
        .map(|pos| pos + size)
}