use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn load_grid() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("src/day8/day8.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }
    Ok(grid)
}

fn in_range(location: (i64, i64), rows: i64, cols: i64) -> bool {
    let row_in_range = location.0 >= 0 && location.0 < rows;
    let col_in_range = location.1 >= 0 && location.1 < cols;
    return row_in_range && col_in_range;
}

pub fn part1() -> Result<u64, Box<dyn Error>> {
    let grid = load_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antenna_locs: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..rows {
        for j in 0..cols {
            let grid_val: char = grid[i][j];
            if grid_val != '.' {
                //antenna found
                antenna_locs.insert((i, j));
                //check map for existing list
                antenna_map.entry(grid_val).or_insert(vec![]).push((i, j));
            }
        }
    }
    //find antinodes
    let mut antinode_locs: HashSet<(i64, i64)> = HashSet::new();
    for (_antenna, locs) in antenna_map.iter() {
        for pair in locs.iter().combinations(2) {
            let loc_vector: (i64, i64) = (
                (pair[1].0 as i64 - pair[0].0 as i64),
                (pair[1].1 as i64 - pair[0].1 as i64),
            );
            let prev_antinode: (i64, i64) = (
                pair[0].0 as i64 - loc_vector.0,
                pair[0].1 as i64 - loc_vector.1,
            );
            let next_antinode: (i64, i64) = (
                pair[1].0 as i64 + loc_vector.0,
                pair[1].1 as i64 + loc_vector.1,
            );
            if in_range(prev_antinode, rows as i64, cols as i64) {
                antinode_locs.insert(prev_antinode);
            }
            if in_range(next_antinode, rows as i64, cols as i64) {
                antinode_locs.insert(next_antinode);
            }
        }
    }
    Ok(antinode_locs.len() as u64)
}

pub fn part2() -> Result<u64, Box<dyn Error>> {
    let grid = load_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut antenna_locs: HashSet<(i64, i64)> = HashSet::new();
    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..rows {
        for j in 0..cols {
            let grid_val: char = grid[i][j];
            if grid_val != '.' {
                //antenna found
                antenna_locs.insert((i as i64, j as i64));
                //check map for existing list
                antenna_map.entry(grid_val).or_insert(vec![]).push((i, j));
            }
        }
    }
    //find antinodes
    let mut antinode_locs: HashSet<(i64, i64)> = HashSet::new();
    for (_antenna, locs) in antenna_map.iter() {
        for pair in locs.iter().combinations(2) {
            let loc_vector: (i64, i64) = (
                (pair[1].0 as i64 - pair[0].0 as i64),
                (pair[1].1 as i64 - pair[0].1 as i64),
            );
            let mut prev_antinode: (i64, i64) = (
                pair[0].0 as i64 - loc_vector.0,
                pair[0].1 as i64 - loc_vector.1,
            );
            while in_range(prev_antinode, rows as i64, cols as i64) {
                antinode_locs.insert(prev_antinode);
                prev_antinode = (
                    prev_antinode.0 as i64 - loc_vector.0,
                    prev_antinode.1 as i64 - loc_vector.1,
                );
            }
            let mut next_antinode: (i64, i64) = (
                pair[1].0 as i64 + loc_vector.0,
                pair[1].1 as i64 + loc_vector.1,
            );
            while in_range(next_antinode, rows as i64, cols as i64) {
                antinode_locs.insert(next_antinode);
                next_antinode = (
                    next_antinode.0 as i64 + loc_vector.0,
                    next_antinode.1 as i64 + loc_vector.1,
                );
            }
        }
    }
    let total_location_count = antinode_locs.union(&antenna_locs).count();
    Ok(total_location_count as u64)
}
