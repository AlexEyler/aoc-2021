use clap::App;
use clap::Arg;

struct Fold {
    is_vertical: bool,
    line: usize,
}

fn get_min_max(lines: &Vec<String>) -> ((usize, usize), (usize, usize)) {
    let mut x = (0, usize::MIN);
    let mut y = (0, usize::MIN);
    for line in lines {
        if !line.starts_with("fold along") && !line.is_empty() {
            let parts: Vec<usize> = line
                .split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            if parts[0] > x.1 {
                x.1 = parts[0];
            }
            if parts[1] > y.1 {
                y.1 = parts[1];
            }
        }
    }
    (x, y)
}

fn print_paper(paper: &Vec<Vec<bool>>, fold: Option<&Fold>) {
    for y in 0..paper.len() {
        if fold.is_some() && fold.unwrap().is_vertical && y == fold.unwrap().line {
            print!("{}", "-".repeat(paper[y].len()))
        } else {
            for x in 0..paper[y].len() {
                if fold.is_some() && !fold.unwrap().is_vertical && x == fold.unwrap().line {
                    print!("|");
                } else {
                    print!("{}", if paper[y][x] { "#" } else { "." })
                }
            }
        }
        println!();
    }
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut folds: Vec<Fold> = Vec::new();
    let ((_, max_x), (_, max_y)) = get_min_max(lines);
    let mut paper = vec![vec![false; max_x + 1]; max_y + 1];
    for line in lines {
        if line.starts_with("fold along ") {
            let parts: Vec<&str> = line
                .split("fold along ")
                .filter(|p| !p.is_empty())
                .collect();
            let assignment = parts[0];
            let assignment_parts: Vec<&str> =
                assignment.split("=").filter(|p| !p.is_empty()).collect();
            folds.push(Fold {
                is_vertical: assignment_parts[0] == "y",
                line: assignment_parts[1].parse::<usize>().unwrap(),
            });
        }
        if !line.starts_with("fold along") && !line.is_empty() {
            let parts: Vec<usize> = line
                .split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect();
            paper[parts[1]][parts[0]] = true;
        }
    }
    (paper, folds)
}

fn count(paper: &Vec<Vec<bool>>) -> i32 {
    let mut count = 0;
    for y in 0..paper.len() {
        for x in 0..paper[y].len() {
            if paper[y][x] {
                count += 1;
            }
        }
    }
    count
}

fn fold(paper: &Vec<Vec<bool>>, fold: &Fold) -> Vec<Vec<bool>> {
    if fold.is_vertical {
        let mut new_paper = vec![vec![false; paper[0].len()]; fold.line];
        for y in 0..fold.line {
            for x in 0..paper[y].len() {
                new_paper[y][x] = paper[y][x];
            }
        }
        for y in fold.line + 1..paper.len() {
            let dist_from_fold = y - fold.line;
            for x in 0..paper[y].len() {
                if paper[y][x] {
                    new_paper[fold.line - dist_from_fold][x] = true;
                }
            }
        }
        return new_paper;
    } else {
        let mut new_paper = vec![vec![false; fold.line]; paper.len()];
        for y in 0..paper.len() {
            for x in 0..fold.line {
                new_paper[y][x] = paper[y][x];
            }
        }
        for y in 0..paper.len() {
            for x in fold.line + 1..paper[y].len() {
                let dist_from_fold = x - fold.line;
                if paper[y][x] {
                    new_paper[y][fold.line - dist_from_fold] = true;
                }
            }
        }
        return new_paper;
    }
}

fn p1(lines: &Vec<String>, debug: bool) {
    let (mut paper, folds) = parse(lines);
    if debug {
        print_paper(&paper, Some(&folds[0]));
        println!();
    }
    paper = fold(&paper, &folds[0]);
    if debug {
        print_paper(&paper, None);
        println!();
    }
    println!("Count: {}", count(&paper));
}

fn p2(lines: &Vec<String>, debug: bool) {
    let (mut paper, folds) = parse(lines);
    if debug {
        print_paper(&paper, Some(&folds[0]));
        println!();
    }
    for i in 0..folds.len() {
        if debug {
            let next_fold = if i + 1 < folds.len() {
                Some(&folds[i + 1])
            } else {
                None
            };
            print_paper(&paper, next_fold);
            println!();
        }
        paper = fold(&paper, &folds[i]);
    }
    println!("Final paper:");
    print_paper(&paper, None);
}

fn main() {
    let matches = App::new("day13")
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
        common::read_file("/Users/alexeyler/Development/aoc/2021/day13/src/data/paper.txt");
    match part {
        "1" => p1(&lines, debug),
        "2" => p2(&lines, debug),
        _ => eprintln!("Unknown part {}", part),
    }
}
