use std::{
    fs,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let f = fs::File::open("input")?;
    let mut numbers: Vec<u64> = vec![];
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() != 0 {
        numbers.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }
    let solution = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("{}", solution);

    Ok(())
}




//test