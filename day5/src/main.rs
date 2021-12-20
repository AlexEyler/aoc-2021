use clap::App;
use clap::Arg;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{},{}", self.x, self.y))
            .unwrap();
        Ok(())
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Segment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{} -> {}", self.p1, self.p2))
            .unwrap();
        Ok(())
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
    fn is_upper_diagonal(&self) -> bool {
        let x_diff = self.p1.x - self.p2.x;
        let y_diff = self.p1.y - self.p2.y;
        x_diff == -y_diff
    }
    fn is_lower_diagonal(&self) -> bool {
        let x_diff = self.p1.x - self.p2.x;
        let y_diff = self.p1.y - self.p2.y;
        x_diff == y_diff
    }
    fn get_left_right_points(&self) -> (Point, Point) {
        let left_point;
        let right_point;
        if self.p1.x < self.p2.x {
            left_point = self.p1;
            right_point = self.p2;
        } else {
            left_point = self.p2;
            right_point = self.p1;
        }
        (left_point, right_point)
    }
    fn points(&self, enable_diagonals: bool) -> Vec<Point> {
        if self.is_vertical() {
            return (cmp::min(self.p1.y, self.p2.y)..cmp::max(self.p1.y, self.p2.y) + 1)
                .into_iter()
                .map(|y| Point { x: self.p1.x, y: y })
                .collect();
        }
        if self.is_horizontal() {
            return (cmp::min(self.p1.x, self.p2.x)..cmp::max(self.p1.x, self.p2.x) + 1)
                .into_iter()
                .map(|x| Point { x: x, y: self.p1.y })
                .collect();
        }
        if enable_diagonals {
            if self.is_lower_diagonal() {
                let (left_point, right_point) = self.get_left_right_points();
                let mut vec: Vec<Point> = Vec::new();
                let mut y = left_point.y;
                for x in left_point.x..right_point.x + 1 {
                    vec.push(Point { x, y });
                    y += 1;
                }
                return vec;
            }
            if self.is_upper_diagonal() {
                let (left_point, right_point) = self.get_left_right_points();
                let mut vec: Vec<Point> = Vec::new();
                let mut y = left_point.y;
                for x in left_point.x..right_point.x + 1 {
                    vec.push(Point { x, y });
                    y -= 1;
                }
                return vec;
            }
        }

        return Vec::new();
    }
}

fn get_segments(lines: &Vec<String>) -> Vec<Segment> {
    let mut points: Vec<Point> = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    for line in lines {
        points.extend(
            line.split("->")
                .filter(|&l| !l.is_empty())
                .map(|l| l.trim())
                .map(|l| {
                    let parts: Vec<&str> = l.split(",").map(|l| l.trim()).collect();
                    return Point {
                        x: parts[0].parse::<i32>().unwrap(),
                        y: parts[1].parse::<i32>().unwrap(),
                    };
                }),
        );
    }
    for p in 0..points.len() {
        if p % 2 == 0 {
            segments.push(Segment {
                p1: points[p],
                p2: points[p + 1],
            });
        }
    }
    segments
}

fn get_max(segments: &Vec<Segment>) -> (i32, i32) {
    let mut max_x = -1;
    let mut max_y = -1;
    for segment in segments {
        if segment.p1.x > max_x {
            max_x = segment.p1.x;
        }
        if segment.p2.x > max_x {
            max_x = segment.p2.x;
        }
        if segment.p1.y > max_y {
            max_y = segment.p1.y;
        }
        if segment.p2.y > max_y {
            max_y = segment.p2.y;
        }
    }

    (max_x, max_y)
}

fn print_board(segments: &Vec<Segment>) {
    let (max_x, max_y) = get_max(&segments);
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    println!("(0, 0) -> ({},{})", max_x, max_y);
    for r in 0..max_x + 1 {
        println!("x: {}", r);
        for c in 0..max_y + 1 {
            let point = Point { x: r, y: c };
            for i in 0..segments.len() {
                let entry = point_map.entry(point).or_insert(0);
                let segment = &segments[i];
                let segment_points = segment.points(true);
                if segment_points.contains(&point) {
                    *entry += 1
                }
            }
            print!(
                "{}",
                if point_map[&point] > 0 {
                    point_map[&point].to_string()
                } else {
                    ".".to_string()
                }
            );
        }
        println!();
    }

    let mut max_intersections = 0;
    let mut num_max_intersections = 0;
    for point in point_map.keys() {
        if point_map[point] >= max_intersections {
            if point_map[point] > max_intersections {
                max_intersections = point_map[point];
                num_max_intersections = 0;
            }
            num_max_intersections += 1;
        }
    }

    println!("{}", num_max_intersections);
}

fn p1(lines: &Vec<String>) {
    let segments = get_segments(lines);
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    for segment in segments {
        for point in segment.points(false) {
            let entry = point_map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    let mut num_max_intersections = 0;
    for point in point_map.keys() {
        if point_map[point] >= 2 {
            num_max_intersections += 1;
        }
    }
    println!("{}", num_max_intersections);
}

fn p2(lines: &Vec<String>) {
    let segments = get_segments(lines);
    let mut point_map: HashMap<Point, i32> = HashMap::new();
    for segment in segments {
        for point in segment.points(true) {
            let entry = point_map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    let mut num_max_intersections = 0;
    for point in point_map.keys() {
        if point_map[point] >= 2 {
            num_max_intersections += 1;
        }
    }
    println!("{}", num_max_intersections);
}

fn main() {
    let matches = App::new("day5")
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
        common::read_file("/Users/alexeyler/Development/aoc/2021/day5/src/data/vents.txt");
    match part {
        "1" => p1(&lines),
        "2" => p2(&lines),
        _ => eprintln!("Bad arg value: {}", part),
    }
}
