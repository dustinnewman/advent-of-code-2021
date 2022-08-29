use std::{collections::HashSet, ops::Range};

fn split_input(x: &str) -> (isize, isize) {
    let (x1, x2) = x.split_once("..").unwrap();
    let (_, x1) = x1.split_once("=").unwrap();
    (x1.parse().unwrap(), x2.parse().unwrap())
}

fn triangular_number(n: isize) -> isize {
    n * (n + 1) / 2
}

fn magic(x: isize, n: usize) -> f32 {
    (x as f32 / (n + 1) as f32) + (n as f32 / 2.0)
}

fn get_range_for_x(n: usize, x1: isize, x2: isize) -> Range<isize> {
    let triangular_number = triangular_number(n as isize);
    let lower = if triangular_number <= x1 {
        magic(x1, n)
    } else {
        (((8 * x1 + 1) as f32).sqrt() - 1 as f32) / 2.0
    };
    let upper = if triangular_number >= x2 {
        (((8 * x2 + 1) as f32).sqrt() - 1 as f32) / 2.0
    } else {
        magic(x2, n)
    };
    Range {
        start: lower.ceil() as isize,
        end: upper.floor() as isize + 1,
    }
}

fn get_range_for_y(n: usize, y1: isize, y2: isize) -> Range<isize> {
    let lower = magic(y1, n);
    let upper = magic(y2, n);
    Range {
        start: lower.ceil() as isize,
        end: upper.floor() as isize + 1,
    }
}

fn part2(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    let mut coordinates: HashSet<(isize, isize)> = HashSet::new();
    let bound = y1.abs() as usize * 2;
    for n in 0..bound {
        let xs = get_range_for_x(n, x1, x2);
        for x in xs {
            let ys = get_range_for_y(n, y1, y2);
            for y in ys {
                coordinates.insert((x, y));
            }
        }
    }
    coordinates.len()
}

fn main() {
    let (x, y) = include_str!("input.txt")
        .split_once(", ")
        .unwrap();
    let (x1, x2) = split_input(x);
    let (y1, y2) = split_input(y);
    println!("Part 1: {}", triangular_number(y1));
    println!("Part 2: {}", part2(x1, y1, x2, y2));
}
