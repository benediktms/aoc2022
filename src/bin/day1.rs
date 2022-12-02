use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> std::io::Result<()> {
    let path = Path::new("input/input1.txt");
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut calories = vec![0];

    for line in lines {
        let line = line?;

        match line.parse::<i32>() {
            Ok(val) => {
                let prev = calories.last_mut().unwrap();
                *prev += val;
            }
            Err(_) => calories.push(0),
        }
    }

    let largest = calories.iter().max().unwrap();
    println!("largest number: {}", largest);

    assert_eq!(*largest, 67633);

    Ok(())
}
