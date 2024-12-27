#![allow(dead_code)]

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
        // Init flags
        let mut safe = true;
        let mut is_decreasing = true;

        for i in 1..report.len() {
            // For the first check set the order
            if i == 1 {
                if report[i] > report[i - 1] {
                    is_decreasing = false;
                } else if report[i] == report[i - 1] {
                    // If two numbers are the same, it is already not safe
                    safe = false;
                    break;
                }
            }
            // Get the difference
            let diff = report[i] - report[i - 1];
            // Do the checks
            // - Check if the difference is 0
            if diff == 0 {
                safe = false;
                break;
            }
            // - Check that diff is not greater than 3
            if -3 > diff || diff > 3 {
                safe = false;
                break;
            }
            // - Now conditionally check that the numbers are continuing the trend
            if is_decreasing {
                if diff > 0 {
                    safe = false;
                    break;
                }
            } else {
                if diff < 0 {
                    safe = false;
                    break;
                }
            }
        }

        // If all checks pass, then the report is safe
        if safe {
            num_safe += 1;
        }
    }

    // Step 4: Print result
    println!("{}", num_safe);
}
