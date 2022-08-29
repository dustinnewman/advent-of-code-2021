fn absolute_difference(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn gauss(x: u32) -> u32 {
    x * (x + 1) / 2
}

fn part1(positions: &Vec<u32>) -> u32 {
    // Calculate median
    let len = positions.len();
    let mid = len / 2;
    let median = if len % 2 == 0 {
        (positions[mid - 1] + positions[mid]) / 2
    } else {
        positions[mid]
    };
    // Each position's absolute difference from median
    positions
        .iter()
        .fold(0u32, |acc, &curr| acc + absolute_difference(curr, median))
}

fn part2(positions: &Vec<u32>) -> u32 {
    let mean = positions.iter().sum::<u32>() / positions.len() as u32;
    positions
        .iter()
        .fold(0u32, |acc, &p| acc + gauss(absolute_difference(p, mean)))
}

fn main() {
    let mut positions = include_str!("input.txt")
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    positions.sort();
    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));
}
