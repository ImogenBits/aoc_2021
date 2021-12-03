use aoc_2021::get_input;
use core::panic;
use std::str::FromStr;

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" ").unwrap();
        let b = b.parse().unwrap();
        Ok(match a {
            "forward" => Command::Forward(b),
            "down" => Command::Down(b),
            "up" => Command::Up(b),
            _ => panic!(),
        })
    }
}

fn part1(commands: &[Command]) -> i64 {
    let mut depth = 0;
    let mut distance = 0;

    for c in commands {
        use Command::*;
        match c {
            Forward(d) => distance += d,
            Down(d) => depth += d,
            Up(d) => depth -= d,
        }
    }

    depth * distance
}

fn part2(commands: &[Command]) -> i64 {
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;

    for c in commands {
        use Command::*;
        match c {
            Forward(d) => {
                depth += aim * d;
                distance += d;
            }
            Down(d) => aim += d,
            Up(d) => aim -= d,
        }
    }

    depth * distance
}

fn main() {
    let commands: Vec<Command> = get_input!(|s| s.parse().unwrap());

    println!("part1: {}", part1(&commands));
    println!("part2: {}", part2(&commands));
}
