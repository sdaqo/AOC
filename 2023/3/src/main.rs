use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn check_pos(lines: &Vec<&str>, row: usize, start: usize, end: usize) -> (bool, (usize, usize)) {
    if start != 0 {
        if lines[row].chars().nth(start-1).unwrap() != '.' {
            return (true, (row, start-1));
        }
        if row != 0 {
            let top_left = lines[row-1].chars().nth(start-1).unwrap();
            if  top_left != '.' && !top_left.is_digit(10) {
                return (true, (row-1, start-1));
            }
        }

        if lines.len() - 1 != row {
            let bottom_left = lines[row+1].chars().nth(start-1).unwrap();
            if bottom_left != '.' && !bottom_left.is_digit(10) {
                return (true, (row+1, start-1));
            }
        }
    }

    if lines[row].len() - 1 != end {
        if lines[row].chars().nth(end+1).unwrap() != '.' {
            return (true, (row, end+1));
        }
        if row != 0 {
            let top_right = lines[row-1].chars().nth(end+1).unwrap();
            if  top_right != '.' && !top_right.is_digit(10) {
                return (true, (row-1, end+1));
            }
        }

        if lines.len() - 1 != row {
            let bottom_right = lines[row+1].chars().nth(end+1).unwrap();
            if bottom_right != '.' && !bottom_right.is_digit(10) {
                return (true, (row+1, end+1));
            }
        }
    }
    
    if row != 0 {
        for (i, c) in lines[row-1][start..end+1].chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                return (true, (row-1, start+i));
            }
        }
    }

    if lines.len() - 1 != row {
        for (i, c) in lines[row+1][start..end+1].chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                return (true, (row+1, start+i));
            }
        }
    }

    (false, (0,0))
}

fn part1(file: &str) -> Result<()> {
    let lines: Vec<&str> = file.lines().collect();
    let mut sum = 0;

    for (i, l) in lines.iter().enumerate() {
        let chars = l.chars().collect::<Vec<char>>();
        let mut cur = 0;
        loop {
            if chars[cur].is_digit(10) {
                let start_cur = cur; 
                cur += 1;

                loop {
                    if chars[cur].is_digit(10) {
                        cur += 1;
                    } else {
                        break;
                    }

                    if cur >= chars.len() {
                        break;
                    }
                }
                let end_cur = cur;
                let check = check_pos(&lines, i, start_cur, end_cur - 1);
                if check.0 {
                    sum += &l[start_cur..end_cur].parse::<i32>()?;
                }
            } else {
                cur += 1;
            }

            if cur >= chars.len() {
                break;
            }
        }
    }

    println!("{sum}");
    Ok(())
}

fn part2(file: &str) -> Result<()> {
    let lines: Vec<&str> = file.lines().collect();
    let mut gear_sum = 0;
    // let mut gears: Vec<((usize, usize), Vec<i32>)> = vec![];
    let mut gears: std::collections::HashMap<(usize, usize), Vec<i32>> = std::collections::HashMap::new();

    for (i, l) in lines.iter().enumerate() {
        let chars = l.chars().collect::<Vec<char>>();
        let mut cur = 0;
        loop {
            if chars[cur].is_digit(10) {
                let start_cur = cur; 
                cur += 1;

                loop {
                    if chars[cur].is_digit(10) {
                        cur += 1;
                    } else {
                        break;
                    }

                    if cur >= chars.len() {
                        break;
                    }
                }
                let end_cur = cur;
                let check = check_pos(&lines, i, start_cur, end_cur - 1);
                if check.0 {
                    // println!("true: {l}  at {start_cur}:{end_cur} `{}`; {}: {:?}", &l[start_cur..end_cur], &lines[check.1.0].chars().nth(check.1.1).unwrap(), check.1);
                    // sum += &l[start_cur..end_cur].parse::<i32>()?;
                    let num = l[start_cur..end_cur].parse::<i32>().unwrap();
                    if let Some(g) = gears.get_mut(&check.1) {
                        g.push(num);
                    } else {
                        gears.insert(check.1, vec![num]);
                    }
                }
            } else if chars[cur] == '.' {
                cur += 1;
            } else {
                cur += 1;
            }

            if cur >= chars.len() {
                break;
            }
        }
    }
    
    for (_k, v) in gears.iter() {
        if v.len() > 1 {
            gear_sum += v[0] * v[1];
        }
    }

    println!("{gear_sum}");
    Ok(())
}

fn main() -> Result<()> {
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;
    part1(&contents);
    // part2(&contents)?;
    Ok(())
}
