use clap::App;
use clap::Arg;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const TIMER_RESTART: i32 = 6;
const TIMER_NEW: i32 = 8;

struct Lanternfish {
    timer: i32,
}

fn print_fish_timers(day: i32, fishes: &Vec<Lanternfish>) {
    let timers: Vec<i32> = fishes.iter().map(|f| f.timer).collect();
    if day == 0 {
        println!("Initial state: {:?}", timers)
    } else {
        println!("After {} days: {:?}", day, timers);
    }
}

fn print_map(day: i32, map: &HashMap<i32, u64>) {
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by_key(|k| k.0);

    println!("Day {}", &day);
    for i in sorted {
        println!("{}: {}", &i.0, &i.1);
    }
    println!();
}

fn get_sum(map: &HashMap<i32, u64>) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..9 {
        sum += map[&i];
    }
    sum
}

fn p1(lines: &Vec<String>, days: i32, debug: bool) {
    let initial_state_line = lines.get(0).unwrap();
    let mut fishes: Vec<Lanternfish> = initial_state_line
        .split(',')
        .map(|t| Lanternfish {
            timer: t.parse::<i32>().unwrap(),
        })
        .collect();
    if debug {
        print_fish_timers(0, &fishes)
    };
    for day in 1..days + 1 {
        println!("Day {}", day);
        let new_fishes = Arc::new(Mutex::new(0));
        fishes.par_iter_mut().for_each(|fish| {
            fish.timer -= 1;
            if fish.timer == -1 {
                fish.timer = TIMER_RESTART;
                *new_fishes.lock().unwrap() += 1;
            }
        });
        for _ in 0..*new_fishes.lock().unwrap() {
            fishes.push(Lanternfish { timer: TIMER_NEW })
        }
        if debug {
            print_fish_timers(day, &fishes);
        }
    }

    println!("{} fishies", fishes.len());
}

fn p2(lines: &Vec<String>, days: i32) {
    let initial_state_line = lines.get(0).unwrap();
    let mut map: HashMap<i32, u64> = HashMap::new();
    for i in 0..9 {
        map.insert(i, 0);
    }
    let initial_timers: Vec<i32> = initial_state_line
        .split(',')
        .map(|t| t.parse::<i32>().unwrap())
        .collect();
    for timer in initial_timers {
        let entry = map.entry(timer).or_insert(0);
        *entry += 1;
    }
    for day in 1..days + 1 {
        let expiring_timers = map[&0];
        for i in 1..9 {
            let prev_i = i - 1;
            map.insert(prev_i, map[&i]);
        }
        map.insert(6, expiring_timers + map[&6]);
        map.insert(8, expiring_timers);
    }

    let mut sum: u64 = 0;
    for i in 0..9 {
        sum += map[&i];
    }

    println!("Fishies: {}", sum);
}

fn main() {
    let matches = App::new("day6")
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
            Arg::with_name("days")
                .short("d")
                .long("days")
                .value_name("DAYS")
                .help("# of days")
                .required(true),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .value_name("DEBUG")
                .takes_value(false)
                .help("Enable debug mode"),
        )
        .get_matches();

    let part = matches.value_of("part").unwrap();
    let days = matches.value_of("days").unwrap().parse::<i32>().unwrap();
    let debug = matches.is_present("debug");
    let lines: Vec<String> =
        common::read_file("/Users/alexeyler/Development/aoc/2021/day6/src/data/fish.txt");
    match part {
        "1" => p1(&lines, days, debug),
        "2" => p2(&lines, days),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
