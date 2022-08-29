use std::collections::HashMap;

type Polymer = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;

// This is to allow us to account for a "tuple" of only one char
// We treat the identity char like the empty string. It is essential
// that this char never appear in the polymer string or rule set
const IDENTITY_CHAR: char = '0';
const LAST_CHAR: char = 'N';

fn apply(polymer_a: Polymer, rules: &Rules) -> HashMap<(char, char), usize> {
    let mut polymer_b = HashMap::with_capacity(polymer_a.len());
    for (key@(a, b), value) in polymer_a {
        let insert = match rules.get(&key) {
            Some(&insert) => insert,
            None => IDENTITY_CHAR,
        };
        let ac = (a, insert);
        let bc = (insert, b);
        *(polymer_b.entry(ac).or_insert(0)) += value;
        *(polymer_b.entry(bc).or_insert(0)) += value;
    }
    polymer_b
}

fn rounds(initial_polymer: Polymer, rules: &Rules, last_char: char, rounds: usize) -> usize {
    let final_round = (0..rounds).fold(initial_polymer, |prev, _| apply(prev, rules));
    let mut polymer = HashMap::with_capacity(final_round.len());
    for ((a, _), value) in final_round {
        *(polymer.entry((a, IDENTITY_CHAR)).or_insert(0)) += value;
    }
    *(polymer.entry((last_char, IDENTITY_CHAR)).or_insert(0)) += 1;
    polymer.values().max().unwrap() - polymer.values().min().unwrap()
}

fn main() {
    let (seed, rules) = include_str!("input.txt").split_once("\n\n").unwrap();
    let mut polymer = HashMap::new();
    for pair in seed.as_bytes().windows(2) {
        let pair = (pair[0] as char, pair[1] as char);
        *(polymer.entry(pair).or_insert(0)) += 1;
    }
    let rules = rules
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, product)| {
            let pair = pair.as_bytes();
            let pair: (char, char) = (pair[0] as char, pair[1] as char);
            (pair, product.as_bytes()[0] as char)
        })
        .collect::<Rules>();
    println!("Part 1: {}", rounds(polymer.clone(), &rules, LAST_CHAR, 10));
    println!("Part 2: {}", rounds(polymer, &rules, LAST_CHAR, 40));
}
