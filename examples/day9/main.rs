use std::str::FromStr;
use anyhow::anyhow;

use std::collections::HashSet;

fn main() {
    func1();
    func2();
    func3();
    func4();
}

pub fn func1() {
    let cmds = include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut h, mut t, mut s): ((i32, i32), (_, _), rustc_hash::FxHashSet<_>) = Default::default();
    s.insert((0, 0));

    for (d, l) in cmds {
        for _ in 0..l {
            h = (h.0 + d.0, h.1 + d.1);
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                t = (h.0 - d.0, h.1 - d.1);
                s.insert(t);
            }
        }
    }

    println!("{}", s.len());
}

// whole
// An implementation that attempts to process a full move in a single iteration.
// Sadly, this doesn't seem to be any faster.
pub fn func2() {
    let cmds = include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi::<i32>(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut h, mut t, mut seen): ((i32, i32), (i32, i32), HashSet<_>) = Default::default();
    seen.insert((0, 0));

    for (d, l) in cmds {
        h = (h.0 + d.0 * l, h.1 + d.1 * l);
        if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
            let to = (h.0 - d.0, h.1 - d.1);
            while t != to {
                t.0 += (to.0 - t.0).signum();
                t.1 += (to.1 - t.1).signum();
                seen.insert(t);
            }
        }
    }

    println!("{}", seen.len());
}

pub fn func3() {
    let cmds = include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut knots, mut s): ([(i32, i32); 10], rustc_hash::FxHashSet<_>) = Default::default();
    s.insert((0, 0));

    for (d, l) in cmds {
        for _ in 0..l {
            knots[0].0 += d.0;
            knots[0].1 += d.1;

            for i in 1..10 {
                let (h, t) = knots.split_at_mut(i);
                let (h, t) = (h[i - 1], &mut t[0]);
                if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                    let d = (t.0 - h.0, t.1 - h.1);
                    let l = d.0.abs().max(d.1.abs());
                    let m = (d.0 / l, d.1 / l);
                    *t = (h.0 + m.0, h.1 + m.1);
                    (i == 9).then(|| s.insert(*t));
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", s.len());
}

// An implementation that attempts to process a full move in a single iteration, switching to
// step-by-step when the tail moves. Sadly, this doesn't seem to be any faster.
fn func4() {
    let cmds = include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .map(|l| (l[0], atoi::atoi(&l[2..]).unwrap()));

    let mut knots: [(isize, isize); 10] = Default::default();
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    for (d, l) in cmds {
        let d = match d {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };

        let mut tail_moves = false;
        let mut knots_check = knots;
        let h = &mut knots_check[0];
        *h = (h.0 + d.0 * l, h.1 + d.1 * l);
        for i in 1..10 {
            let (h, t) = knots_check.split_at_mut(i);
            let h = &mut h[i - 1];
            let t = &mut t[0];

            if !adjacent(*h, *t) {
                if i == 5 {
                    tail_moves = true;
                    break;
                }

                let d = (t.0 - h.0, t.1 - h.1);
                let l = d.0.abs().max(d.1.abs());
                let m = (d.0 / l, d.1 / l);
                *t = (h.0 + m.0, h.1 + m.1);
            } else {
                break;
            }
        }

        if tail_moves {
            for _ in 0..l {
                let h = &mut knots[0];
                *h = (h.0 + d.0, h.1 + d.1);

                for i in 1..10 {
                    let (h, t) = knots.split_at_mut(i);
                    let h = &mut h[i - 1];
                    let t = &mut t[0];

                    if !adjacent(*h, *t) {
                        let d = (t.0 - h.0, t.1 - h.1);
                        let l = d.0.abs().max(d.1.abs());
                        let m = (d.0 / l, d.1 / l);
                        *t = (h.0 + m.0, h.1 + m.1);
                        if i == 9 {
                            seen.insert(*t);
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            knots = knots_check;
        }
    }

    println!("{}", seen.len());
}

#[inline(always)]
fn adjacent(h: (isize, isize), t: (isize, isize)) -> bool {
    h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1   
}