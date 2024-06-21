use std::{fs::File, io::{BufRead, BufReader}};
pub fn day02() {
    let file = File::open(r"D:\Rust_practice\AdventofCode\bla\src\inputs\input.txt").expect("can not file the file");
    let lines = BufReader::new(file)
    .lines()
    .map(|x| x.unwrap())
    .collect::<Vec<String>>();
    println!("part 01: {}",part1(&lines));

    println!("Day 02 - part 1: ");
    println!("Day 02 - part 2: ");
}
fn part1(command: &[String]) -> i32 {
    let result: (i32, i32) = command
        .iter()
        .map(|s| s.split_whitespace())
        .map(|mut cmd| match cmd.next().unwrap() {
            "forward" => (0, cmd.next().unwrap().parse().unwrap()),
            "up" => (-(cmd.next().unwrap().parse::<i32>().unwrap()), 0),
            "down" => ((cmd.next().unwrap().parse().unwrap()), 0),
            x => panic!("get a strange command:{}", x),
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    result.0 * result.1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let command = vec![
            "forward 5\n".to_string(),
            "down 5\n".to_string(),
            "forward 8\n".to_string(),
            "up 3\n".to_string(),
            "down 8\n".to_string(),
            "forward 2\n".to_string(),
        ];
        assert_eq!(part1(&command), 150);
    }
}