use std::error::Error;
use std::collections::HashMap;
use regex::Regex;
use num::integer::Integer;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn traverse(starting_node: &str, directions: &HashMap<String, (String, String)>, instructions: &str) -> i32 {
    let mut steps = 0;
    let mut inst_iter = instructions.chars();
    let mut current_node = starting_node;

    while !current_node.ends_with("Z") {
        if let Some(inst) = inst_iter.next() {
            let node = directions.get(current_node).unwrap();
            if inst == 'L' {
                current_node = &node.0;
            } else {
                current_node = &node.1;
            }

            steps += 1;
        } else {
            inst_iter = instructions.chars();
            continue;
        }
    }

    steps
}

fn part1(contents: &str) -> Result<()> {
    let mut lines = contents.lines();
    let instructions = lines.next().unwrap();

    lines.next();

    let mut directions: HashMap<String, (String, String)> = HashMap::new();
    let re = Regex::new(r"(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)")?;

    for i in lines {
        let Some(caps) = re.captures(i) else {
            return Err(format!("`{}`, No Match", i).into())
        };

        directions.insert(
            caps["name"].to_string(),
            (caps["left"].to_string(), caps["right"].to_string())
        );
    }

    let steps = traverse("AAA", &directions, instructions);

    println!("Steps to traverse: {steps}");

    Ok(())
}

fn part2(contents: &str) -> Result<()> {
    let mut lines = contents.lines();
    let instructions = lines.next().unwrap();

    lines.next();

    let mut directions: HashMap<String, (String, String)> = HashMap::new();
    let re = Regex::new(r"(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)")?;

    for i in lines {
        let Some(caps) = re.captures(i) else {
            return Err(format!("`{}`, No Match", i).into())
        };

        directions.insert(
            caps["name"].to_string(),
            (caps["left"].to_string(), caps["right"].to_string())
        );
    }

    let starting_positions = directions.keys().filter(|k| {k.ends_with('A')}).collect::<Vec<_>>();
    let mut steps: Vec<i64> = vec![];

    for p in starting_positions {
        steps.push(traverse(p, &directions, instructions).into());
    }

    let steps = steps.into_iter().reduce(|a, i| { a.lcm(&i) }).unwrap();
    println!("{steps}");

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
