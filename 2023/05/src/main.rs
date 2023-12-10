use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse_nums(line: &str) -> Result<Vec<i64>> {
    Ok(line
        .trim()
        .split(" ")
        .filter_map(|e| {
            if e.is_empty() {
                None
            }  else {
                Some(e.parse::<i64>().unwrap())
            }
        })
        .collect())
}
fn part2(contents: &str) -> Result<()> {
    let mut parts = contents.split("\n\n");
    let seeds = parse_nums(
        parts.next().unwrap()
        .split(":")
        .nth(1).unwrap())?;
    let mut min = i64::MAX;

    for i in parts {
        let mut maps_iter = i.split("\n");

        // Text
        maps_iter.next();
        let maps = maps_iter.collect::<Vec<&str>>();

        for seed in seeds.chunks_exact(2) {
            for mut seed in seed[0]..seed[0] + seed[1] {

                println!("{seed}");
                for m in &maps {
                    if m.is_empty() {
                        continue;
                    }
                    let nums = parse_nums(m)?;
                    
                    let dest = nums[0];
                    let src  = nums[1];
                    let len  = nums[2];
                    let dest_offset = seed - src;

                    if dest_offset >= 0 && dest_offset <= len {
                        seed = dest + dest_offset;
                        break;
                    }
                } 

                min = min.min(seed);
            }
            println!("{:?}", seed);
        }
    }
    
    println!("{}, {:?}", min, seeds);
    Ok(())
}

fn part1(contents: &str) -> Result<()> {
    let mut parts = contents.split("\n\n");
    let mut seeds = parse_nums(
        parts.next().unwrap()
        .split(":")
        .nth(1).unwrap())?;

    for i in parts {
        let mut maps_iter = i.split("\n");
        // Text
        maps_iter.next();
        let maps = maps_iter.collect::<Vec<&str>>();

        for s in seeds.iter_mut() {
            for m in &maps {
                if m.is_empty() {
                    continue;
                }
                let nums = parse_nums(m)?;
                
                let dest = nums[0];
                let src  = nums[1];
                let len  = nums[2];
                let dest_offset = *s - src;

                if dest_offset >= 0 && dest_offset <= len {
                    *s = dest + dest_offset;
                    break;
                }
            } 
        }
        println!("{i}");
    }
    println!("{}", seeds.iter().min().unwrap());
    Ok(())
}

fn main() -> Result<()>{
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;
    part2(&contents);
    Ok(())
}
