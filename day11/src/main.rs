use clap::App;
use clap::Arg;
use colored::*;

fn print_board(board: &Vec<Vec<u32>>, flashed: &Vec<Vec<bool>>) {
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            print!(
                "{}",
                if flashed[r][c] {
                    board[r][c].to_string().yellow()
                } else {
                    board[r][c].to_string().white()
                }
            );
        }
        println!();
    }
}

fn get_board(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut board: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        board.push(row);
    }
    board
}

fn increase_energy(board: &mut Vec<Vec<u32>>) {
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            board[r][c] += 1;
        }
    }
}

fn reset_energy(board: &mut Vec<Vec<u32>>) {
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            if board[r][c] > 9 {
                board[r][c] = 0;
            }
        }
    }
}

fn safe_update(board: &mut Vec<Vec<u32>>, r: i32, c: i32) {
    if r >= 0 && c >= 0 {
        let r_usize = r as usize;
        let c_usize = c as usize;
        if r_usize < board.len() && c_usize < board[r_usize].len() {
            board[r_usize][c_usize] += 1;
        }
    }
}

fn flash(board: &mut Vec<Vec<u32>>, flashed: &mut Vec<Vec<bool>>) -> i32 {
    let mut new_flashes = false;
    let mut num_flashes = 0;
    loop {
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                if board[r][c] > 9 && !flashed[r][c] {
                    new_flashes = true;
                    num_flashes += 1;
                    flashed[r][c] = true;
                    let r_i32 = r as i32;
                    let c_i32 = c as i32;

                    // Update surroundings
                    safe_update(board, r_i32 - 1, c_i32 - 1);
                    safe_update(board, r_i32 - 1, c_i32);
                    safe_update(board, r_i32 - 1, c_i32 + 1);
                    safe_update(board, r_i32, c_i32 + 1);
                    safe_update(board, r_i32 + 1, c_i32 + 1);
                    safe_update(board, r_i32 + 1, c_i32);
                    safe_update(board, r_i32 + 1, c_i32 - 1);
                    safe_update(board, r_i32, c_i32 - 1);
                }
            }
        }
        if !new_flashes {
            return num_flashes;
        }
        new_flashes = false;
    }
}

fn run(lines: &Vec<String>, steps: i32, debug: bool) {
    let mut board = get_board(lines);
    let mut flashed: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
    let mut num_flashes = 0;
    if debug {
        println!("Before any steps:");
        print_board(&board, &flashed);
        println!();
    }
    for step in 1..steps + 1 {
        flashed = vec![vec![false; board[0].len()]; board.len()];
        increase_energy(&mut board);
        let step_flashes = flash(&mut board, &mut flashed);
        num_flashes += step_flashes;
        if step_flashes >= (board.len() * board[0].len()) as i32 {
            println!("Step {} had a total flash!", step);
        }
        reset_energy(&mut board);
        if debug {
            println!("After step {}", step);
            print_board(&board, &flashed);
            println!();
        }
    }

    println!("Total flashes: {}", num_flashes);
}

fn main() {
    let matches = App::new("day11")
        .version("1.0")
        .author("Alex Eyler <alex.eyler@outlook.com>")
        .about("Advent of code")
        .arg(
            Arg::with_name("steps")
                .short("s")
                .long("steps")
                .value_name("STEPS")
                .help("How many steps")
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
    let steps = matches.value_of("steps").unwrap().parse::<i32>().unwrap();
    let debug = matches.is_present("debug");
    let lines: Vec<String> =
        common::read_file("/Users/alexeyler/Development/aoc/2021/day11/src/data/octopuses.txt");
    run(&lines, steps, debug);
}
