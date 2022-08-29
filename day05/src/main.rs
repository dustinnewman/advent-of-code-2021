use std::collections::HashMap;

fn count(coordinates: Vec<(i32, i32, i32, i32)>) -> usize {
    let mut board: HashMap<(i32, i32), u32> = HashMap::new();
    for (x1, y1, x2, y2) in coordinates {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let (mut xi, mut yi) = (x1, y1);
        while (xi, yi) != (x2+dx, y2+dy) {
            *board.entry((xi, yi)).or_insert(0) += 1;
            xi += dx;
            yi += dy;
        }
    }
    board.values().filter(|&&count| count > 1).count()
}

fn main() {
    let mut coordinates = vec![];
    for line in include_str!("input.txt").lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let c1: Vec<i32> = points[0].split(",").map(|a| a.parse().unwrap()).collect();
        let c2: Vec<i32> = points[1].split(",").map(|a| a.parse().unwrap()).collect();
        coordinates.push((c1[0], c1[1], c2[0], c2[1]));
    }
    println!("Part 1: {:?}", count(coordinates.clone().into_iter().filter(|(x1,y1,x2,y2)| x1 == x2 || y1 == y2).collect()));
    println!("Part 2: {:?}", count(coordinates));
}
