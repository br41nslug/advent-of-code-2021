use std::io::{self, BufRead};

fn decode_generator(mut lines: Vec<String>, index: usize) -> String {
    let criteria = { // get most common bit
        let mut total: i32 = 0;
        let mut ones: i32 = 0;
        for line in lines.clone() {
            total = total + 1;
            if line.chars().nth(index).unwrap() == '1' {
                ones = ones + 1;
            }
        }
        if ones >= (total - ones) { '1' } else { '0' }
    };
    // filter lines
    lines.retain(|line| line.chars().nth(index).unwrap() == criteria);
    // println!("Generator result[{}, {}]: {:?}", index, criteria, lines);
    // recurse if needed
    if lines.first().unwrap().len() > (index + 1) && lines.len() > 1 {
        decode_generator(lines.to_vec(), index + 1)
    } else {
        lines.first().unwrap().to_string()
    }
}

fn decode_scrubber(mut lines: Vec<String>, index: usize) -> String {
    let criteria = { // get least common bit
        let mut total: i32 = 0;
        let mut zeros: i32 = 0;
        for line in lines.clone() {
            total = total + 1;
            if line.chars().nth(index).unwrap() == '0' {
                zeros = zeros + 1;
            }
        }
        if zeros <= (total - zeros) { '0' } else { '1' }
    };
    // filter lines
    lines.retain(|line| line.chars().nth(index).unwrap() == criteria);
    // println!("Scrubber result[{}, {}]: {:?}", index, criteria, lines);
    // recurse if needed
    if lines.first().unwrap().len() > (index + 1) && lines.len() > 1 {
        decode_scrubber(lines.to_vec(), index + 1)
    } else {
        lines.first().unwrap().to_string()
    }
}

fn decode(lines: Vec<String>) -> isize {
    let generator_str: String = decode_generator(lines.to_vec(), 0);
    let scrubber_str: String = decode_scrubber(lines.to_vec(), 0);
    // decode binary
    let generator = isize::from_str_radix(&*generator_str, 2).unwrap();
    let scrubber = isize::from_str_radix(&*scrubber_str, 2).unwrap();
    println!("Oxygen generator rating: {} ({})", generator_str, generator);
    println!("CO2 scrubber rating: {} ({})", scrubber_str, scrubber);
    // calculate life support
    generator * scrubber
}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        lines.push(line.unwrap() as String);
    }
    let lifesupport = decode(lines);
    println!("\nlife support rating: {}", lifesupport);
}