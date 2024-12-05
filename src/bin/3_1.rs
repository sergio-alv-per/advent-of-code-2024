use regex::Regex;
mod io_utils;

fn parse_muls_in_line(line: &String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let captures = re.captures_iter(&line).map(|c| c.extract());
    let number_str_pairs = captures.map(|(_, [n1, n2])| (n1, n2));

    number_str_pairs
        .map(|(n1, n2)| (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()))
        .map(|(n1, n2)| n1 * n2)
        .sum()
}

fn solve(lines: Vec<String>) -> i32 {
    lines.iter().map(parse_muls_in_line).sum()
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
        let result = solve(io_utils::read_file("inputs/3.in"));
        let solution = 184511516;
        assert_eq!(result, solution);
    }
}
