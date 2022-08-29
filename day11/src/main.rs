use std::collections::HashSet;

type Grid = Vec<Vec<u8>>;
type Coordinate = (usize, usize);

const NEIGHBORS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

fn to_u8(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        _ => panic!("Invalid byte"),
    }
}

fn flash(grid: &mut Grid, set: &mut HashSet<Coordinate>, point@(x, y): Coordinate) {
    // If we are not > 9 or if we have already flashed this step then do not count
    if grid[y][x] < 10 || !set.insert(point) {
        return;
    }
    // To handle type wrangling magic
    let add = |a: usize, b: &isize| ((a as isize + b) as usize);
    // Filter all illegal combinations to prevent integer overlow (unsigned 0 - 1)
    let neighbors = NEIGHBORS
        .iter()
        .filter(|(dx, _)| x as isize + dx >= 0)
        .map(|(dx, dy)| (add(x, dx), add(y, dy)));
    // Go through each neighbor and see if valid. If so, increment due to flash
    // and recurse (we may or may not actually flash this neighbor)
    for (x2, y2) in neighbors {
        // If x or y >= 10 for example
        if grid.get(y2).and_then(|row| row.get(x2)).is_none() {
            continue;
        }
        grid[y2][x2] += 1;
        flash(grid, set, (x2, y2));
    }
}

fn step(grid: &mut Grid) -> usize {
    let mut set = HashSet::new();
    // Increment energy level +1 each step
    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|octopus| *octopus += 1));
    // Do flashing logics
    (0..10).for_each(|y| (0..10).for_each(|x| flash(grid, &mut set, (x, y))));
    // Reset any flashed back to 0
    grid.iter_mut().for_each(|row| row.iter_mut().for_each(|octopus| if *octopus > 9 { *octopus = 0; }));
    // Return number of successful flashes (number of additions to set)
    set.len()
}

fn part1(grid: &mut Grid) -> usize {
    // 100 steps, count each number of flashes and sum up
    (0..100).map(|_| step(grid)).sum()
}

fn part2(grid: &mut Grid)-> usize {
    // First round that we have flashed all 100 octopus will be the round AFTER
    // we count 100 flashes (this is due to implementation)
    // .find() stops after the first match
    (100..).find(|_| step(grid) == 100).unwrap() + 1
}

fn main() {
    let mut grid: Grid = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|line| line.iter().map(|&b| to_u8(b)).collect())
        .collect();
    println!("Part 1: {}", part1(&mut grid));
    println!("Part 2: {}", part2(&mut grid));
}
