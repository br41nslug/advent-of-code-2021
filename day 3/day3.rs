use std::io::{self, BufRead};

fn decode(total: isize, ones: [isize; 12]) -> isize {
    // calculate gamma and epsilon
    let gamma_str: String = ones.iter()
        .map(|one| if one > &(total - one) { '1' } else { '0' })
        .collect();
    let epsilon_str: String = ones.iter()
        .map(|one| if one > &(total - one) { '0' } else { '1' })
        .collect();
    // decode binary
    let gamma = isize::from_str_radix(&*gamma_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&*epsilon_str, 2).unwrap();
    println!("gamma: {} ({})", gamma_str, gamma);
    println!("epsilon: {} ({})", epsilon_str, epsilon);
    // calculate power consumption
    gamma * epsilon
}

fn main() {
    let stdin = io::stdin();
    let mut total: isize = 0;
    let mut ones: [isize; 12] = [0,0,0,0,0,0,0,0,0,0,0,0];
    for line in stdin.lock().lines() {
        for (i, ch) in line.unwrap().chars().enumerate() {
            if ch == '1' {
                ones[i] = ones[i] + 1;
            }
        }
        total = total + 1;
    }
    let power = decode(total, ones);
    println!("\npower consumption: {}", power);
}