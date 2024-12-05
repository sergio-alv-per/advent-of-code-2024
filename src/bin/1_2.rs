mod io_utils;
use std::collections::HashMap;

fn solve(lines: Vec<String>) -> i32 {
    let numbers_it = lines
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()));

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for (n1, n2) in numbers_it {
        col1.push(n1);
        let v = col2.get(&n2);

        match v {
            Some(freq) => col2.insert(n2, freq + 1),
            None => col2.insert(n2, 1),
        };
    }

    col1.iter()
        .map(|n1| (n1, col2.get(&n1)))
        .filter(|(_, freq)| freq.is_some())
        .map(|(&n1, freq)| n1 * freq.unwrap())
        .sum()
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
        let result = solve(io_utils::read_file("inputs/1.in"));
        let solution = 27384707;
        assert_eq!(result, solution);
    }
}
