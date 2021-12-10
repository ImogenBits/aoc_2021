use aoc_2021::get_input;

fn get_min(numbers: &[i64], weight: impl Fn(i64) -> i64) -> i64 {
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();
    (*min..*max).map(|p| numbers.iter().map(|x| weight((p - x).abs())).sum::<i64>()).min().unwrap()
}

fn wah(d: i64) -> i64 {
    d * (d+1) / 2
}

fn part1(numbers: &[i64]) -> i64 {
    get_min(numbers, |x| x)
}

fn part2(numbers: &[i64]) -> i64 {
    get_min(numbers, wah)
}

fn main() {
    let numbers = &get_input!(|s| s
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>())[0];
    
    println!("part1: {}", part1(numbers));
    println!("part2: {}", part2(numbers));
}
