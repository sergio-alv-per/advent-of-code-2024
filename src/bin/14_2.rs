use std::collections::HashMap;

use regex::Regex;

mod io_utils;

struct Robot {
    pi: i32,
    pj: i32,
    di: i32,
    dj: i32,
}

fn position_at_second(robot: &Robot, second: i32) -> (i32, i32) {
    let rows = 103;
    let columns = 101;

    let i = (robot.pi + second * robot.di).rem_euclid(rows);
    let j = (robot.pj + second * robot.dj).rem_euclid(columns);
    (i, j)
}

fn line_to_robot(line: &String, pattern: &Regex) -> Robot {
    let captures = pattern.captures(line).unwrap();

    Robot {
        pj: captures.get(1).unwrap().as_str().parse().unwrap(),
        pi: captures.get(2).unwrap().as_str().parse().unwrap(),
        dj: captures.get(3).unwrap().as_str().parse().unwrap(),
        di: captures.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn print_positions(positions: &Vec<(i32, i32)>) {
    let rows = 103;
    let columns = 101;

    let mut frequencies: HashMap<(i32, i32), i32> = HashMap::new();
    for &p in positions {
        *frequencies.entry(p).or_insert(0) += 1;
    }

    for i in 0..rows {
        for j in 0..columns {
            let representation = match frequencies.get(&(i, j)) {
                Some(k @ 1..10) => &k.to_string(),
                Some(10..) => "X",
                _ => ".",
            };

            print!("{representation}");
        }
        println!("");
    }
}

fn all_unique(vec: &Vec<(i32, i32)>) -> bool {
    let mut frequencies: HashMap<(i32, i32), i32> = HashMap::new();
    for &p in vec {
        *frequencies.entry(p).or_insert(0) += 1;
    }

    vec.len() == frequencies.len()
}

fn solve(lines: Vec<String>) -> i32 {
    let pattern = Regex::new(r"^p=(-?\d+),(-?\d+) v=?(-?\d+),(-?\d+)$").unwrap();
    let robots: Vec<Robot> = lines.iter().map(|l| line_to_robot(l, &pattern)).collect();

    for i in 1.. {
        let positions: Vec<(i32, i32)> = robots.iter().map(|r| position_at_second(r, i)).collect();

        if all_unique(&positions) {
            // print_positions(&positions);
            return i;
        }
    }
    0
}

fn main() {
    let lines = io_utils::read_stdin();
    let solution = solve(lines);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_correct() {
        let result = solve(io_utils::read_file("inputs/14.in"));
        let solution = 6285;
        assert_eq!(result, solution);
    }
}
