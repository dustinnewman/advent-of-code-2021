fn part_1(lines: &Vec<(&str, u32)>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    for (cmd, x) in lines {
        match *cmd {
            "forward" => pos += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => (),
        };
    }
    pos * depth
}

fn part_2(lines: &Vec<(&str, u32)>) -> u32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (cmd, x) in lines {
        match *cmd {
            "forward" => {
                pos += x;
                depth += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => (),
        };
    }
    pos * depth
}

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(cmd, x)| (cmd, x.parse().unwrap()))
        .collect::<Vec<(&str, u32)>>();
    let part_1 = part_1(&lines);
    let part_2 = part_2(&lines);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
