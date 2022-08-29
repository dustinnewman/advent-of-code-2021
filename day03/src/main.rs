const BIT_LEN: usize = 12;

fn count_at_pos(nums: &[u32], pos: usize) -> (u32, u32) {
    let mut counts = (0, 0);
    for &num in nums {
        let bit = (num >> pos) & 1;
        if bit == 0 {
            counts.0 += 1;
        } else {
            counts.1 += 1;
        }
    }
    counts
}

fn part_1(nums: &Vec<u32>) -> u32 {
    let mut gamma = 0;
    for i in 0..BIT_LEN {
        let (zeros, ones) = count_at_pos(&nums, i);
        if ones > zeros {
            gamma |= 1 << i;
        }
    }
    let epsilon = !gamma & ((1 << BIT_LEN) - 1);
    gamma * epsilon
}

fn process_numbers(nums: &[u32], default_value: u32, secondary_value: u32) -> u32 {
    let mut nums = nums.to_vec();
    for i in (0..BIT_LEN).rev() {
        let (zeros, ones) = count_at_pos(&nums, i);
        // >= because we default to 1 if the number of ones equals number of zeros
        let common = if ones >= zeros {
            default_value
        } else {
            secondary_value
        };
        nums.retain(|&x| (x >> i) & 1 == common);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

fn part_2(nums: &Vec<u32>) -> u32 {
    let o2_generator_rating = process_numbers(&nums, 1, 0);
    let co2_scrubber_rating = process_numbers(&nums, 0, 1);
    o2_generator_rating * co2_scrubber_rating
}

fn main() {
    let nums: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let part_1 = part_1(&nums);
    let part_2 = part_2(&nums);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
