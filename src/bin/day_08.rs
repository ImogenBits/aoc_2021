use std::collections::HashSet;

use aoc_2021::get_input;
use itertools::Itertools;

fn find_remove(
    input: &mut Vec<HashSet<char>>,
    predicate: impl Fn(&HashSet<char>) -> bool,
) -> HashSet<char> {
    let (i, digit) = input
        .iter()
        .enumerate()
        .find(|(_, s)| predicate(s))
        .unwrap();
    let digit = digit.to_owned();
    input.remove(i);
    digit
}

fn decode(mut input: Vec<HashSet<char>>) -> Vec<HashSet<char>> {
    let mut digits = vec![HashSet::new(); 10];

    digits[1] = find_remove(&mut input, |s| s.len() == 2);
    digits[4] = find_remove(&mut input, |s| s.len() == 4);
    digits[7] = find_remove(&mut input, |s| s.len() == 3);
    digits[8] = find_remove(&mut input, |s| s.len() == 7);

    digits[6] = find_remove(&mut input, |s| {
        s.len() == 6 && s.intersection(&digits[1]).count() < 2
    });
    digits[0] = find_remove(&mut input, |s| {
        s.len() == 6
            && s.intersection(
                &digits[6]
                    .difference(&digits[4])
                    .copied()
                    .collect::<HashSet<_>>(),
            )
            .count()
                == 3
    });
    digits[9] = find_remove(&mut input, |s| s.len() == 6);

    digits[3] = find_remove(&mut input, |s| {
        s.len() == 5 && s.intersection(&digits[1]).count() == 2
    });
    digits[2] = find_remove(&mut input, |s| {
        s.len() == 5
            && s.intersection(
                &digits[8]
                    .difference(&digits[6])
                    .copied()
                    .collect::<HashSet<_>>(),
            )
            .count()
                == 1
    });
    digits[5] = find_remove(&mut input, |_| true);

    digits
}

fn part1(input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    input
        .into_iter()
        .map(|(i, d)| {
            let digits = decode(i);
            d.iter()
                .map(|digit| {
                    digits
                        .iter()
                        .enumerate()
                        .find(|(_, e)| *e == digit)
                        .unwrap()
                        .0
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|n| matches!(n, 1 | 4 | 7 | 8))
        .count()
}

fn part2(input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    input
        .into_iter()
        .map(|(i, d)| {
            let digits = decode(i);
            d.iter()
                .map(|digit| digits.iter().find_position(|e| *e == digit).unwrap().0)
                .fold(0, |acc, digit| 10 * acc + digit)
        })
        .sum()
}

fn main() {
    let input = get_input!(|s| {
        let (d, n) = s.split_once(" | ").unwrap();
        let f = |x: &str| {
            x.split(' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
        };
        (f(d), f(n))
    });

    println!("part1: {}", part1(input.clone()));
    println!("part2: {}", part2(input));
}
