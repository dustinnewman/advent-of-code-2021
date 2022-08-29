const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn to_usize(byte: u8) -> usize {
    (byte - b'0') as usize
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, &point) in line.iter().enumerate() {
            if y > 0 && grid[y - 1][x] <= point {
                continue;
            }
            if y < grid.len() - 1 && grid[y + 1][x] <= point {
                continue;
            }
            if x > 0 && line[x - 1] <= point {
                continue;
            }
            if x < line.len() - 1 && line[x + 1] <= point {
                continue;
            }
            // If we have made it here we have local minimum
            sum += to_usize(point) + 1
        }
    }
    sum
}

fn basin(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    grid[y][x] = b'9';
    let mut sum = 1;
    for (dx, dy) in NEIGHBORS {
        let y2 = (y as isize + dy) as usize;
        let x2 = (x as isize + dx) as usize;
        if grid.get(y2).and_then(|line| line.get(x2)).map_or(false, |&neighbor| neighbor < b'9') {
            sum += basin(grid, x2, y2);
        }
    }
    sum
}

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let mut basins = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] < b'9' {
                basins.push(basin(&mut grid, x, y));
            }
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn main() {
    let grid: Vec<Vec<u8>> = include_bytes!("input.txt")
        .split(|&b| b == b'\n')
        .map(|line| line.to_vec())
        .collect();
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(grid));
}
