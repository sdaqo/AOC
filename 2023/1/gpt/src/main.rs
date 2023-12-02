use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Change the file path to your calibration document
    let file_path = "../input.txt";

    // Read the file and sum the calibration values
    let sum = read_calibrations(file_path)?;

    println!("Sum of calibration values: {}", sum);

    Ok(())
}

fn read_calibrations(file_path: &str) -> io::Result<u32> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((first_digit, last_digit)) = extract_digits(&line) {
            let calibration_value = first_digit * 10 + last_digit;
            sum += calibration_value;
        }
    }

    Ok(sum)
}

fn extract_digits(line: &str) -> Option<(u32, u32)> {
    print!("{line}: ");
    let mut first_digit = None;
    let mut last_digit = None;

    for c in line.chars() {
        if let Some(digit) = char_to_digit(c) {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        println!("{first}, {last}");
        Some((first, last))
    } else {
        None
    }
}

fn char_to_digit(c: char) -> Option<u32> {
    match c {
        '0'..='9' => Some(c.to_digit(10).unwrap()),
        'o' => Some(1),
        't' => Some(2),
        'w' => Some(3),
        'h' => Some(4),
        'r' => Some(5),
        'f' => Some(6),
        's' => Some(7),
        'e' => Some(8),
        'n' => Some(9),
        _ => None,
    }
}

