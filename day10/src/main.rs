use std::str::Lines;

fn pair(a: char, b: char) -> bool {
    match (a, b) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false
    }
}

fn process_char(stack: &mut Vec<char>, char: char) -> Option<usize> {
    match char {
        '(' | '[' | '{' | '<' => {
            stack.push(char);
            None
        },
        _ => match (stack.pop(), char) {
            (Some(a), char) if pair(a, char) => None,
            (_, ')') => Some(3),
            (_, ']') => Some(57),
            (_, '}') => Some(1197),
            (_, '>') => Some(25137),
            _ => None
        }
    }
}

fn corruption_score(line: &str) -> usize {
    let mut stack = vec![];
    line
        .chars()
        .find_map(|char| process_char(&mut stack, char))
        .unwrap_or(0)
}

fn part1(lines: &mut Lines) -> usize {
    lines
        .map(|line| corruption_score(line))
        .sum()
}

fn complete_line(line: &str) -> Option<usize> {
    let mut stack = vec![];
    if line.chars().map(|char| process_char(&mut stack, char)).any(|score| score.is_some()) {
        return None;
    }
    Some(stack.iter().rev().fold(0, |acc, char| acc * 5 + match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0
    }))
}

fn part2(lines: &mut Lines) -> usize {
    let mut scores: Vec<usize> = lines
        .filter_map(complete_line)
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let lines = include_str!("input.txt");
    println!("Part 1: {}", part1(&mut lines.lines()));
    println!("Part 2: {}", part2(&mut lines.lines()));
}