use aoc_2021::get_input;

fn vec_to_num(v: &[bool]) -> u64 {
    u64::from_str_radix(
        &v.iter()
            .map(|t| if *t { '1' } else { '0' })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn vec_inverse(v: &[bool]) -> Vec<bool> {
    v.iter().map(|t| !t).collect::<Vec<_>>()
}

fn get_most_common(numbers: &[Vec<bool>]) -> Vec<bool> {
    let mut digits = vec![];
    for _ in 0..numbers[0].len() {
        digits.push(vec![])
    }
    for a in numbers {
        for (i, d) in a.iter().enumerate() {
            digits[i].push(*d)
        }
    }

    let total = numbers.len();
    digits
        .iter()
        .map(|column| column.iter().filter(|d| **d).count() * 2 >= total)
        .collect()
}

fn part1(numbers: &[Vec<bool>]) -> u64 {
    let gamma = get_most_common(numbers);
    let eta = vec_to_num(&vec_inverse(&gamma));
    let gamma = vec_to_num(&gamma);
    eta * gamma
}

fn part2(mut numbers: Vec<Vec<bool>>) -> u64 {
    let numbers_copy = numbers.clone();
    let mut i = 0;
    while numbers.len() > 1 {
        let most_common = get_most_common(&numbers);
        numbers = numbers
            .into_iter()
            .filter(|s| s[i] == most_common[i])
            .collect::<Vec<Vec<_>>>();
        i += 1
    }
    let oxygen = vec_to_num(&numbers[0]);

    numbers = numbers_copy;
    let mut i = 0;
    while numbers.len() > 1 {
        let most_common = get_most_common(&numbers);
        let least_common = vec_inverse(&most_common);
        numbers = numbers
            .into_iter()
            .filter(|s| s[i] == least_common[i])
            .collect::<Vec<Vec<_>>>();
        i += 1
    }
    let co2 = vec_to_num(&numbers[0]);

    oxygen * co2
}

fn main() {
    let numbers: Vec<Vec<_>> = get_input!(|s| s.chars().map(|c| c == '1').collect());

    println!("part1: {}", part1(&numbers));
    println!("part2: {}", part2(numbers));
}
