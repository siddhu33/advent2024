use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn load_grid() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("src/day6/day6.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }
    Ok(grid)
}

fn in_range(val: i32, max: i32) -> bool {
    return 0 <= val && val < max;
}

fn next_offset(offset: (i32, i32)) -> (i32, i32) {
    match offset {
        (-1, 0) => return (0, 1),
        (0, 1) => return (1, 0),
        (1, 0) => return (0, -1),
        (0, -1) => return (-1, 0),
        _ => panic!("offset is wrong!"),
    }
}

fn traverse_grid(address: (i32, i32), dims: (i32, i32), grid: &Vec<Vec<char>>) -> i32 {
    let (mut i, mut j) = address;
    let mut steps: HashSet<(i32, i32)> = HashSet::from([address]);
    let (rows, cols) = dims;
    let mut offset: (i32, i32) = (-1, 0);
    let mut in_grid = true;
    let mut step_count = 0;
    while in_grid && step_count < rows * cols {
        let next_i = i + offset.0;
        let next_j = j + offset.1;
        if in_range(next_i, rows) && in_range(next_j, cols) {
            if grid[next_i as usize][next_j as usize] == '#' {
                offset = next_offset(offset);
            } else {
                i = next_i;
                j = next_j;
                steps.insert((i, j));
                step_count += 1;
            }
        } else {
            in_grid = false;
        }
    }
    if !in_grid {
        return steps.len() as i32;
    } else {
        return -1;
    }
}

fn traverse_grid_obstacles(
    start_address: (i32, i32),
    dims: (i32, i32),
    grid: &mut Vec<Vec<char>>,
) -> i32 {
    let mut locations: i32 = 0;
    let (rows, cols) = dims;
    for x in 0..rows {
        for y in 0..cols {
            if grid[x as usize][y as usize] == '.' {
                grid[x as usize][y as usize] = '#';
                let count = traverse_grid(start_address, dims, grid);
                if count == -1 {
                    locations += 1;
                }
                grid[x as usize][y as usize] = '.';
            }
        }
    }

    return locations;
}
pub fn part1() -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    let grid = load_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                count = traverse_grid((i as i32, j as i32), (rows as i32, cols as i32), &grid);
                break;
            }
        }
    }

    Ok(count)
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let mut locations = 0;
    let mut grid = load_grid()?;
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '^' {
                locations = traverse_grid_obstacles(
                    (i as i32, j as i32),
                    (rows as i32, cols as i32),
                    &mut grid,
                );
            }
        }
    }

    Ok(locations)
}
