use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    let file = File::open("src/day5/day5.txt")?;
    let reader = BufReader::new(file);
    let mut page_switch: bool = false;
    let mut after_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut before_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            page_switch = true;
            continue;
        }
        if page_switch {
            let page: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut in_order = true;
            for (idx, val) in page.iter().enumerate() {
                let before_vals: HashSet<i32> = HashSet::from_iter(page[..idx].to_vec());
                let empty_set: HashSet<i32> = HashSet::new();
                let before_map_value = before_map.get(val).unwrap_or(&empty_set);
                let before_diff = before_vals.difference(before_map_value).count();
                if before_diff > 0 {
                    in_order = false;
                    break;
                }
            }
            if in_order {
                count += page[page.len() / 2];
            }
        } else {
            let pair: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            let existing = after_map.get_mut(&pair[0]);
            if existing.is_some() {
                existing.unwrap().insert(pair[1]);
            } else {
                after_map.insert(pair[0], HashSet::from([pair[1]]));
            }
            let existing = before_map.get_mut(&pair[1]);
            if existing.is_some() {
                existing.unwrap().insert(pair[0]);
            } else {
                before_map.insert(pair[1], HashSet::from([pair[0]]));
            }
        }
    }

    Ok(count)
}

fn custom_cmp(a: i32, b: i32, before_map: &HashMap<i32, HashSet<i32>>) -> Ordering {
    let empty_set: HashSet<i32> = HashSet::new();
    let set = before_map.get(&b).unwrap_or(&empty_set);
    if set.contains(&a) {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    let file = File::open("src/day5/day5.txt")?;
    let reader = BufReader::new(file);
    let mut page_switch: bool = false;
    let mut before_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            page_switch = true;
            continue;
        }
        if page_switch {
            let mut page: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut in_order = true;
            for (idx, val) in page.iter().enumerate() {
                let before_vals: HashSet<i32> = HashSet::from_iter(page[..idx].to_vec());
                let empty_set: HashSet<i32> = HashSet::new();
                let before_map_value = before_map.get(val).unwrap_or(&empty_set);
                let before_diff = before_vals.difference(before_map_value).count();
                if before_diff > 0 {
                    in_order = false;
                    break;
                }
            }
            if !in_order {
                page.sort_by(|&a, &b| custom_cmp(a, b, &before_map));
                count += page[page.len() / 2];
            }
        } else {
            let pair: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
            let existing = before_map.get_mut(&pair[1]);
            if existing.is_some() {
                existing.unwrap().insert(pair[0]);
            } else {
                before_map.insert(pair[1], HashSet::from([pair[0]]));
            }
        }
    }

    Ok(count)
}
