use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Requires one argument (the problem)!");
        process::exit(1);
    }
    let subproblem = &args[1];
    match subproblem.as_str() {
        "1" => p1(),
        "2" => p2(),
        _ => eprintln!("Bad arg value: {}", subproblem),
    }
}

fn get_depths() -> Vec<i32> {
    let fname = "/Users/alexeyler/Development/aoc/2021/day1/src/data/depths.txt";
    let contents = fs::read_to_string(fname).expect("Couldn't read.");
    let contents_lines: Vec<String> = contents
        .split('\n')
        .map(|s| String::from(s.trim()))
        .collect();
    let mut results: Vec<i32> = Vec::new();
    for l in 0..contents_lines.len() {
        let line = &contents_lines[l];
        results.push(line.parse().expect(&format!("Couldn't parse {}", line)));
    }
    return results;
}

fn find_maxima(results: Vec<i32>) {
    let mut n: i32 = 0;
    for i in 0..results.len() {
        if i == 0 {
            println!("{} (N/A - no previous sum)", results[i]);
            continue;
        }
        if results[i] > results[i - 1] {
            println!("{} (increased)", results[i]);
            n = &n + 1;
        } else if results[i] == results[i - 1] {
            println!("{} (no change)", results[i]);
        } else {
            println!("{} (decrased)", results[i]);
        }
    }
    println!("The number of measurements that are larger than the previous measurement is:");
    println!("{}", n);
}

fn get_window_depths(depths: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    for i in 0..depths.len() {
        if i + 2 < depths.len() {
            results.push(depths[i] + depths[i + 1] + depths[i + 2]);
        }
    }

    return results;
}

fn p1() {
    let depths = get_depths();
    find_maxima(depths);
}

fn p2() {
    let depths = get_depths();
    let window_depths = get_window_depths(depths);
    find_maxima(window_depths);
}
