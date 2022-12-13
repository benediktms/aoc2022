use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_file(input: &str) -> Lines<BufReader<File>> {
    let path = File::open(input).expect("Cannot read file path");
    let buf_reader = BufReader::new(path);
    buf_reader.lines()
}
