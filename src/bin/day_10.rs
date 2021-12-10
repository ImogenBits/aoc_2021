use aoc_2021::get_input;

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '_',
    }
}

fn run_pda(input: &str, score_type: bool) -> u64 {
    let mut stack = vec![];
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(closing(c)),
            _ => {
                if let Some(n) = stack.pop() {
                    if c != n {
                        return if score_type {
                            match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => 0,
                            }
                        } else {
                            0
                        };
                    }
                } else {
                    panic!();
                }
            }
        }
    }
    if !score_type {
        stack.iter().rev().fold(0, |acc, c| {
            acc * 5
                + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        })
    } else {
        0
    }
}

fn part1(input: &[String]) -> u64 {
    input.iter().map(|s| run_pda(s, true)).sum()
}

fn part2(input: &[String]) -> u64 {
    let mut res = input
        .iter()
        .map(|s| run_pda(s, false))
        .filter(|n| *n != 0)
        .collect::<Vec<_>>();
    res.sort_unstable();
    res[res.len() / 2]
}

fn main() {
    let input = get_input!(|s| s);

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
