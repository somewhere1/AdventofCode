use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(address: &str) -> Vec<String> {
    BufReader::new(File::open(address).expect("can not find the file"))
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}
