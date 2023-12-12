use std::error::Error;
use ndarray::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn expand_universe(universe: &mut Array2<char>) -> Array2<char> {
    let mut rows_to_insert: Vec<_> = vec![];
    for (idx,r) in universe.rows().into_iter().enumerate() {
        if r.iter().all(|c| *c == '.') {
            rows_to_insert.push(idx);
        }
    }

    for i in rows_to_insert {
        let mut new = Array1::<char>::default(universe.len_of(Axis(0)));
        new.fill('.');
        universe.push_row(new.view());
        universe.slice_each_axis
    }

    let mut cols_to_insert: Vec<_> = vec![];
    for (idx,c) in universe.columns().into_iter().enumerate() {
        if c.iter().all(|c| *c == '.') {
            cols_to_insert.push(idx)
        }
    
    }

    println!("{rows_to_insert:?}; {cols_to_insert:?}");
    new_arr
}

fn part1(contents: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = contents.lines().map(|l| {l.chars().collect::<Vec<char>>()}).collect();
    let row_len = grid.len();
    let col_len = grid[0].len();

    let mut arr = Array2::<char>::default((row_len, col_len));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = grid[i][j].clone();
        }
    }
    println!("{arr}");
    expand_universe(&mut arr);
    Ok(())
}

fn part2(contents: &str) -> Result<()> {
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
