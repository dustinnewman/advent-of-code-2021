const GESTATION: usize = 7;
const FIRST_CYCLE_DELAY: usize = 2;
const POP_SIZE: usize = GESTATION + FIRST_CYCLE_DELAY;

fn simulate(mut fishes: [usize; POP_SIZE], days: usize) -> usize {
    for _ in 0..days {
        // All the pregnant fish i.e. about to produe a new lanternfish
        let pregnant = fishes[0];
        // Each fish de-ages one day
        for i in 1..POP_SIZE {
            fishes[i - 1] = fishes[i];
        }
        // Each pregnant fish has exactly one baby which starts the cycle
        // at 8 days old
        fishes[POP_SIZE - 1] = pregnant;
        // Each pregnant fish then goes back to 6 days old
        fishes[GESTATION - 1] += pregnant;
    }
    // How many fish total
    fishes.iter().sum()
}

fn main() {
    let mut fishes = [0usize; POP_SIZE];
    for age in include_str!("input.txt").split(",") {
        fishes[age.parse::<usize>().unwrap()] += 1;
    }
    println!("Part 1: {}", simulate(fishes.clone(), 80));
    println!("Part 2: {}", simulate(fishes, 256));
}
