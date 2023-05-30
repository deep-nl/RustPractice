const NEXT: [(usize, usize); 4] = [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];

fn main() {
    func1();
    func2();
    func3();
}

pub fn func1() {
    let data = include_str!("./input.txt");
    let mut map: Vec<_> = data
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let w = data.bytes().position(|b| b == b'\n').unwrap();
    let h = map.len() / w;
    let mut start = data.bytes().position(|b| b == b'S').unwrap();
    let mut end = data.bytes().position(|b| b == b'E').unwrap();
    (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

    println!(
        "{}",
        // pathfing
        pathfinding::directed::bfs::bfs(
            &(end % w, end / w),
            |(x, y)| {
                let cur = map[y * w + x];
                NEXT.iter()
                    .map(|(xx, yy)| (x.wrapping_add(*xx), y.wrapping_add(*yy)))
                    .filter(|(x, y)| x < &w && y < &h && map[y * w + x] >= cur.saturating_sub(1))
                    .collect::<Vec<_>>()
            },
            |&p| p == (start % w, start / w),
        )
        .unwrap()
        .len()
            - 1
    );
}

pub fn func2() {
    let data = include_str!("./input.txt");
    let mut map: Vec<_> = data
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let w = data.bytes().position(|b| b == b'\n').unwrap();
    let h = map.len() / w;
    let mut start = data.bytes().position(|b| b == b'S').unwrap();
    let mut end = data.bytes().position(|b| b == b'E').unwrap();
    (start, end, map[start], map[end]) = (start - start / (w + 1), end - end / (w + 1), 0, 25);

    println!(
        "{}",
        map.iter()
            .enumerate()
            .filter(|(_, b)| **b == 0)
            .filter_map(|(start, _)| pathfinding::directed::bfs::bfs(
                &(start % w, start / w),
                |(x, y)| {
                    let cur = map[y * w + x];
                    NEXT.iter()
                        .map(|(xx, yy)| (x.wrapping_add(*xx), y.wrapping_add(*yy)))
                        .filter(|(x, y)| x < &w && y < &h && map[y * w + x] <= cur + 1)
                        .collect::<Vec<_>>()
                },
                |&p| p == (end % w, end / w),
            )
            .map(|r| r.len() - 1))
            .min()
            .unwrap()
    );
}

pub fn func3() {
    todo!()
}
