use aoc_2021::get_input;






fn part1(input: &[Vec<u32>]) -> usize {
    
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
    let input = get_input!(|s| s.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<_>>());

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
