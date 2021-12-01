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

fn p1() {
    let depths = get_depths();
    let mut n: i32 = 0;
    for i in 0..depths.len() {
        if i == 0 {
            println!("{} (N/A)", depths[i]);
            continue;
        }
        if depths[i] > depths[i - 1] {
            println!("{} (increased)", depths[i]);
            n = &n + 1;
        } else {
            println!("{} (decreased)", depths[i]);
        }
    }
    println!("The number of measurements that are larger than the previous measurement is:");
    println!("{}", n)
}

fn p2() {}
