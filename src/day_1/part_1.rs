#![allow(dead_code)]

pub fn run() {
    // Step 1: Read the input file
    let input =
        crate::utils::read_input("src/day_1/day_1_1.txt").expect("Failed to read input file");

    // Step 2: Parse the input file into two vectors of integers
    let lines: Vec<&str> = input.lines().collect();
    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        for i in 0..split.len() {
            if i == 0 {
                left_nums.push(split[i].parse().expect("Failed to parse to integer"));
            } else {
                right_nums.push(split[i].parse().expect("Failed to parse to integer"));
            }
        }
    }

    // Step 3: Sort the two vectors
    left_nums.sort();
    right_nums.sort();
    // println!("{:?}", left_nums);

    // Step 4: Iterate through the length of the vectors
    let mut total_diff = 0;
    for i in 0..left_nums.len() {
        let diff = (left_nums[i] - right_nums[i]).abs();
        total_diff += diff;
    }

    // Step 5: Print result
    println!("{}", total_diff);
}
