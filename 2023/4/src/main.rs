use std::error::Error;


type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn part2(contents: &str) -> Result<()>{
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut cards: Vec<i32> = vec![0; lines.len()];

    for (i, l) in lines.iter().enumerate() {
        let games  = l.split(":").nth(1).unwrap().split("|").collect::<Vec<&str>>();
        let winning = games[0]
            .trim().split(" ")
            .filter_map(|e| { 
                if !e.is_empty() { 
                    Some(e.trim().parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<i32>>();

        let ours = games[1]
            .trim().split(" ")
            .filter_map(|e| { 
                if !e.is_empty() { 
                    Some(e.trim().parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<i32>>();
        let inter = winning.intersection(&ours).count();
        cards[i] += 1;

        for j in i+1..i+inter+1 {
            cards[j] += cards[i];
        }
    }

    println!("{}", cards.iter().sum::<i32>());
    Ok(())
}




fn part1(contents: &str) -> Result<()>{
    let mut sum = 0;
    for l in contents.lines() {
        let games  = l.split(":").nth(1).unwrap().split("|").collect::<Vec<&str>>();
        let winning = games[0]
            .trim().split(" ")
            .filter_map(|e| { 
                if !e.is_empty() { 
                    Some(e.trim().parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<i32>>();

        let ours = games[1]
            .trim().split(" ")
            .filter_map(|e| { 
                if !e.is_empty() { 
                    Some(e.trim().parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<i32>>();
        let inter = winning.intersection(&ours).count() as u32;
        if inter != 0 {
            sum += 2_i32.pow(inter - 1);
        }

    }

    println!("{sum}");
    Ok(())
}

fn main() -> Result<()> {
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;
    part2(&contents)?;
    Ok(())
}
