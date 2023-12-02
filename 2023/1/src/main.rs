use std::{fs::read_to_string};


fn main() {
    let input = read_to_string("input.txt").expect("File not found!");
    let mut num = 0;
    for i in input.split("\n") {
        let word = &[i, "xxxxxx"].concat();
        let mut num_vec: Vec<char> = vec![];
        let mut cur = 0;
        for s in word.chars() {
            if s.is_numeric() {
                 num_vec.push(s);
             }


            match s {
                'o' => {
                    if &word[cur..cur+3] == "one" {
                        num_vec.push('1');
                    }
                },
                't' => {
                    if &word[cur..cur+3] == "two" {
                        num_vec.push('2');
                    } else if &word[cur..cur+5] == "three" {
                        num_vec.push('3');
                    }
                },
                'f' => {
                    if &word[cur..cur+4] == "four" {
                        num_vec.push('4');
                    } else if &word[cur..cur+4] == "five" {
                        num_vec.push('5');
                    }
                },
                's' => {
                    if &word[cur..cur+3] == "six" {
                        num_vec.push('6');
                    } else if &word[cur..cur+5] == "seven" {
                        num_vec.push('7');
                    }
                },
                'e' => {
                    if &word[cur..cur+5] == "eight" {
                        num_vec.push('8')
                    }
                },
                'n' => {
                    if &word[cur..cur+4] == "nine" {
                        num_vec.push('9')
                    }
                },
                _   => {}
            }
            cur += 1;
        }
        println!("{:?}", num_vec);
        if num_vec.len() >= 1 {
            if let Ok(x) = [num_vec.first().copied().unwrap(), num_vec.last().copied().unwrap()].iter().collect::<String>().parse::<i32>() {
                num += x;
            }
        }
    }

    println!("{num}");
}
