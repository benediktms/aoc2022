use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Copy)]
enum Symbol {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Draw,
    Loose,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl FromStr for Symbol {
    type Err = ();
    fn from_str(s: &str) -> Result<Symbol, Self::Err> {
        match s {
            "A" => Ok(Symbol::Rock),
            "B" => Ok(Symbol::Paper),
            "C" => Ok(Symbol::Scissor),
            "X" => Ok(Symbol::Rock),
            "Y" => Ok(Symbol::Paper),
            "Z" => Ok(Symbol::Scissor),
            _ => Err(()),
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Self::Paper => "Paper".to_string(),
            Self::Rock => "Rock".to_string(),
            Self::Scissor => "Scissor".to_string(),
        }
    }
}

impl ToString for Outcome {
    fn to_string(&self) -> String {
        match self {
            Self::Draw => "Draw".to_string(),
            Self::Loose => "Loose".to_string(),
            Self::Win => "Win".to_string(),
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.to_string();
        let b = &other.to_string();

        a == b
    }
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.to_string();
        let b = &other.to_string();

        a == b
    }
}

fn determine_symbol(oppenent_symbol: &Symbol, desired_outcome: &Outcome) -> Symbol {
    match oppenent_symbol {
        Symbol::Rock => {
            if desired_outcome.eq(&Outcome::Win) {
                Symbol::Paper
            } else if desired_outcome.eq(&Outcome::Draw) {
                Symbol::Rock
            } else {
                Symbol::Scissor
            }
        }
        Symbol::Paper => {
            if desired_outcome.eq(&Outcome::Win) {
                Symbol::Scissor
            } else if desired_outcome.eq(&Outcome::Draw) {
                Symbol::Paper
            } else {
                Symbol::Rock
            }
        }
        Symbol::Scissor => {
            if desired_outcome.eq(&Outcome::Win) {
                Symbol::Rock
            } else if desired_outcome.eq(&Outcome::Draw) {
                Symbol::Scissor
            } else {
                Symbol::Paper
            }
        }
    }
}

fn play_rps(opponent_symbol: &Symbol, your_symbol: &Symbol) -> i32 {
    let mut base: i32 = 0;

    match your_symbol {
        Symbol::Paper => {
            base += 2;
            if opponent_symbol.eq(&Symbol::Scissor) {
                base += 0;
            } else if opponent_symbol.eq(&Symbol::Rock) {
                base += 6
            } else if opponent_symbol.eq(&Symbol::Paper) {
                base += 3
            }
        }
        Symbol::Rock => {
            base += 1;
            if opponent_symbol.eq(&Symbol::Scissor) {
                base += 6;
            } else if opponent_symbol.eq(&Symbol::Rock) {
                base += 3
            } else if opponent_symbol.eq(&Symbol::Paper) {
                base += 0
            }
        }
        Symbol::Scissor => {
            base += 3;
            if opponent_symbol.eq(&Symbol::Scissor) {
                base += 3;
            } else if opponent_symbol.eq(&Symbol::Rock) {
                base += 0
            } else if opponent_symbol.eq(&Symbol::Paper) {
                base += 6
            }
        }
    };

    base
}

fn main() -> anyhow::Result<()> {
    let path = Path::new("input/day2/input1.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut score = 0;
    let mut determined_score = 0;

    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.split_whitespace().collect();
        let split: (&str, &str) = (split[0], split[1]);

        let opponent = Symbol::from_str(split.0).unwrap();
        let you = Symbol::from_str(split.1).unwrap();
        let determined_symbol = determine_symbol(&opponent, &Outcome::from_str(split.1).unwrap());

        determined_score += play_rps(&opponent, &determined_symbol);
        score += play_rps(&opponent, &you);
    }

    assert_eq!(score, 11906);
    assert_eq!(determined_score, 11186);
    println!("score: {}", score);
    println!("determined_score: {}", determined_score);

    Ok(())
}
