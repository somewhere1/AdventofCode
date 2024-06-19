
use std::{fs::File, io::{BufRead, BufReader}};

pub fn day01 (){
    let file_path : &str = r"D:\Rust_practice\AdventofCode\bla\src\inputs\input.txt";
    let nums: Vec<i32>=  BufReader::new(File::open(file_path).expect("Can not file a file")).lines().map( |line| line.unwrap().parse().unwrap()).collect();

    
    let mut count = 0;
    for i in nums.windows(2){
        if i[0]<i[1]{
             count = count +  1;
        }
    }
    print!("Day 01 - Part 1: {}\n",count);

    count = 0;
    for i in nums.windows(4){
       
        if i[0] < i[3]{
            count = count +1 ;
        }
    }
    print!("Day 01 - Part 2: {}",count);
}