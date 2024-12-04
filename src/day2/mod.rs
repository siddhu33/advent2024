use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() -> Result<u32, Box<dyn Error>> {
    let file = File::open("src/day2/day2.txt")?;
    let reader = BufReader::new(file);
    let mut safe: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().expect("failure to parse int"))
            .collect();
        let (row_safe, _fail_index) = test_vector(&nums);
        safe += row_safe;
    }
    return Ok(safe);
}

pub fn part2() -> Result<u32, Box<dyn Error>> {
    let file = File::open("src/day2/day2.txt")?;
    let reader = BufReader::new(file);
    let mut safe: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().expect("failure to parse int"))
            .collect();
        let (mut row_safe, fail_index) = test_vector(&nums);
        if row_safe != 1 {
            let mut test_index = max(0, fail_index - 1);
            while test_index < nums.len().try_into().unwrap() {
                let mut nums_clone = nums.clone();
                nums_clone.remove(test_index.try_into().unwrap());
                let (safe_attempt, _fail_idx_attempt) = test_vector(&nums_clone);
                if safe_attempt == 1 {
                    row_safe = 1;
                    break;
                }
                test_index += 1;
            }
        }
        safe += row_safe;
    }
    return Ok(safe);
}

fn test_vector(nums: &Vec<i32>) -> (u32, i32) {
    let mut row_safe = 1;
    let mut line_sign = 0;
    let mut fail_index = -1;
    for idx in 0..nums.len() - 1 {
        let curr = nums[idx];
        let next = nums[idx + 1];
        let diff = next - curr;
        let diff_sign = diff.signum();
        if line_sign == 0 {
            line_sign = diff_sign;
        }
        if diff_sign == 0 || line_sign != diff_sign {
            row_safe = 0;
            fail_index = idx.try_into().unwrap();
            break;
        } else {
            //check gap, has to be between 1-3
            let abs_diff = diff.abs();
            if abs_diff < 1 || abs_diff > 3 {
                row_safe = 0;
                fail_index = idx.try_into().unwrap();
                break;
            }
        }
    }
    return (row_safe, fail_index);
}
