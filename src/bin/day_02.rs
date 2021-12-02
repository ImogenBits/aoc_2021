use std::{
    fs,
    io::{self, BufRead},
};
use itertools::izip;

fn main() -> io::Result<()> {
    let f = fs::File::open("input")?;
    let mut numbers: Vec<u64> = vec![];
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() != 0 {
        numbers.push(buffer.trim().parse().unwrap());
        buffer.clear();
    }
    let iter = izip!(numbers.iter(), numbers.iter().skip(1), numbers.iter().skip(2)).map(|(a, b, c)| a + b + c);
    let iter2 = iter.clone();

    let solution = iter
        .zip(iter2.skip(1))
        .filter(|(a, b)| a < b)
        .count();

    println!("{}", solution);

    Ok(())
}

fn main2() -> io::Result<()> {
    let f = fs::File::open("input")?;
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();
    let mut depth: i64 = 0;
    let mut distance: i64 = 0;
    let mut aim: i64 = 0;

    while reader.read_line(&mut buffer).unwrap() != 0 {
        let mut line_iter = buffer.trim().split(" ");
        let s = line_iter.next().unwrap();
        let n: i64 = line_iter.next().unwrap().parse().unwrap();
        match s {
            "forward" => {
                depth += aim * n;
                distance += n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => panic!(),
        }
        buffer.clear();
    }

    let solution = distance * depth;

    println!("{}", solution);

    Ok(())
}
