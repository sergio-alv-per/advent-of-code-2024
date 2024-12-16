use std::collections::HashMap;

use regex::Regex;

mod io_utils;

struct Robot {
    pi: i32,
    pj: i32,
    di: i32,
    dj: i32,
}

fn quadrant((i, j): (i32, i32)) -> Option<i32> {
    // Return the cuadrant for a position
    match (i, j) {
        (0..=50, 0..=49) => Some(1),
        (0..=50, 51..101) => Some(2),
        (52..103, 0..=49) => Some(3),
        (52..103, 51..101) => Some(4),
        _ => None,
    }
}

fn final_position(robot: Robot) -> (i32, i32) {
    let seconds = 100;
    let rows = 103;
    let columns = 101;

    let i = (robot.pi + seconds * robot.di).rem_euclid(rows);
    let j = (robot.pj + seconds * robot.dj).rem_euclid(columns);
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

fn solve(lines: Vec<String>) -> i32 {
    let pattern = Regex::new(r"^p=(-?\d+),(-?\d+) v=?(-?\d+),(-?\d+)$").unwrap();
    let robots = lines.iter().map(|l| line_to_robot(l, &pattern));
    let quadrants = robots.map(final_position).filter_map(quadrant);

    let frequencies = quadrants.fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    println!("{frequencies:?}");
    frequencies.values().product()
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
        let solution = 215476074;
        assert_eq!(result, solution);
    }
}
