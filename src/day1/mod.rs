use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::iter::zip;

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let (mut start_nums, mut end_nums) = parse_nums()?;
    start_nums.sort();
    end_nums.sort();
    let total_dist = zip(start_nums, end_nums)
        .map(|(x, y)| (y - x).abs())
        .reduce(|acc, e| acc + e);
    return Ok(total_dist.unwrap());
}

pub fn part2() -> Result<usize, Box<dyn Error>> {
    let (start_nums, end_nums) = parse_nums()?;
    let mut similarity: usize = 0;
    for i in start_nums {
        let score = end_nums.iter().filter(|x| **x == i).count();
        similarity += (i as usize) * score;
    }
    return Ok(similarity);
}

fn parse_nums() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut start_nums: Vec<i32> = Vec::new();
    let mut end_nums: Vec<i32> = Vec::new();
    let file = File::open("src/day1/day1.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let sizes: Vec<&str> = line.split("   ").collect();
        start_nums.push(sizes.get(0).unwrap().parse::<i32>().unwrap());
        end_nums.push(sizes.get(1).unwrap().parse::<i32>().unwrap());
    }
    Ok((start_nums, end_nums))
}
