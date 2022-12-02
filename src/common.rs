use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn map_lines_to_strings(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}


pub fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).expect("Missing file");
    let mut result_string = String::new();
    file.read_to_string(&mut result_string).expect("Failed to read into string");

    return result_string;
}
