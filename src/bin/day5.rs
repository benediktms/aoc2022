#![feature(iter_next_chunk)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use anyhow::Context;
use itertools::Itertools;

fn read_file(input: &str) -> Lines<BufReader<File>> {
    let path = File::open(input).expect("Cannot read file path");
    let buf_reader = BufReader::new(path);
    buf_reader.lines()
}

fn main() -> anyhow::Result<()> {
    let crates = read_file("input/day5/crates.txt");
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(9);

    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in crates {
        let line = line.context("Failed to load line")?;
        let mut chars = line.chars();

        // https://www.youtube.com/watch?v=qCA8C-1D0T0&t=606s&ab_channel=RitualCoding
        stacks.iter_mut().take(9).for_each(|stack| {
            if let Ok(chunk) = chars.next_chunk::<3>() {
                if chunk[0] == '[' {
                    stack.insert(0, chunk[1])
                }
            }
            chars.next();
        });
    }

    let mut stacks_alt: Vec<Vec<char>> = stacks.clone();

    let instructions = read_file("input/day5/instructions.txt")
        .map(|line| {
            let line = line.context("Can not read line").unwrap();
            let split = line.split(' ').collect_vec();
            (
                split[1].parse::<i32>().unwrap(),
                split[3].parse::<usize>().unwrap() - 1,
                split[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect_vec();

    for line in instructions {
        let (amount, from, to) = line;

        for _ in 0..amount {
            let stack_crate = stacks[from].pop().context("Could not pop element")?;
            stacks[to].push(stack_crate)
        }

        let from_stack = stacks_alt[from].clone();
        let (remaining, to_move) = from_stack.split_at(from_stack.len() - (amount as usize));

        stacks_alt[from] = remaining.to_vec();
        stacks_alt[to].extend_from_slice(to_move);
    }

    let last_crates = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_string())
        .collect_vec()
        .join("");

    let last_crates_alt = stacks_alt
        .iter()
        .map(|s| s.last().unwrap().to_string())
        .collect_vec()
        .join("");

    println!("last crates: {last_crates}",);
    assert_eq!("WCZTHTMPS", last_crates);

    println!("last crates alt: {last_crates_alt}");
    assert_eq!("BLSGJSDTS", last_crates_alt);

    Ok(())
}
