mod io_utils;

fn report_safe(report: &String) -> bool {
    let report_numbers: Vec<i32> = report
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let differences: Vec<i32> = report_numbers.windows(2).map(|s| s[1] - s[0]).collect();

    differences.iter().all(|&x| x > 0 && x <= 3) || differences.iter().all(|&x| x < 0 && x >= -3)
}

fn solve(lines: Vec<String>) -> i32 {
    lines.iter().filter(|&s| report_safe(s)).count() as i32
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
        let result = solve(io_utils::read_file("inputs/2.in"));
        let solution = 402;
        assert_eq!(result, solution);
    }
}
