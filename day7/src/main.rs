use clap::App;
use clap::Arg;
use std::collections::HashMap;

fn get_min_max(positions: &Vec<i32>) -> (i32, i32) {
    (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    )
}

fn get_p1_cost(positions: &Vec<i32>, position: i32) -> i32 {
    let mut cost = 0i32;
    for pos in positions {
        cost += &(pos - position).abs();
    }

    cost
}

fn get_p2_cost(positions: &Vec<i32>, position: i32, known_sums: &mut HashMap<i32, u64>) -> u64 {
    let mut cost = 0u64;
    for pos in positions {
        let diff = &(pos - position).abs();
        cost += *known_sums
            .entry(*diff)
            .or_insert((0u64..(*diff + 1) as u64).sum::<u64>());
    }

    cost
}

fn get_positions(line: &String) -> Vec<i32> {
    line.split(",").map(|p| p.parse::<i32>().unwrap()).collect()
}

fn p1(lines: &Vec<String>) {
    let positions: Vec<i32> = get_positions(&lines[0]);
    let (min, max) = get_min_max(&positions);
    let mut min_cost = i32::MAX;
    let mut min_pos = i32::MAX;
    for pos in min..max {
        let cost = get_p1_cost(&positions, pos);
        if cost < min_cost {
            min_cost = cost;
            min_pos = pos;
        }
    }

    println!("{} (cost: {})", min_pos, min_cost);
}

fn p2(lines: &Vec<String>) {
    let mut known_sums: HashMap<i32, u64> = HashMap::new();
    let positions: Vec<i32> = get_positions(&lines[0]);
    let (min, max) = get_min_max(&positions);
    let mut min_cost = u64::MAX;
    let mut min_pos = i32::MAX;
    for pos in min..max + 1 {
        let cost = get_p2_cost(&positions, pos, &mut known_sums);
        if cost < min_cost {
            min_cost = cost;
            min_pos = pos;
        }
    }

    println!("{} (cost: {})", min_pos, min_cost);
}

fn main() {
    let matches = App::new("day7")
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
        common::read_file("/Users/alexeyler/Development/aoc/2021/day7/src/data/crabs.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
