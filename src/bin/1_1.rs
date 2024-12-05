mod io_utils;

fn solve(lines: Vec<String>) -> i32 {
    let numbers_it = lines
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(n1, n2)| (n1.parse().unwrap(), n2.parse().unwrap()));

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for (n1, n2) in numbers_it {
        col1.push(n1);
        col2.push(n2);
    }

    col1.sort();
    col2.sort();

    col1.iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
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
        let solution = 1341714;
        assert_eq!(result, solution);
    }
}
