type Configuration<'a> = Vec<([&'a str; 10], [&'a str; 4])>;

fn part1(signals: &Configuration) -> usize {
    signals
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

fn light_mask(wires: &str) -> u8 {
    wires
        .bytes()
        .fold(0, |acc, wire| acc | (1 << (wire - b'a')))
}

fn overlaps(x: u8, y: u8) -> u32 {
    (x & y).count_ones()
}

fn count_digits(wires: &[u8; 10]) -> [u8; 10] {
    let mut digit = [0; 10];
    for &bitmask in wires.iter() {
        match bitmask.count_ones() {
            2 => digit[1] = bitmask,
            3 => digit[7] = bitmask,
            4 => digit[4] = bitmask,
            7 => digit[8] = bitmask,
            _ => (),
        }
    }
    
    for &bitmask in wires.iter() {
        match bitmask.count_ones() {
            5 => {
                if overlaps(bitmask, digit[1]) == 2 {
                    digit[3] = bitmask;
                } else if overlaps(bitmask, digit[4]) == 2 {
                    digit[2] = bitmask;
                } else {
                    digit[5] = bitmask;
                }
            },
            6 => {
                if overlaps(bitmask, digit[4]) == 4 {
                    digit[9] = bitmask;
                } else if overlaps(bitmask, digit[1]) == 2 {
                    digit[0] = bitmask;
                } else {
                    digit[6] = bitmask;
                }
            },
            _ => (),
        }
    }
    digit
}

fn part2(signals: &Configuration) -> usize {
    signals
        .iter()
        .map(|(wires, outputs)| (wires.map(light_mask), outputs.map(light_mask)))
        .map(|(wires, outputs)| {
            let digits = count_digits(&wires);
            let mut output = 0;
            for d in outputs.iter().copied() {
                let mapped = digits.iter().enumerate().filter(|(_, segs)| **segs == d).map(|(n, _)| n).next().unwrap();
                output *= 10;
                output += mapped as usize;
            }
            output
        })
        .sum()
}

fn main() {
    let signals: Configuration = include_str!("input.txt")
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split(" | ").collect();
            let wires: [&str; 10] = line[0]
                .split(" ")
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let digits: [&str; 4] = line[1]
                .split(" ")
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            (wires, digits)
        })
        .collect();
    let part1 = part1(&signals);
    let part2 = part2(&signals);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
