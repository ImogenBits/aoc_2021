use aoc_2021::get_input;
use itertools::izip;

fn part1(numbers: &[u64]) -> usize {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

fn part2(numbers: &[u64]) -> usize {
    let iter = izip!(
        numbers.iter(),
        numbers.iter().skip(1),
        numbers.iter().skip(2)
    )
    .map(|(a, b, c)| a + b + c);
    let iter2 = iter.clone();

    iter.zip(iter2.skip(1)).filter(|(a, b)| a < b).count()
}

fn main() {
    let numbers: Vec<u64> = get_input!(|s| s.parse().unwrap());

    println!("part 1: {}", part1(&numbers));
    println!("part 2: {}", part2(&numbers));
}
