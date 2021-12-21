use clap::App;
use clap::Arg;
use colored::*;
use std::collections::LinkedList;

static LEFT_CHARS: [char; 4] = ['(', '[', '{', '<'];
static RIGHT_CHARS: [char; 4] = [')', ']', '}', '>'];
static COSTS: [i32; 4] = [3, 57, 1197, 25137];
static SCORES: [u64; 4] = [1, 2, 3, 4];

fn parse(line: &str) -> (i32, String, LinkedList<char>) {
    let mut stack: LinkedList<char> = LinkedList::new();
    let mut cost = 0i32;
    let mut err_msg = "".to_string();
    for c in line.chars() {
        if LEFT_CHARS.contains(&c) {
            stack.push_front(c);
        } else if RIGHT_CHARS.contains(&c) {
            let left = stack.pop_front().unwrap();
            let left_position = LEFT_CHARS.iter().position(|&lc| lc == left).unwrap();
            let right_position = RIGHT_CHARS.iter().position(|&rc| rc == c).unwrap();
            if left_position != right_position {
                cost += COSTS[right_position];
                err_msg = format!(
                    "Expected {}, but found {} instead.",
                    &RIGHT_CHARS[left_position], &RIGHT_CHARS[right_position]
                );
                break;
            }
        }
    }
    (cost, err_msg, stack)
}

fn autocomplete(stack: &mut LinkedList<char>) -> (String, u64) {
    let mut completion: String = "".to_string();
    let mut score: u64 = 0;
    while !stack.is_empty() {
        let current_char = stack.pop_front().unwrap();
        let left_position = LEFT_CHARS.iter().position(|&c| c == current_char).unwrap();
        let right_char = RIGHT_CHARS[left_position];
        completion += &right_char.to_string();
        score *= 5;
        score += SCORES[left_position];
    }
    (completion, score)
}

fn p1(lines: &Vec<String>) {
    let mut total_cost: i32 = 0;
    for line in lines {
        let (cost, err_msg, _) = parse(line);
        if !err_msg.is_empty() {
            eprintln!("{}", err_msg.red())
        }
        total_cost += cost;
    }
    println!("Total cost: {}", total_cost);
}

fn p2(lines: &Vec<String>) {
    let mut scores: Vec<u64> = Vec::new();
    for line in lines {
        let (_, err_msg, mut remaining) = parse(line);
        if err_msg.is_empty() {
            let (completion, score) = autocomplete(&mut remaining);
            println!("Completing {} with {}", line, completion);
            scores.push(score);
        }
    }
    scores.sort();
    let middle_score = scores[scores.len() / 2];
    println!("{:?}", middle_score);
}

fn main() {
    let matches = App::new("day10")
        .version("1.0")
        .author("Alex Eyler <alex.eyler@outlook.com>")
        .about("Advent of code")
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .help("Which part")
                .required(true),
        )
        .get_matches();
    let part = matches.value_of("part").unwrap();
    let lines: Vec<String> =
        common::read_file("/Users/alexeyler/Development/aoc/2021/day10/src/data/syntax.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
