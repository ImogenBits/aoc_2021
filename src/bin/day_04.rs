use aoc_2021::get_input;

fn is_solved(square: &[Vec<(u64, bool)>]) -> bool {
    square.iter().any(|v| v.iter().all(|(_, b)| *b)) || {
        let mut res = false;
        for i in 0..square.len() {
            let mut column = true;
            for j in 0..square.len() {
                column &= square[j][i].1
            }
            res |= column
        }
        res
    }
}

fn part1(mut squares: Vec<Vec<Vec<(u64, bool)>>>, numbers: &[u64]) -> u64 {
    for n in numbers {
        for square in &mut squares {
            for row in square {
                for (x, b) in row {
                    if x == n {
                        *b = true;
                    }
                }
            }
        }
        let solved = squares
            .iter()
            .filter(|s| is_solved(s))
            .map(|s| {
                s.iter()
                    .map(|r| r.iter().filter(|(_, b)| !*b).map(|(n, _)| n).sum::<u64>())
                    .sum::<u64>()
            })
            .collect::<Vec<_>>();
        if !solved.is_empty() {
            return solved[0] * n;
        }
    }
    0
}

fn part2(mut squares: Vec<Vec<Vec<(u64, bool)>>>, numbers: &[u64]) -> u64 {
    for n in numbers {
        for square in &mut squares {
            for row in square {
                for (x, b) in row {
                    if x == n {
                        *b = true;
                    }
                }
            }
        }
        if squares.len() > 1 {
            squares.retain(|s| !is_solved(s));
        } else {
            if is_solved(&squares[0]) {
                return n
                    * (squares[0]
                        .iter()
                        .map(|r| r.iter().filter(|(_, b)| !*b).map(|(n, _)| n).sum::<u64>())
                        .sum::<u64>());
            }
        }
    }
    0
}

fn main() {
    let mut input = get_input!(|s| s);
    let numbers = input
        .remove(0)
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut squares: Vec<Vec<Vec<(u64, bool)>>> = vec![];
    for s in input {
        if s == "" {
            squares.push(vec![]);
        } else {
            let i = squares.len() - 1;
            squares[i].push(
                s.split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| (s.parse::<u64>().unwrap(), false))
                    .collect::<Vec<_>>(),
            );
        }
    }

    println!("part1: {}", part1(squares.clone(), &numbers));
    println!("part2: {}", part2(squares.clone(), &numbers));
}
