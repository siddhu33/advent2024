use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn in_range(val: i32, max: usize) -> bool {
    return 0 <= val && val < max.try_into().unwrap();
}

fn found_xmas(grid: &Vec<Vec<char>>, i: i32, j: i32, rows: usize, columns: usize) -> i32 {
    let offsets = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut matches = 0;
    for offset in offsets {
        let mut other_chars: [char; 3] = [' ', ' ', ' '];
        let mut offset_i = i;
        let mut offset_j = j;
        for attempt in 0..3 {
            offset_i = offset_i + offset.0;
            offset_j = offset_j + offset.1;
            if in_range(offset_i, rows) && in_range(offset_j, columns) {
                let offset_char = grid[offset_i as usize][offset_j as usize];
                other_chars[attempt as usize] = offset_char;
            }
        }
        if other_chars == ['M', 'A', 'S'] {
            matches += 1;
        }
    }

    return matches;
}

fn load_grid() -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let file = File::open("src/day4/day4.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let char_vec: Vec<char> = line.chars().collect();
        grid.push(char_vec);
    }
    Ok(grid)
}

pub fn part1() -> Result<i32, Box<dyn Error>> {
    let grid = load_grid()?;
    let mut count = 0;

    let rows = grid.len();
    let columns = grid[0].len();
    for i in 0..rows {
        for j in 0..columns {
            if grid[i][j] == 'X' {
                count += found_xmas(
                    &grid,
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    rows,
                    columns,
                );
            }
        }
    }

    Ok(count)
}

fn found_x_mas(grid: &Vec<Vec<char>>, i: i32, j: i32, rows: usize, columns: usize) -> i32 {
    let offsets = vec![(-1, 1), (1, -1), (-1, -1), (1, 1)];
    let mut matches = 0;
    let mut other_chars: [i32; 4] = [-2, -2, -2, -2];
    for (idx, (i2, j2)) in offsets.iter().enumerate() {
        let offset_i = i + i2;
        let offset_j = j + j2;
        if in_range(offset_i, rows) && in_range(offset_j, columns) {
            let offset_char = grid[offset_i as usize][offset_j as usize];
            if offset_char == 'M' {
                other_chars[idx] = 1;
            } else if offset_char == 'S' {
                other_chars[idx] = -1
            }
        }
    }
    if other_chars[0] + other_chars[1] == 0 && other_chars[2] + other_chars[3] == 0 {
        matches += 1
    }

    return matches;
}

pub fn part2() -> Result<i32, Box<dyn Error>> {
    let grid = load_grid()?;

    let rows = grid.len();
    let columns = grid[0].len();
    let mut count = 0;
    for i in 0..rows {
        for j in 0..columns {
            if grid[i][j] == 'A' {
                count += found_x_mas(
                    &grid,
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    rows,
                    columns,
                );
            }
        }
    }

    Ok(count)
}
