fn main() {
    let numbers = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    let part_1 = numbers.windows(2).filter(|t| t[0] < t[1]).count();
    // Observation: Since the window sizes are 3 numbers long and we are comparing
    // their sums, we can reduce this to only checking the numbers that are different
    // that is: A + B + C < B + C + D iff A < D. We jump ahead in our iteration and
    // subtract the length of the vector just once at the beginning to avoid invalid
    // access.
    let part_2 = numbers
        .windows(4)
        .filter(|pairs| pairs[0] < pairs[3])
        .count();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2)
}
