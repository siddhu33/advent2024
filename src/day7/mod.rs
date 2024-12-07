use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn possible(output: u64, inputs: &Vec<u64>) -> bool {
    let num_operators: u64 = (inputs.len() - 1).try_into().unwrap();
    let large_output: u64 = output.try_into().unwrap();
    for code in 0..u64::pow(2, num_operators.try_into().unwrap()) {
        let mut start_val: u64 = inputs[0].try_into().unwrap();
        for i in 0..num_operators {
            let next_val: u64 = inputs[(i + 1) as usize].try_into().unwrap();
            // println!(
            //     "{}, {}, {}, {}, {}, {}",
            //     output,
            //     code,
            //     i,
            //     code >> i,
            //     start_val,
            //     next_val
            // );
            let val: u64 = (code >> i) & 1;
            if val == 1 {
                start_val *= next_val;
            } else {
                start_val += next_val;
            }
        }
        if start_val == large_output {
            return true;
        }
    }
    return false;
}

fn possible_3(output: u64, inputs: &Vec<u64>) -> bool {
    let num_operators: u64 = (inputs.len() - 1).try_into().unwrap();
    let large_output: u64 = output.try_into().unwrap();
    for code in 0..u64::pow(3, num_operators.try_into().unwrap()) {
        let mut start_val: u64 = inputs[0].try_into().unwrap();
        for i in 0..num_operators {
            let next_val: u64 = inputs[(i + 1) as usize].try_into().unwrap();
            // println!(
            //     "{}, {}, {}, {}, {}, {}",
            //     output,
            //     code,
            //     i,
            //     code >> i,
            //     start_val,
            //     next_val
            // );
            let val: u64 = (code / u64::pow(3, i.try_into().unwrap())) % 3;
            if val == 2 {
                start_val = format!("{}{}", start_val, next_val).parse::<u64>().unwrap();
            } else if val == 1 {
                start_val *= next_val;
            } else {
                start_val += next_val;
            }
        }
        if start_val == large_output {
            return true;
        }
    }
    return false;
}

pub fn part1() -> Result<u64, Box<dyn Error>> {
    let file = File::open("src/day7/day7.txt")?;
    let reader = BufReader::new(file);
    let mut total: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        let line_split: Vec<&str> = line.split(": ").collect();
        let output: u64 = line_split[0]
            .parse::<u64>()
            .expect("Could not parse output to int");
        let inputs: Vec<u64> = line_split[1]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        if possible(output, &inputs) {
            total += output;
        }
    }

    Ok(total)
}

pub fn part2() -> Result<u64, Box<dyn Error>> {
    let file = File::open("src/day7/day7.txt")?;
    let reader = BufReader::new(file);
    let mut total: u64 = 0;
    for line in reader.lines() {
        let line = line?;
        let line_split: Vec<&str> = line.split(": ").collect();
        let output: u64 = line_split[0]
            .parse::<u64>()
            .expect("Could not parse output to int");
        let inputs: Vec<u64> = line_split[1]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        if possible_3(output, &inputs) {
            total += output;
        }
    }

    Ok(total)
}
