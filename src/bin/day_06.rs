use aoc_2021::get_input;

fn boop(numbers: &[u64], steps: u64) -> usize {
    let mut counts = (0..=8)
        .map(|x| numbers.iter().filter(|n| **n == x).count())
        .collect::<Vec<_>>();
    for _ in 0..steps {
        let n = counts.remove(0);
        counts[6] += n;
        counts.push(n);
    }
    counts.iter().sum()
}
fn part1(numbers: &[u64]) -> usize {
    boop(numbers, 80)
}

fn part2(numbers: &[u64]) -> usize {
    boop(numbers, 256)
}

fn main() {
    let numbers = &get_input!(|s| s
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>())[0];

    println!("part1: {}", part1(numbers));
    println!("part2: {}", part2(numbers));
}
