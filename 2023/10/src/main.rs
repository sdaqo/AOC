use std::{error::Error, collections::HashMap};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn check_adjecent(coord: (i32, i32), last_pos: (i32, i32), grid: &Vec<Vec<char>>) -> (i32,i32) {
    let pos = (coord.0, coord.1);
    let len_row = grid.len() as i32;
    let len_col = grid[0].len() as i32;

    let allowed = match grid[pos.0 as usize][pos.1 as usize] {
        '-' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
        '|' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
        'J' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
        'F' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
        'L' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
        '7' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
        _ => vec![]
    };
    for (row, col) in allowed.iter() {
        if &0 <= row && row < &len_row && &0 <= col && col < &len_col{
            if grid[*row as usize][*col as usize] == '.' {
                continue;
            }
            if (*row as i32, *col as i32) != last_pos {
                println!("{}: {},{}", grid[*row as usize][*col as usize], row, col);
                return (*row as i32, *col as i32)
            }
        }
    }

    return (0,0)
}

fn part1(contents: &str) -> Result<()> {
    let lines: Vec<Vec<char>> = contents.lines().map(|l| {l.chars().collect::<Vec<char>>()}).collect();
    let mut s_pos = (0,0);
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == 'S' {
                s_pos = (row as i32, col as i32);
                break;
            }
        }
    }

    let mut path: Vec<(i32, i32)> = vec![s_pos, (s_pos.0 + 0, s_pos.1 + 1)];
    println!("{path:?}");
    loop { 
        let next_pos = check_adjecent(*path.iter().nth_back(0).unwrap(), *path.iter().nth_back(1).unwrap(), &lines);
        if next_pos == (0,0) {
            break;
        } else {
            path.push(next_pos);
        }
    }
    path.remove(0);

    println!("S: {s_pos:?}");
    println!("{}", path.len()/2);
    Ok(())
}

fn part2(contents: &str) -> Result<()> {
    let lines: Vec<Vec<char>> = contents.lines().map(|l| {l.chars().collect::<Vec<char>>()}).collect();
    let mut s_pos = (0,0);
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == 'S' {
                s_pos = (row as i32, col as i32);
                break;
            }
        }
    }
                                                //  this is static 
    let mut path: Vec<(i32, i32)> = vec![s_pos, (s_pos.0 + 0, s_pos.1 + 1)];
    println!("{path:?}");
    loop { 
        let next_pos = check_adjecent(*path.iter().nth_back(0).unwrap(), *path.iter().nth_back(1).unwrap(), &lines);
        if next_pos == (0,0) {
            break;
        } else {
            path.push(next_pos);
        }
    }
    path.remove(0);
    let mut num_in_loop = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if path.contains(&(row as i32, col as i32)) {
                continue;
            }
            let row_i32 = row as i32;

            let num = path.iter().filter(|a| { 
                a.0 == row_i32 && a.1 < col as i32
            }).fold(0, |acc, c| {
                if  "|LJ".contains(lines[c.0 as usize][c.1 as usize]) {
                    acc + 1
                } else  {
                    acc
                }
            });

            if num % 2 != 0  {
                num_in_loop += 1;
            }
            println!("row {row_i32} : {num}  : {num_in_loop}");
        }
    }
    println!("{num_in_loop}");

    Ok(())
}

fn main() -> Result<()> {
    let mut file = std::env::args().nth(1).expect("No File Specified");
    if file == "ex" {
        file = "./example.txt".to_string();
    } else if file == "in" {
        file = "./input.txt".to_string();
    }
    let contents = std::fs::read_to_string(file)?;

    let part = std::env::args().nth(2);
    if let Some(p) = part {
        if p == "1".to_string() {
            part1(&contents)?;
        } else {
            part2(&contents)?;
        }
    } else {
        part1(&contents)?; 
    }
    Ok(())
}
