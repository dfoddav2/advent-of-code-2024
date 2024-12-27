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

    // Step 3: Get similiarity score
    let mut similiarity_score = 0;
    for left_num in left_nums {
        let matches = right_nums.iter().filter(|&x| x == &left_num).count() as i32;
        if matches > 0 {
            similiarity_score += left_num * matches;
        }
    }
    
    // Step 4: Print result
    println!("{}", similiarity_score);
}
