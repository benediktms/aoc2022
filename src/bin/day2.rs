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

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.to_string();
        let b = &other.to_string();

        a == b
    }
}

fn play_rps(opponent_symbol: &str, your_symbol: &str) -> i32 {
    let oppenent_symbol = Symbol::from_str(opponent_symbol).unwrap();
    let your_symbol = Symbol::from_str(your_symbol).unwrap();

    let mut base: i32 = 0;

    match your_symbol {
        Symbol::Paper => {
            base += 2;
            if oppenent_symbol.eq(&Symbol::Scissor) {
                base += 0;
            } else if oppenent_symbol.eq(&Symbol::Rock) {
                base += 6
            } else if oppenent_symbol.eq(&Symbol::Paper) {
                base += 1
            }
        }
        Symbol::Rock => {
            base += 1;
            if oppenent_symbol.eq(&Symbol::Scissor) {
                base += 6;
            } else if oppenent_symbol.eq(&Symbol::Rock) {
                base += 1
            } else if oppenent_symbol.eq(&Symbol::Paper) {
                base += 0
            }
        }
        Symbol::Scissor => {
            base += 3;
            if oppenent_symbol.eq(&Symbol::Scissor) {
                base += 1;
            } else if oppenent_symbol.eq(&Symbol::Rock) {
                base += 0
            } else if oppenent_symbol.eq(&Symbol::Paper) {
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

    for line in lines {
        let line = line?;
        let split: Vec<&str> = line.split_whitespace().collect();
        let split: (&str, &str) = (split[0], split[1]);

        let oppenent = split.0;
        let you = split.1;

        score += play_rps(oppenent, you);
    }

    println!("score: {}", score);

    Ok(())
}
