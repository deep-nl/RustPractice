use std::str::FromStr;
use anyhow::anyhow;

fn main() {
    func1();
    func2();
    func3();
}

pub fn func1() {
    let d = include_bytes!("./input.txt");
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    let g: Vec<&[u8]> = d.split(|b| b == &b'\n').filter(|l| !l.is_empty()).collect();
    // rustc hash
    let mut seen = rustc_hash::FxHashSet::default();

    for i in 1..s - 1 {
        seen.extend(
            (1..s - 1)
                .scan(g[0][i], |max, y| match (*max, g[y][i]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(i + y * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[s - 1][i], |max, y| match (*max, g[s - y][i]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(i + (s - y) * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[i][0], |max, x| match (*max, g[i][x]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(x + i * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
        seen.extend(
            (1..s - 1)
                .scan(g[i][s - 1], |max, x| match (*max, g[i][s - x]) {
                    (m, n) if n > m => {
                        *max = n;
                        Some(Some(s - x + i * s))
                    }
                    (m, _) if m >= b'9' => None,
                    _ => Some(None),
                })
                .flatten(),
        );
    }

    println!("{}", seen.len() + ((s - 1) * 4));
}

pub fn func2() {
    let d = include_bytes!("./input.txt");
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    let g: Vec<&[u8]> = d.split(|b| b == &b'\n').filter(|l| !l.is_empty()).collect();

    let mut max = 0;
    for x in 0..s {
        for y in 0..s {
            let cur = g[y][x];
            if cur <= 1 {
                continue;
            }
            max = max.max(
                ((1..y)
                    .map(|yy| g[y - yy][x])
                    .position(|h| h >= cur)
                    .unwrap_or_else(|| y.wrapping_sub(1))
                    .wrapping_add(1))
                    * ((y + 1..s)
                        .map(|y| g[y][x])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| (s - y).wrapping_sub(2))
                        .wrapping_add(1))
                    * ((1..x)
                        .map(|xx| g[y][x - xx])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| x.wrapping_sub(1))
                        .wrapping_add(1))
                    * ((x + 1..s)
                        .map(|x| g[y][x])
                        .position(|h| h >= cur)
                        .unwrap_or_else(|| (s - x).wrapping_sub(2))
                        .wrapping_add(1)),
            );
        }
    }

    println!("{}", max);
}

// Implementation querying raw input, doesn't seem to be faster.
pub fn func3() {
    let d = include_bytes!("./input.txt");
    let s = d.iter().position(|b| b == &b'\n').unwrap();
    println!(
        "{}",
        (0..s * s)
            .map(|i| {
                let icur = i + i / s;
                let c = d[icur];
                if c <= b'1' {
                    return 0;
                }

                ((1..(i / s))
                    .map(|ii| d[icur - ii * (s + 1)])
                    .position(|h| h >= c)
                    .unwrap_or_else(|| (i / s).wrapping_sub(1))
                    .wrapping_add(1))
                    * ((1..s - (i / s))
                        .map(|ii| d[icur + ii * (s + 1)])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (s - (i / s)).wrapping_sub(2))
                        .wrapping_add(1))
                    * ((1..(i % s))
                        .map(|ii| d[icur - ii])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (i % s).wrapping_sub(1))
                        .wrapping_add(1))
                    * ((1..s - (i % s))
                        .map(|ii| d[icur + ii])
                        .position(|h| h >= c)
                        .unwrap_or_else(|| (s - (i % s)).wrapping_sub(2))
                        .wrapping_add(1))
            })
            .max()
            .unwrap()
    );
}
