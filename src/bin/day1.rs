use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> anyhow::Result<()> {
    let path = Path::new("input/day1/input1.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);

    let mut calories = vec![0];

    for line in buf_reader.lines() {
        let line = line?;

        if line.is_empty() {
            calories.push(0);
        } else {
            let val = line.parse::<i32>()?;
            let prev = calories.last_mut().unwrap();
            *prev += val;
        }
    }

    let largest = calories.iter().max().unwrap();
    println!("largest number: {largest}");

    assert_eq!(*largest, 67633);

    calories.sort_by(|a, b| b.cmp(a));

    assert_eq!(calories[..3], [67633, 66296, 65699]);

    // todo find total of sorted
    let total = calories.iter().take(3).sum::<i32>();
    println!("total of largest 3: {total}");

    Ok(())
}
