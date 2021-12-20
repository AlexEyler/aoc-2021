use clap::App;
use clap::Arg;
use std::collections::HashMap;

fn get_patterns_outputs(lines: &Vec<String>) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let mut patterns: Vec<Vec<&str>> = Vec::new();
    let mut outputs: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split("|").filter(|p| !p.is_empty()).collect();
        let line_patterns_untokenized = parts[0];
        let line_outputs_untokenized = parts[1];
        let line_patterns = line_patterns_untokenized
            .split(" ")
            .filter(|p| !p.is_empty())
            .collect();
        let line_outputs = line_outputs_untokenized
            .split(" ")
            .filter(|p| !p.is_empty())
            .collect();
        patterns.push(line_patterns);
        outputs.push(line_outputs);
    }

    (patterns, outputs)
}

fn get_segment_map<'a>(pattern: &Vec<&'a str>) -> HashMap<&'a str, usize> {
    let mut segments: HashMap<usize, &str> = HashMap::new();
    let mut unknown_segments: Vec<&str> = Vec::new();
    for segment in pattern {
        match segment.len() {
            2 => {
                segments.insert(1, segment);
                ()
            }
            3 => {
                segments.insert(7, segment);
                ()
            }
            4 => {
                segments.insert(4, segment);
                ()
            }
            7 => {
                segments.insert(8, segment);
                ()
            }
            _ => unknown_segments.push(segment),
        };
    }
    if !segments.contains_key(&1)
        || !segments.contains_key(&4)
        || !segments.contains_key(&7)
        || !segments.contains_key(&8)
    {
        panic!("Couldn't find patterns for 1, 4, 7, or 8");
    }
    for segment in unknown_segments {
        if segment.len() == 5 {
            let segment_without_one: Vec<char> = segment
                .chars()
                .filter(|&c| !segments[&1].contains(c))
                .collect();
            // A 3 is the only 5-length digit that contains all the segments in a one digit
            if segment_without_one.len() == 3 {
                segments.insert(3, segment);
            } else {
                let segment_without_four: Vec<char> = segment
                    .chars()
                    .filter(|&c| !segments[&4].contains(c))
                    .collect();
                if segment_without_four.len() == 3 {
                    segments.insert(2, segment);
                } else {
                    segments.insert(5, segment);
                }
            }
        } else if segment.len() == 6 {
            let segment_without_one: Vec<char> = segment
                .chars()
                .filter(|&c| !segments[&1].contains(c))
                .collect();
            // A 6 is the only 6-length digit that does not contain all the segments in a one digit
            if segment_without_one.len() == 5 {
                segments.insert(6, segment);
            } else {
                let segment_without_four: Vec<char> = segment
                    .chars()
                    .filter(|&c| !segments[&4].contains(c))
                    .collect();
                // A 9 is the only 6-length digit that does not contain all the segments in a four digit
                if segment_without_four.len() == 2 {
                    segments.insert(9, segment);
                } else {
                    // A 0 is the only other remaining 6-length digit
                    segments.insert(0, segment);
                }
            }
        }
    }

    let mut invert_hash_map: HashMap<&'a str, usize> = HashMap::new();
    for segment in segments {
        invert_hash_map.insert(segment.1, segment.0);
    }

    invert_hash_map
}

fn get_digit(
    segments: &HashMap<&str, usize>,
    output_segment: &str,
) -> std::result::Result<usize, &'static str> {
    for entry in segments {
        if entry.0.len() != output_segment.len() {
            continue;
        }
        let segment_chars = entry.0.chars();
        let intersection: Vec<char> = segment_chars
            .filter(|&c| !output_segment.contains(c))
            .collect();
        if intersection.len() == 0 {
            return Ok(*entry.1);
        }
    }
    Err("Could not find segment in map")
}

fn p1(lines: &Vec<String>) {
    let (_, outputs) = get_patterns_outputs(&lines);
    let mut count = 0i32;
    for output in outputs {
        for segment in output {
            count += match segment.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }
    }

    println!("{}", count);
}

fn p2(lines: &Vec<String>) {
    let (patterns, outputs) = get_patterns_outputs(&lines);
    let mut sum: i32 = 0;
    for i in 0..patterns.len() {
        let pattern = &patterns[i];
        let output = &outputs[i];
        let segments = get_segment_map(&pattern);
        if segments.len() != 10 {
            panic!(format!("Missing segment entry for pattern {:?}", pattern));
        }
        let mut number_string: String = String::new();
        for segment in output {
            let digit = get_digit(&segments, segment).unwrap();
            number_string += &digit.to_string();
        }
        let number = number_string.parse::<i32>().unwrap();
        sum += number;
        println!("{:?}: {}", output, number);
    }
    println!("Total: {}", sum);
}

fn main() {
    let matches = App::new("day8")
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
        common::read_file("/Users/alexeyler/Development/aoc/2021/day8/src/data/signals.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
