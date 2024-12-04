use regex::Regex;
use std::io;

fn parse_muls_in_line(line: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let captures = re.captures_iter(&line).map(|c| c.extract());
    let number_str_pairs = captures.map(|(_, [n1, n2])| (n1, n2));

    number_str_pairs
        .map(|(n1, n2)| (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap()))
        .map(|(n1, n2)| n1 * n2)
        .sum()
}

fn main() {
    let mul_sum: i32 = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(parse_muls_in_line)
        .sum();

    println!("{mul_sum}");
}
