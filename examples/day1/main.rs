use anyhow::{Context, bail};


pub fn main() {
    func1();
    func2();
    func3();
}

// method 1:
fn func1() {
    println!(
        "Method1's answer is {}",
        include_str!("./input.txt")
            .split("\n\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap(),
    );
}

// method 2:
pub fn func2() {
    let mut cals = include_str!("./input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    // sort
    cals.sort_unstable();
    println!("Method2's answer is {}", cals.into_iter().rev().take(1).sum::<u32>());
}

// method 3: more complex
fn func3() {
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
    let part_1 = MyStruct.solve(&input)?;
    println!("Method3's Answer: {part_1}");
    Ok(())
}

pub struct ItemsCalories<'a, I, E> {
    items: I,
    // 外部用来记录错误的变量的可变引用
    error: &'a mut Result<(), E>,
}

impl<'a, I, E> ItemsCalories<'a, I, E>
where
    I: Iterator<Item = Result<Option<u64>, E>>,
{
    pub fn new(items: I, error: &'a mut Result<(), E>) -> Self {
        Self { items, error }
    }
}

impl<'a, I, E> Iterator for ItemsCalories<'a, I, E>
where
    I: Iterator<Item = Result<Option<u64>, E>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // 跳过空行
        let mut items = (&mut self.items)
            .skip_while(|calories| matches!(calories, Ok(None)));
        let mut sum = None;
        loop {
            match items.next() {
                Some(Ok(Some(calories))) => {
                    sum = Some(sum.unwrap_or(0) + calories);
                }
                Some(Err(e)) => {
                    // 有错误时，记录下错误
                    *self.error = Err(e);
                    break;
                }
                _ => break,
            };
        }
        sum
    }
}

pub struct MyStruct;

impl Solution for MyStruct {
    type Output = u64;
    fn solve(&self, input: &str) -> anyhow::Result<Self::Output> {
        let lines = input.lines().enumerate().map(|(line, text)| {
            if text.is_empty() {
                Ok(None)
            } else {
                text.parse::<u64>().map(Some).context(format!(
                    "invalid number `{}` on line {}",
                    text,
                    line + 1
                ))
            }
        });

        // 初始化错误
        let mut err = Ok(());
        let items_calories = ItemsCalories::new(lines, &mut err);
        let max_calories = items_calories.max();
        err.map(|_| max_calories.unwrap())
    }
}

pub trait Solution {
    type Output;
    fn solve(&self, input: &str) -> anyhow::Result<Self::Output>;
}