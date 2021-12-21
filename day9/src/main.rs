extern crate queues;
use clap::App;
use clap::Arg;
use colored::*;
use queues::*;

#[derive(Clone, Copy, Debug)]
struct Point {
    height: usize,
    low_point: bool,
    row: usize,
    col: usize,
    basin: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

static COLORS: [(u8, u8, u8); 9] = [
    (255, 0, 0),
    (0, 255, 0),
    (0, 0, 255),
    (255, 255, 0),
    (0, 255, 255),
    (255, 0, 255),
    (192, 192, 192),
    (128, 0, 0),
    (0, 128, 0),
];

fn get_dimensions(map: &Vec<Vec<Point>>) -> (usize, usize) {
    (map.len(), map[0].len())
}

fn get_risklevel(map: &Vec<Vec<Point>>) -> usize {
    let mut risklevel = 0;
    for row in map {
        for point in row {
            if point.low_point {
                risklevel += point.height + 1;
            }
        }
    }
    risklevel
}

fn print_heightmap(map: &Vec<Vec<Point>>) {
    for row in map {
        for point in row {
            print!(
                "{}",
                if point.low_point {
                    let true_color = COLORS[point.basin - 1];
                    point.height.to_string().on_white().truecolor(
                        true_color.0,
                        true_color.1,
                        true_color.2,
                    )
                } else if point.basin > 0 {
                    let true_color = COLORS[point.basin - 1];
                    point
                        .height
                        .to_string()
                        .truecolor(true_color.0, true_color.1, true_color.2)
                } else {
                    point.height.to_string().white()
                }
            )
        }
        println!();
    }
    println!("Risk level: {}", get_risklevel(map));
}

fn make_heightmap(lines: &Vec<String>) -> Vec<Vec<Point>> {
    let mut map: Vec<Vec<Point>> = Vec::new();
    for r in 0..lines.len() {
        let line = &lines[r];
        let mut row: Vec<Point> = Vec::new();
        for c in 0..line.len() {
            let height = (line.as_bytes()[c] as char).to_digit(10).unwrap() as usize;
            row.push(Point {
                height,
                low_point: false,
                row: r,
                col: c,
                basin: 0,
            });
        }
        map.push(row);
    }
    map
}

fn calc_low_points(heightmap: &mut Vec<Vec<Point>>) -> Vec<Point> {
    let (rows, cols) = get_dimensions(&heightmap);
    let mut low_points: Vec<Point> = Vec::new();
    let mut color_index: usize = 1;
    for r in 0..rows {
        for c in 0..cols {
            let point = &heightmap[r][c];

            if (r == 0 || point.height < heightmap[r - 1][c].height)
                && (r + 1 >= rows || point.height < heightmap[r + 1][c].height)
                && (c == 0 || point.height < heightmap[r][c - 1].height)
                && (c + 1 >= cols || point.height < heightmap[r][c + 1].height)
            {
                let mut_point = &mut heightmap[r][c];
                mut_point.low_point = true;
                mut_point.basin = color_index;
                color_index += 1;
                if color_index > 9 {
                    color_index = 1;
                }
                low_points.push(heightmap[r][c]);
            }
        }
    }
    low_points
}

fn calc_basin(height_map: &mut Vec<Vec<Point>>, low_point: &Point) -> i32 {
    let (rows, cols) = get_dimensions(height_map);
    let mut seen: Vec<Point> = Vec::new();
    let mut queue: Queue<Point> = queue![];
    queue.add(*low_point).unwrap();

    let mut basin_size = 0;
    while queue.size() > 0 {
        let current_point = queue.remove().unwrap();
        if !seen.contains(&current_point) {
            seen.push(current_point);
            if current_point.height < 9 {
                let mut_point = &mut height_map[current_point.row][current_point.col];
                mut_point.basin = low_point.basin;
                basin_size += 1;
                if current_point.row != 0
                    && height_map[current_point.row - 1][current_point.col].height != 9
                {
                    queue
                        .add(height_map[current_point.row - 1][current_point.col])
                        .unwrap();
                }
                if current_point.row < rows - 1
                    && height_map[current_point.row + 1][current_point.col].height != 9
                {
                    queue
                        .add(height_map[current_point.row + 1][current_point.col])
                        .unwrap();
                }
                if current_point.col != 0
                    && height_map[current_point.row][current_point.col - 1].height != 9
                {
                    queue
                        .add(height_map[current_point.row][current_point.col - 1])
                        .unwrap();
                }
                if current_point.col < cols - 1
                    && height_map[current_point.row][current_point.col + 1].height != 9
                {
                    queue
                        .add(height_map[current_point.row][current_point.col + 1])
                        .unwrap();
                }
            }
        }
    }
    basin_size
}

fn p1(lines: &Vec<String>) {
    let mut heightmap = make_heightmap(&lines);
    calc_low_points(&mut heightmap);
    print_heightmap(&heightmap);
}

fn p2(lines: &Vec<String>) {
    let mut heightmap = make_heightmap(&lines);
    let low_points = calc_low_points(&mut heightmap);
    let mut basin_sizes: Vec<i32> = Vec::new();
    for low_point in low_points {
        let basin_size = calc_basin(&mut heightmap, &low_point);
        basin_sizes.push(basin_size);
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    print_heightmap(&heightmap);
    println!(
        "{} * {} * {} = {}",
        basin_sizes[0],
        basin_sizes[1],
        basin_sizes[2],
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    )
}

fn main() {
    let matches = App::new("day9")
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
        common::read_file("/Users/alexeyler/Development/aoc/2021/day9/src/data/heightmap.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
