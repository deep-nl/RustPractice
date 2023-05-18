use std::iter::Peekable;

// for fun2
struct Dir(Vec<Dir>, u64);

fn main() {
    fun1();
    fun2();
}

pub fn fun1() {
    let (d, mut s) = (include_bytes!("./input.txt"), 0);
    sh1(&mut d.split(|b| b == &b'\n').peekable(), &mut s);
    println!("{}", s);
}

fn sh1(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>, sum: &mut u64) -> u64 {
    let mut size = 0;
    while let Some(i) = lines.next() {
        match i {
            b"$ cd .." => break,
            _ if &i[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
                    .filter(|i| i[0] != b'd')
                    .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                    .sum()
            }
            _ => size += sh1(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

// --------------------------------------------------------------------------------------


pub fn fun2() {
    let root = sh2(&mut include_bytes!("./input.txt")
        .split(|b| b == &b'\n')
        .peekable());
    println!("{}", search(&root, root.1 - 40_000_000).unwrap());
}

fn sh2(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>) -> Dir {
    let (mut dirs, mut size) = (vec![], 0);
    while let Some(i) = lines.next() {
        match i {
            b"$ cd .." => break,
            _ if &i[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
                    .filter(|i| i[0] != b'd')
                    .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                    .sum()
            }
            _ => dirs.push(sh2(lines)),
        }
    }
    size += dirs.iter().map(|d| d.1).sum::<u64>();
    Dir(dirs, size)
}

fn search(d: &Dir, min: u64) -> Option<u64> {
    d.0.iter()
        .filter(|d| d.1 >= min)
        .flat_map(|d| [Some(d.1), search(d, min)])
        .flatten()
        .min()
}