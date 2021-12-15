extern crate clap;
extern crate colored;
use clap::{App, Arg};
use colored::*;
use common;
use std::io::stdin;

#[derive(Clone)]
struct BoardPosition<'a> {
    number: &'a str,
    checked: bool,
}

fn update_boards(number: &str, boards: &mut Vec<Vec<Vec<BoardPosition>>>) {
    for board in boards {
        for row in board {
            for pos in row {
                if pos.number == number {
                    pos.checked = true;
                }
            }
        }
    }
}

fn calc_unchecked_sum(board: &Vec<Vec<BoardPosition>>) -> i32 {
    let mut unchecked_sum: i32 = 0;
    for row in board {
        for pos in row {
            unchecked_sum += if pos.checked {
                0
            } else {
                pos.number.parse::<i32>().unwrap()
            };
        }
    }

    unchecked_sum
}

fn is_win(board: &Vec<Vec<BoardPosition>>) -> bool {
    for i in 0..5 {
        let mut row_win = true;
        let mut col_win = true;
        for j in 0..5 {
            row_win &= board[i][j].checked;
            col_win &= board[j][i].checked;
        }

        if row_win || col_win {
            return true;
        }
    }

    false
}

fn find_winner(board: &Vec<Vec<BoardPosition>>) -> i32 {
    if is_win(board) {
        return calc_unchecked_sum(board);
    }

    return -1;
}

fn print_board(board: &Vec<Vec<BoardPosition>>) {
    for row in board {
        for pos in row {
            print!(
                "{}\t",
                if pos.checked {
                    pos.number.green()
                } else {
                    pos.number.black()
                }
            );
        }
        println!();
    }
    println!();
}

fn get_input(lines: &Vec<String>) -> (Vec<&str>, Vec<Vec<Vec<BoardPosition>>>) {
    let number_draw: &String = &lines[0];
    let numbers_drawn: Vec<&str> = number_draw.split(',').collect();

    let mut boards: Vec<Vec<Vec<BoardPosition>>> = Vec::new();
    let mut i = 2;
    while i < lines.len() {
        if i + 5 >= lines.len() {
            panic!("Out of bounds!");
        }

        let mut current_board = vec![
            vec![
                BoardPosition {
                    number: "0",
                    checked: false,
                };
                5
            ];
            5
        ];
        let mut row_index = 0;
        for j in i..i + 5 {
            let line = &lines[j];
            let row: Vec<&str> = line.split(' ').filter(|&n| !n.is_empty()).collect();
            for num in 0..5 {
                current_board[row_index][num] = BoardPosition {
                    number: row[num],
                    checked: false,
                };
            }
            row_index += 1;
        }

        boards.push(current_board);
        i += 6;
    }

    return (numbers_drawn, boards);
}

fn p1(lines: &Vec<String>) {
    let (numbers_drawn, mut boards) = get_input(lines);
    for number in numbers_drawn {
        update_boards(number, &mut boards);
        for i in 0..boards.len() {
            let board = &boards[i];
            let unchecked_sum = find_winner(&board);
            if unchecked_sum > -1 {
                println!("Winning board: {}", i);
                print_board(&board);
                println!(
                    "{} • {} = {}",
                    unchecked_sum,
                    number,
                    unchecked_sum * number.parse::<i32>().unwrap()
                );
                return;
            }
        }
    }

    println!("No winners!");
}

fn p2(lines: &Vec<String>) {
    let (numbers_drawn, mut boards) = get_input(lines);
    for number in numbers_drawn {
        update_boards(number, &mut boards);
        if boards.len() > 1 {
            boards.retain(|board| find_winner(board) < 0);
        } else {
            let unchecked_sum = find_winner(&boards[0]);
            if unchecked_sum > -1 {
                println!("Final board");
                print_board(&boards[0]);
                println!(
                    "{} * {} = {}",
                    unchecked_sum,
                    number,
                    unchecked_sum * number.parse::<i32>().unwrap()
                );
                return;
            }
        }
    }
}

fn main() {
    let matches = App::new("day4")
        .version("1.0")
        .author("Alex Eyler <alex.eyler@outlook.com>")
        .about("Advent of code")
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .help("Which part"),
        )
        .get_matches();

    let part = matches
        .value_of("part")
        .expect("Part (-p, --part) required.");

    let lines: Vec<String> =
        common::read_file("/Users/alexeyler/Development/aoc/2021/day4/src/data/cards.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
