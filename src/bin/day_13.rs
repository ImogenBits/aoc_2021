use aoc_2021::get_input;
use itertools::Itertools;



fn fold(points: &[(usize, usize)], fold : &(char, usize)) -> Vec<(usize, usize)> {
    let (axis, i) = fold;
    points.iter().map(|(x, y)| {
        if *axis == 'y' {
            if y > i {   
                (*x, (*i as isize - (*y as isize - *i as isize)) as usize)
            } else {
                (*x, *y)
            }
        } else {
            if x > i {
                ((*i as isize - (*x as isize - *i as isize)) as usize, *y)
            } else {
                (*x, *y)
            }
        }
    }).unique().collect()
}


fn part1(points: &[(usize, usize)], folds: &[(char, usize)]) -> usize {
    fold(&points, &folds[0]).iter().count()
}

fn part2(mut points: Vec<(usize, usize)>, folds: &[(char, usize)]) {
    for f in folds {
        points = fold(&points, f);
    }
    let x = points.iter().max_by_key(|(x, _)| x).unwrap().0;
    let y = points.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut v = vec![vec![','; x + 1]; y + 1];
    for (x, y) in points {
        v[y][x] = '#';
    }
    let mut out = vec![];
    for row in v {
        out.push(row.iter().collect::<String>());
    }
    println!("{:?}",out);
}

fn main() {
    let input = get_input!(|s| s);
    let mut folds = vec![];
    let mut points = vec![];
    for line in input {
        let line = line.trim();
        if line != "" {
            if line.starts_with("fold along") {
                let mut iter = line.chars().skip(11);
                let axis = iter.next().unwrap();
                let line = iter.skip(1).collect::<String>().parse::<usize>().unwrap();
                folds.push((axis, line));
            } else {
                let mut iter = line.split(',').map(|s| s.parse::<usize>().unwrap());
                points.push((iter.next().unwrap(), iter.next().unwrap()))
            }
        }
    }

    println!("part1: {}", part1(&points, &folds));
    println!("part2: {:?}", part2(points, &folds));
}
