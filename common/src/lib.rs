use std::fs;

pub fn read_file(fname: &str) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("Couldn't read.");
    return contents
        .split('\n')
        .map(|s| String::from(s.trim()))
        .collect();
}