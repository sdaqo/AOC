use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse_nums(line: &str) -> Result<Vec<i32>> {
    Ok(line
        .trim()
        .split(" ")
        .filter_map(|e| {
            if e.is_empty() {
                None
            }  else {
                Some(e.parse::<i32>().unwrap())
            }
        })
        .collect())
}

fn create_diff(nums: &Vec<i32>) -> Vec<i32> {
    let mut new_nums: Vec<i32> = vec![];
    for (i, a) in nums.iter().enumerate() {
        if let Some(b) = nums.iter().nth(i+1) {
            new_nums.push(*b - a);
        } else {
            break;
        }
    }
    new_nums
}
fn part1(contents: &str) -> Result<()> {
    let mut prediction_sum = 0;
    for l in contents.lines()  {
        let nums = parse_nums(l)?;

        let mut diff: Vec<i32> = nums.clone();
        let mut pred_offset = 0;
        loop {
            diff = create_diff(&diff);

            if diff.iter().all(|e| *e == 0) {
                break;
            }
            pred_offset += diff.iter().last().unwrap();
        }

        prediction_sum += nums.iter().last().unwrap() + pred_offset;
    }
    
    println!("{prediction_sum}");
    Ok(())
}

fn part2(contents: &str) -> Result<()> {
    let mut prediction_sum = 0;
    for l in contents.lines()  {
        let nums = parse_nums(l)?;

        let mut diff: Vec<i32> = nums.clone();
        let mut pred_offsets: Vec<i32>= vec![];
        loop {
            diff = create_diff(&diff);
            pred_offsets.push(diff[0]);
            if diff.iter().all(|e| *e == 0) {
                break;
            }
        }

        pred_offsets.reverse();
        prediction_sum += nums[0] - pred_offsets.iter().fold(0, |acc, i| {
            i - acc
        });
    }
    
    println!("{prediction_sum}");
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
