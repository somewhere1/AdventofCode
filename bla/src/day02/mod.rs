use crate::utils::read_file;
pub fn day02() {
    let file = r"D:\Rust_practice\AdventofCode\bla\src\inputs\input.txt";
    let lines = read_file(file);
    println!("Day 02 - part 1: ");
    println!("part 01: {}", part1(&lines));

    println!("Day 02 - part 2: ");
    println!("part 02: {}", part2(&lines));
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
fn part2(command: &[String]) -> i32 {
    //aim hpos,depth
    let mut aim = 0;
    let result = command
        .iter()
        .map(|x| x.split_whitespace())
        .map(|mut cmd| match cmd.next().unwrap() {
            "forward" => {
                let a: i32 = cmd.next().unwrap().parse().unwrap();
                (aim * a, a)
            }
            "up" => {
                let a: i32 = cmd.next().unwrap().parse::<i32>().unwrap();
                aim -= a;
                (0, 0)
            }
            "down" => {
                let a: i32 = cmd.next().unwrap().parse::<i32>().unwrap();
                aim += a;
                (0, 0)
            }
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
