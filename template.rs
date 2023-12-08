use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn part1(contents: &str) -> Result<()> {
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
