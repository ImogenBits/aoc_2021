use aoc_2021::get_input;

fn part1(coords: &[Vec<isize>]) -> usize {
    let coords = coords
        .iter()
        .cloned()
        .filter(|v| v[0] == v[2] || v[1] == v[3])
        .collect::<Vec<_>>();
    part2(&coords)
}

fn signum(x: isize) -> isize {
    match x.cmp(&0) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        _ => 0,
    }
}

fn part2(coords: &[Vec<isize>]) -> usize {
    let max_x = coords.iter().map(|v| v[0].max(v[2])).max().unwrap();
    let max_y = coords.iter().map(|v| v[1].max(v[3])).max().unwrap();
    let mut points = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];

    for v in coords {
        let dx = signum(v[2] - v[0]);
        let dy = signum(v[3] - v[1]);
        let distance = (v[3] - v[1]).abs().max((v[2] - v[0]).abs());
        for i in 0..=distance {
            points[(v[1] + i * dy) as usize][(v[0] + i * dx) as usize] += 1
        }
    }
    points
        .into_iter()
        .map(|v| v.into_iter().filter(|p| *p >= 2).count())
        .sum()
}

fn main() {
    let lines = get_input!(|s| s
        .split(" -> ")
        .flat_map(|s| s.split(','))
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>());

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}
