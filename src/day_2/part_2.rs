#![allow(dead_code)]

fn is_safe(report: &Vec<i32>) -> bool {
    // Init flags
    let mut is_decreasing = true;

    for i in 1..report.len() {
        // For the first check set the order
        if i == 1 {
            if report[i] > report[i - 1] {
                is_decreasing = false;
            } else if report[i] == report[i - 1] {
                // If two numbers are the same, it is already not safe
                return false;
            }
        }
        // Get the difference
        let diff = report[i] - report[i - 1];
        // Do the checks
        // - Check if the difference is 0
        if diff == 0 {
            return false;
        }
        // - Check that diff is not greater than 3
        if -3 > diff || diff > 3 {
            return false;
        }
        // - Now conditionally check that the numbers are continuing the trend
        if is_decreasing {
            if diff > 0 {
                return false;
            }
        } else {
            if diff < 0 {
                return false;
            }
        }
    }
    return true;
}

pub fn run() {
    // Step 1: Read the input file
    let input = crate::utils::read_input("src/day_2/day_2.txt").expect("Failed to read input file");

    // Step 2: Parse the input file - get the lines - create vector of vectors
    let lines: Vec<&str> = input.lines().collect();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse to integer"))
            .collect();
        reports.push(report);
    }

    // Step 3: Loop through the reports and check for safe reports
    let mut num_safe = 0;
    for report in reports {
        // Check if the report is safe
        if is_safe(&report) {
            num_safe += 1;
        } else {
            // Else we loop through the report's length checking if with 1 less element it could be safe
            // TODO: This is quite a naive approach, but it works for the input
            for i in 0..report.len() {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if is_safe(&report_copy) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }

    // Step 4: Print result
    println!("{}", num_safe);
}
