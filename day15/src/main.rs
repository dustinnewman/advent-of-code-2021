use std::{cmp::Reverse, collections::BinaryHeap};

const NEIGHBORS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn no_neg(row: usize, dr: isize, col: usize, dc: isize) -> Option<(usize, usize)> {
    if (row == 0 && dr < 0) || (col == 0 && dc < 0) {
        None
    } else {
        Some(((row as isize + dr) as usize, (col as isize + dc) as usize))
    }
}

fn dijkstra(graph: &Vec<Vec<u8>>) -> usize {
    let num_rows = graph.len();
    let num_cols = graph[0].len();
    let end = (num_rows - 1, num_cols - 1);
    let mut dist = vec![vec![usize::MAX; num_cols]; num_rows];
    let mut queue = BinaryHeap::new();
    // Dijkstra's requires a min heap rather than max
    queue.push(Reverse((0, 0, 0)));
    while let Some(Reverse((path, row, col))) = queue.pop() {
        if (row, col) == end {
            return path;
        }
        for (r2, c2) in NEIGHBORS
            .iter()
            .filter_map(|(dr, dc)| no_neg(row, *dr, col, *dc))
        {
            let next = match graph.get(r2).and_then(|row| row.get(c2)) {
                Some(&risk) => risk as usize + path,
                None => continue,
            };
            if next < dist[r2][c2] {
                queue.push(Reverse((next, r2, c2)));
                dist[r2][c2] = next;
            }
        }
    }
    0
}

fn main() {
    let cave: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect();
    let num_rows = cave.len();
    let num_cols = cave[0].len();
    let entire_cave = (0..5 * num_rows)
        .map(|row| {
            (0..5 * num_cols)
                .map(|col| {
                    let risk = cave[row % num_rows][col % num_cols]
                        + (row / num_rows) as u8
                        + (col / num_cols) as u8;
                    if risk < 10 {
                        risk
                    } else {
                        risk - 9
                    }
                })
                .collect()
        })
        .collect();
    println!("Part 1: {}", dijkstra(&cave));
    println!("Part 2: {}", dijkstra(&entire_cave));
}
