use std::error::Error;
use std::fs::read;

use itertools::Itertools;

fn has_gaps(input: &Vec<i64>) -> bool {
    let non_gap_values = input.iter().filter(|&&x| x != -1).count();
    return input[..non_gap_values].iter().filter(|&&x| x == -1).count() > 0;
}

fn next_gap(input: &Vec<i64>) -> i64 {
    for idx in 0..input.len() {
        if input[idx] == -1 {
            return idx as i64;
        }
    }
    return -1;
}

fn last_data_point(input: &Vec<i64>, last_val: i64) -> i64 {
    let mut idx = last_val;
    while idx >= 0 {
        if input[idx as usize] != -1 {
            return idx as i64;
        }
        idx -= 1;
    }
    return -1;
}

fn rotate_vec(input: &mut Vec<i64>) -> i64 {
    let mut last_data_idx = (input.len() - 1) as i64;
    while has_gaps(input) {
        last_data_idx = last_data_point(input, last_data_idx);
        let next_gap_idx = next_gap(input);
        input[next_gap_idx as usize] = input[last_data_idx as usize];
        input[last_data_idx as usize] = -1;
    }
    return input[..(last_data_idx as usize)]
        .iter()
        .enumerate()
        .map(|(a, &b)| (a as i64) * b)
        .reduce(|a, b| a + b)
        .expect("error calculating checksum");
}

pub fn part1() -> Result<i64, Box<dyn Error>> {
    let file: Vec<u8> = read("src/day9/day9.txt")?;
    let mut data: Vec<i64> = Vec::new();
    let mut is_file = true;
    let mut f_id: i64 = -1;
    for byte in file {
        let mut to_add: i64 = -1;
        if is_file {
            f_id += 1;
            to_add = f_id;
        }
        data.extend(vec![to_add; (byte - 48) as usize]);
        is_file = !is_file;
    }

    Ok(rotate_vec(&mut data))
}

pub fn part2() -> Result<i64, Box<dyn Error>> {
    let file: Vec<u8> = read("src/day9/day9.txt")?;
    let mut is_file = true;
    let mut data: Vec<(i64, i64)> = Vec::new();
    let mut f_id = 0;
    for byte in file {
        if is_file {
            data.push((f_id, (byte - 48) as i64));
            f_id += 1;
        } else {
            data.push((-1, (byte - 48) as i64));
        }
        is_file = !is_file;
    }
    let mut start_fid = f_id - 1;
    while start_fid >= 0 {
        let (f_id_idx, f_id_val) = data
            .iter()
            .find_position(|x| x.0 == start_fid)
            .expect("value missing!");
        let (_f_id_val_val, f_id_size) = f_id_val.clone();
        let free_space_pos = data.iter().position(|x| x.0 == -1 && x.1 >= f_id_size);
        if free_space_pos.is_some() {
            let pos = free_space_pos.unwrap();
            if pos < f_id_idx {
                let free_space_size = data[pos].1;
                data.insert(pos, (start_fid, f_id_size));
                data[pos + 1] = (-1, free_space_size - f_id_size);
                data[f_id_idx + 1] = (-1, f_id_size);
            }
        }
        start_fid -= 1;
    }
    let mut checksum_start_idx: i64 = 0;
    let mut checksum: i64 = 0;
    for (val, size) in data {
        if val != -1 {
            for i in 0..size {
                checksum += val * (i + checksum_start_idx);
            }
        }
        checksum_start_idx += size
    }
    Ok(checksum)
}
