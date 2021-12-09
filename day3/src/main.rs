use common;
use std::usize;
use std::env;
use std::process;

fn main() {
    let lines: Vec<String> = common::read_file("/Users/alexeyler/Development/aoc/2021/day3/src/data/diagnostics.txt");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Requires one argument (the problem)!");
        process::exit(1);
    }
    let subproblem = &args[1];
    match subproblem.as_str() {
        "1" => p1(lines),
        "2" => p2(lines),
        _ => eprintln!("Bad arg value: {}", subproblem),
    }
}

fn p1(lines: Vec<String>) {
    let line_length = lines[0].len();
    let mut ones: Vec<usize> = vec![0;line_length];

    for line in &lines {
        let mut c_index = 0;
        for c in line.chars() {
            if c == '1' {
                ones[c_index] = &ones[c_index] + 1;
            }
            c_index = &c_index + 1;
        }
    }

    let half_line_length: usize = lines.len() / 2;
    let mut bits: Vec<u8> = vec![0;ones.len()];
    for (pos, num_ones) in ones.iter().enumerate() {
        if num_ones >= &half_line_length {
            bits[pos] = 1;
        }
    }

    let (gamma, epsilon): (i32, i32) = to_gamma_eps(bits);
    println!("{} • {} = {}", gamma, epsilon, gamma * epsilon);
}

fn filter(line: &String, pos: usize, bit: &u8) -> bool {
    let ch = line.chars().nth(pos).unwrap();
    return (bit == &0 && ch == '0') ||
            (bit == &1 && ch == '1');
}

fn get_count(lines: &Vec<String>, is_o2_generator_rating: bool ) -> u32{
    let mut remaining_lines = lines.to_owned();
    let mut pos = 0;
    while remaining_lines.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;
        for line in &remaining_lines {
            let ch = line.chars().nth(pos).unwrap();
            if ch == '1' {
                ones = &ones + 1;
            } else {
                zeros = &zeros + 1;
            }
        }

        let bit = if is_o2_generator_rating {
            if ones >= zeros { 1u8 } else { 0u8 }
        } else {
            if ones < zeros { 1u8 } else { 0u8 }
        };

        remaining_lines = remaining_lines
            .into_iter()
            .filter(|line| filter(line, pos, &bit))
            .collect();
        pos = &pos + 1;
    }

    let mut result = 0;
    let base: u32 = 2;
    for (pos, c) in remaining_lines[0].chars().rev().enumerate() {
        if c == '1' {
            result = result + base.pow(pos as u32);
        }
    }

    return result;
}

fn p2(lines: Vec<String>) {
    let o2_generator_rating = get_count(&lines, true);
    let co2_scrubber_rating = get_count(&lines, false);

    println!("{} • {} = {}", o2_generator_rating, co2_scrubber_rating, o2_generator_rating * co2_scrubber_rating);
}

fn to_gamma_eps(bits: Vec<u8>) -> (i32, i32) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    for (i, value) in bits.iter().rev().enumerate() {
        let power: i32 = base.pow(i as u32);
        if value == &(true as u8) {
            gamma = gamma + power;
        }
        else {
            epsilon = epsilon + power;
        }
    }

    return (gamma, epsilon);
}
