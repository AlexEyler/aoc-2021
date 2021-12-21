use clap::App;
use clap::Arg;
use queues::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Cave {
    name: String,
    is_small: bool,
    is_end: bool,
    is_start: bool,
    neighbors: HashSet<String>,
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn print_neighbors(caves: &HashMap<String, Cave>) {
    for cave in caves.values() {
        println!("{}: {:?}", cave.name, cave.neighbors);
    }
}

fn parse(lines: &Vec<String>) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split("-").collect();
        {
            let cave = caves.entry(parts[0].to_string()).or_insert(Cave {
                name: parts[0].to_string(),
                is_small: parts[0].to_lowercase() == parts[0],
                is_start: parts[0] == "start",
                is_end: parts[0] == "end",
                neighbors: HashSet::new(),
            });
            cave.neighbors.insert(parts[1].to_string());
        }
        {
            let cave = caves.entry(parts[1].to_string()).or_insert(Cave {
                name: parts[1].to_string(),
                is_small: parts[1].to_lowercase() == parts[1],
                is_start: parts[1] == "start",
                is_end: parts[1] == "end",
                neighbors: HashSet::new(),
            });
            cave.neighbors.insert(parts[0].to_string());
        }
    }
    caves
}

fn multiple_small_cave_ok(path: &Vec<&Cave>, candidate: &Cave) -> bool {
    if !candidate.is_small {
        return true;
    }
    if candidate.is_start {
        return false;
    }
    if !path.contains(&candidate) {
        return true;
    }
    if path.contains(&candidate) {
        let mut seen: Vec<&Cave> = Vec::new();
        for node in path {
            if node.is_small {
                if seen.contains(node) {
                    return false;
                }
                seen.push(node);
            }
        }
        return true;
    }
    false
}

fn find_paths_p2(caves: &HashMap<String, Cave>) -> Vec<Vec<&Cave>> {
    let mut paths: Vec<Vec<&Cave>> = Vec::new();
    let start = caves.get(&"start".to_string()).unwrap();
    let mut queue: Queue<Vec<&Cave>> = Queue::new();
    queue.add(vec![start]).unwrap();
    while queue.size() != 0 {
        let current = queue.remove().unwrap();
        let last_cave = current.last().unwrap();

        if last_cave.is_end {
            paths.push(current.to_vec());
        } else {
            for neighbor in &last_cave.neighbors {
                let neighbor_cave = caves.get(neighbor).unwrap();
                if multiple_small_cave_ok(&current, neighbor_cave) {
                    let mut new_path = current.to_vec();
                    new_path.push(neighbor_cave);
                    queue.add(new_path).unwrap();
                }
            }
        }
    }
    paths
}

fn find_paths(caves: &HashMap<String, Cave>) -> Vec<Vec<&Cave>> {
    let mut paths: Vec<Vec<&Cave>> = Vec::new();
    let start = caves.get(&"start".to_string()).unwrap();
    let mut queue: Queue<Vec<&Cave>> = Queue::new();
    queue.add(vec![start]).unwrap();
    while queue.size() != 0 {
        let current = queue.remove().unwrap();
        let last_cave = current.last().unwrap();

        if last_cave.is_end {
            paths.push(current.to_vec());
        } else {
            for neighbor in &last_cave.neighbors {
                let neighbor_cave = caves.get(neighbor).unwrap();
                if !neighbor_cave.is_small || !current.contains(&neighbor_cave) {
                    let mut new_path = current.to_vec();
                    new_path.push(neighbor_cave);
                    queue.add(new_path).unwrap();
                }
            }
        }
    }
    paths
}

fn p1(lines: &Vec<String>, debug: bool) {
    let caves = parse(lines);
    // print_neighbors(&caves);
    let paths = find_paths(&caves);
    println!("Found {} paths!", paths.len());
    if debug {
        for path in paths {
            for cave in path {
                print!("{},", cave.name);
            }
            println!();
        }
    }
}

fn p2(lines: &Vec<String>, debug: bool) {
    let caves = parse(lines);
    // print_neighbors(&caves);
    let paths = find_paths_p2(&caves);
    println!("Found {} paths!", paths.len());
    if debug {
        for path in paths {
            for cave in path {
                print!("{},", cave.name);
            }
            println!();
        }
    }
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
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .value_name("DEBUG")
                .help("Enable debug")
                .takes_value(false),
        )
        .get_matches();
    let part = matches.value_of("part").unwrap();
    let debug = matches.is_present("debug");
    let lines: Vec<String> =
        common::read_file("/Users/alexeyler/Development/aoc/2021/day12/src/data/cave-system.txt");
    match part {
        "1" => p1(&lines, debug),
        "2" => p2(&lines, debug),
        _ => eprintln!("Unknown part {}", part),
    }
}
