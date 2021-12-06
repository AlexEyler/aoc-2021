use std::env;
use std::fs;
use std::process;

struct Vector {
    magnitude: i32,
    direction: String,
}

impl Vector {
    fn parse(line: &String) -> Result<Vector, &'static str> {
        let parts: Vec<String> = line.split(' ').map(|s| String::from(s.trim())).collect();
        if parts.len() < 2 {
            return Err("Could not find parts");
        }
        let direction = parts[0].to_string();
        let magnitude = parts[1].parse().expect("Could not parse magnitude");
        return Ok(Vector {
            direction,
            magnitude,
        });
    }
}

fn get_course() -> Vec<Vector> {
    let fname = "/Users/alexeyler/Development/aoc/2021/day2/src/data/course.txt";
    let contents = fs::read_to_string(fname).expect("Couldn't read.");
    let contents_lines: Vec<String> = contents
        .split('\n')
        .map(|s| String::from(s.trim()))
        .collect();
    let mut course: Vec<Vector> = Vec::new();
    for l in 0..contents_lines.len() {
        let line = &contents_lines[l];
        course.push(Vector::parse(line).expect("Error parsing line"));
    }

    return course;
}

fn p1() {
    let course = get_course();

    let mut h_pos = 0;
    let mut depth = 0;
    for vector in course {
        match vector.direction.as_str() {
            "forward" => h_pos = &h_pos + vector.magnitude,
            "up" => depth = &depth - vector.magnitude,
            "down" => depth = &depth + vector.magnitude,
            _ => panic!("Unknown vector direction"),
        }
    }
}

fn p2() {
    let course = get_course();

    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for vector in course {
        match vector.direction.as_str() {
            "forward" => {
                h_pos = &h_pos + vector.magnitude;
                depth = &depth + (aim * vector.magnitude)
            }
            "up" => {
                aim = &aim - vector.magnitude;
            }
            "down" => {
                aim = &aim + vector.magnitude;
            }
            _ => panic!("Unknown vector direction"),
        };
        println!("h_pos = {}, depth = {}, aim = {}", h_pos, depth, aim)
    }

    println!("{} • {} = {}", h_pos, depth, depth * h_pos);
}

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
