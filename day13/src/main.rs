use std::collections::HashSet;

type Coordinate = (u32, u32);

const FOLD_INDEX: usize = "fold along ".len();

fn fold_dots(dots: &HashSet<Coordinate>, (axis, offset): (char, u32)) -> HashSet<Coordinate> {
    dots
        .iter()
        .map(|&(x, y)| match (axis, x, y) {
            ('x', x, y) if x > offset => (offset * 2 - x, y),
            ('y', x, y) if y > offset => (x, offset * 2 - y),
            _ => (x, y),
        })
        .collect()
}

fn part2(dots: HashSet<Coordinate>, folds: &Vec<(char, u32)>) {
    let codes = folds.iter().fold(dots, |prev, &fold| fold_dots(&prev, fold));
    let mut maxes = (0, 0);
    for &(x, y) in &codes {
        if x > maxes.0 {
            maxes.0 = x;
        }
        if y > maxes.1 {
            maxes.1 = y;
        }
    }
    for y in 0..=maxes.1 {
        for x in 0..=maxes.0 {
            if codes.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let (dots, folds) = include_str!("input.txt")
        .split_once("\n\n")
        .unwrap();
    let dots: HashSet<Coordinate> = dots
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds: Vec<(char, u32)> = folds
        .lines()
        .map(|line| {
            let (axis, offset) = line[FOLD_INDEX..].split_once("=").unwrap();
            (axis.as_bytes()[0] as char, offset.parse().unwrap())
        })
        .collect();
    println!("Part 1: {}", fold_dots(&dots, folds[0]).len());
    part2(dots, &folds);
}
