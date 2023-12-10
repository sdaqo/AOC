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

fn part1(contents: &str) -> Result<()> {
    let mut lines = contents.lines();
    let times = parse_nums(lines.next().unwrap().split(":").nth(1).unwrap())?;
    let records = parse_nums(lines.next().unwrap().split(":").nth(1).unwrap())?;
    let mut counts = 0;
    for (t, r) in std::iter::zip(times, records) {
        let mut count_better = 0; 
        for i in 0..t+1 {
            if (t - i) * i > r { count_better += 1; }
        }
        if counts > 0 {
            counts = counts * count_better;
        } else {
            counts = count_better;
        }
    }

    println!("{}", counts);

    Ok(())
}

fn part2(contents: &str) -> Result<()> {
    let mut lines = contents.lines();
    let times = parse_nums(lines.next().unwrap().split(":").nth(1).unwrap())?;
    let records = parse_nums(lines.next().unwrap().split(":").nth(1).unwrap())?;
    let time = times[0];
    let record = records[0];

    println!("{}, {}", time, record);
    let mut counts = 0;
    for i in 0..time+1 {
        if (time - i) * i > record { counts += 1; }
    }

    println!("{}", counts);

    Ok(())

}

fn main() -> Result<()>{
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;
    //part1(&contents);
    part2(&contents);
    Ok(())
}
