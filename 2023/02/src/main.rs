use regex::Regex;


const MAX_RED: u32   = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32  = 14;

fn split_line<'a> (re: &'a Regex, line: &'a str) -> Result<(i32, String), Box<dyn std::error::Error>> {
    let Some(caps) = re.captures(line) else {
        return Err(format!("`{}`, No ID Match", line).into())
    };

    return Ok((caps["id"].parse::<i32>()?, caps["rest"].to_string()))
}


fn parse_colors(group: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;


        
    for g in group.split(",") {
       let mut iter = g.chars().into_iter();

       let mut str_num = "".to_string();

       let col_char = loop {
           let c = iter.next().unwrap();

           if c.is_digit(10) {
               str_num.push(c);
           } else {
               break c;
           }
       };
    
       let num: u32 = str_num.parse().unwrap();

       match col_char {
           'g' => {
               green += num;
           },
           'r' => {
               red += num;
           },
           'b' => {
               blue += num                
           },
           _   => {
           }
       }
    }

    (red, green, blue)
}

fn part1(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let id_re: Regex = Regex::new(r"Game (?<id>\d+):(?<rest>.*)").unwrap();
    
    let mut id_sum = 0;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let (id, rest) = split_line(&id_re, line)?;
        let rest_no_ws = rest.replace(" ", "");
        let groups = rest_no_ws.split(";");
        
        let mut possible = true;
        for g in groups {
            let colors = parse_colors(g);
            println!("{:?}", colors);
            if colors.0 > MAX_RED || colors.1 > MAX_GREEN || colors.2 > MAX_BLUE {
                possible = false;
                break;
            } 
        }

        if possible {
            id_sum += id;
        }
    }

    println!("The ID Sum is: {id_sum}");
    Ok(())
}

fn part2(contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    let id_re: Regex = Regex::new(r"Game (?<id>\d+):(?<rest>.*)").unwrap();

    let mut power_sum = 0;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let (_id, rest) = split_line(&id_re, line)?;
        let rest_no_ws = rest.replace(" ", "");
        let groups = rest_no_ws.split(";");


        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for g in groups {
            let colors = parse_colors(g);
            if colors.0 > min_red {
                min_red = colors.0;
            }

            if colors.1 > min_green {
                min_green = colors.1;
            }

            if colors.2 > min_blue {
                min_blue = colors.2;
            }
        }

        power_sum += min_red * min_green * min_blue;
    }

    println!("The POWER Sum is: {power_sum}");
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().nth(1).expect("No File Specified");
    let contents = std::fs::read_to_string(file)?;

    // part1(&contents)?;
    part2(&contents)?;

    Ok(())
}
