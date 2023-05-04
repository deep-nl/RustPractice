use std::str::FromStr;
use anyhow::anyhow;

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
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b + 3 * (1 + b - a).rem_euclid(3))
            .sum::<i16>(),
    );
}

pub fn func2() {
    let _input =  include_bytes!("./input.txt")
            .split(|b| *b == b'\n')
            // TODO: how to print just 1 row
            .inspect(|x|println!("print first element content {:?}",x[0]))
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
            .sum::<i16>();

    println!(
        "{}",
        _input
    );
}

// -----------------------------------------------------------------------------------------------------------
// ----------------------------------------match pattern------------------------------------------------------
// -----------------------------------------------------------------------------------------------------------
// https://zhuanlan.zhihu.com/p/590709577


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
    let rounds = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Round>, _>>()?;
    let part1_score = rounds.into_iter().map(|round| round.score()).sum::<u32>();

    let strategies = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Strategy>, _>>()?;
    let part2_score = strategies
        .into_iter()
        .map(|strategy| strategy.score())
        .sum::<u32>();

    println!("Part 1 Answer: {:?}", part1_score);
    println!("Part 2 Answer: {:?}", part2_score);

    Ok(())
}


pub struct Round {
    pub elves_move: Move,
    pub your_move: Move,
}

impl Round {
    pub fn score(&self) -> u32 {
        let round_result = self.your_move.play(&self.elves_move);
        self.your_move.score() + round_result.score()
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(' ');
        let elves_move = moves
            .next()
            .ok_or_else(invalid_number_of_moves_error)?
            .parse()?;
        let your_move = moves
            .next()
            .ok_or_else(invalid_number_of_moves_error)?
            .parse()?;

        Ok(Round {
            your_move,
            elves_move,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {

    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn play(&self, other: &Move) -> RoundResult {
        match (self, other) {
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => RoundResult::Win,

            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => RoundResult::Draw,

            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => RoundResult::Loss,
        }
    }

    fn win_move(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn loss_move(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn draw_move(&self) -> Move {
        *self
    }

    pub fn expected_move(&self, round_result: RoundResult) -> Move {
        match round_result {
            RoundResult::Win => self.win_move(),
            RoundResult::Draw => self.draw_move(),
            RoundResult::Loss => self.loss_move(),
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(anyhow!("Invalid move: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum RoundResult {
    Win,
    Draw,
    Loss,
}

fn invalid_number_of_moves_error() -> anyhow::Error {
    anyhow!("Invalid number of moves")
}

impl FromStr for RoundResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(anyhow!("Invalid round result: {}", s)),
        }
    }
}

impl RoundResult {
    pub fn score(&self) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
}

impl FromStr for Strategy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(' ');

        let elves_move = moves
            .next()
            .ok_or_else(invalid_number_of_moves_error)?
            .parse()?;

        let round_result = moves
            .next()
            .ok_or_else(invalid_number_of_moves_error)?
            .parse()?;

        Ok(Self {
            elves_move,
            round_result,
        })
    }
}

pub struct Strategy {
    pub elves_move: Move,
    pub round_result: RoundResult,
}

impl Strategy {
    pub fn score(&self) -> u32 {
        let your_move = self.elves_move.expected_move(self.round_result);
        your_move.score() + self.round_result.score()
    }
}