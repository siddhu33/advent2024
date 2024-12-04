use core::str;
use std::cmp::min;
use std::error::Error;
use std::fs::read;
use std::usize;

const VALID_BYTES: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let file: Vec<u8> = read("src/day3/day3.txt")?;
    let mut file_str = str::from_utf8(&file)
        .expect("Could not convert data to string")
        .replace("\n", "");
    let mut acc = 0;
    while file_str.len() > 0 {
        let find_idx: Option<usize> = file_str.find("mul(");
        if find_idx.is_none() {
            break;
        } else {
            let mut complete = false;
            let mut valid = true;
            let mut start_char = find_idx.unwrap() + 4;
            let mut inner: Vec<&str> = Vec::new();
            while !complete {
                let byte = &file_str[start_char..start_char + 1];
                if VALID_BYTES.contains(&byte) || byte == "," || byte == ")" {
                    if byte != ")" {
                        inner.push(byte);
                    } else {
                        complete = true;
                    }
                } else {
                    valid = false;
                    break;
                }
                start_char += 1;
            }
            if valid && inner.contains(&",") {
                let value = inner
                    .concat()
                    .split(",")
                    .map(|x| x.parse::<i32>().expect("parse int error"))
                    .reduce(|acc, e| acc * e);
                acc += value.expect("error calculating sum");
            }
            file_str = (&file_str[start_char..]).to_string();
        }
    }
    return Ok(acc);
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let file: Vec<u8> = read("src/day3/day3.txt")?;
    let mut file_str = str::from_utf8(&file)
        .expect("Could not convert data to string")
        .replace("\n", "");
    let mut acc = 0;
    let mut include: bool = true;
    while file_str.len() > 0 {
        let mut do_idx = file_str.find("do()").unwrap_or(usize::MAX);
        let mut do_not_idx = file_str.find("don't()").unwrap_or(usize::MAX);
        let find_idx: Option<usize> = file_str.find("mul(");
        if find_idx.is_none() {
            break;
        } else {
            let mut complete = false;
            let mut valid = true;
            let mut start_char = find_idx.unwrap() + 4;
            let mul_start_char = find_idx.unwrap();
            let mut inner: Vec<&str> = Vec::new();
            if do_idx < do_not_idx && do_idx < start_char {
                include = true;
                do_idx += 4;
                file_str = (&file_str[do_idx..]).to_string();
                continue;
            } else if do_not_idx < do_idx && do_not_idx < start_char {
                include = false;
                do_not_idx += 7;
                file_str = (&file_str[do_not_idx..]).to_string();
                continue;
            }
            while !complete {
                let byte = &file_str[start_char..start_char + 1];
                if VALID_BYTES.contains(&byte) || byte == "," || byte == ")" {
                    if byte != ")" {
                        inner.push(byte);
                    } else {
                        complete = true;
                    }
                } else {
                    valid = false;
                    break;
                }
                start_char += 1;
            }
            if valid && inner.contains(&",") && include {
                let value = inner
                    .concat()
                    .split(",")
                    .map(|x| x.parse::<i32>().expect("parse int error"))
                    .reduce(|acc, e| acc * e);
                acc += value.expect("error calculating sum");
            }
            file_str = (&file_str[start_char..]).to_string();
        }
    }
    return Ok(acc);
}
