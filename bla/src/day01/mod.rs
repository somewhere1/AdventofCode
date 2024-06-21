use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day01() {
    let file_path: &str = r"D:\Rust_practice\AdventofCode\bla\src\inputs\input.txt";
    let nums: Vec<i32> = BufReader::new(File::open(file_path).expect("Can not file a file"))
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("Day 01 - Part 1: {}", increasing(&nums, 1));

    print!("Day 01 - Part 2: {}", increasing(&nums, 3));
}
fn increasing(num: &[i32], offset: usize) -> usize {
    num.windows(offset + 1)
        .map(|i| (i[0] < i[offset]) as usize)
        .sum::<usize>() //fold(0, |a, b| a + b)
}
