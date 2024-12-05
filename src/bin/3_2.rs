use regex::Regex;
mod io_utils;

enum Capture {
    Do,
    Dont,
    Mul { n1: i32, n2: i32 },
}

fn parse_enabled_muls(full_input: String) -> i32 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    let captures = re
        .captures_iter(&full_input)
        .map(|c| match c.get(0).unwrap().as_str() {
            "do()" => Capture::Do,
            "don't()" => Capture::Dont,
            _ => Capture::Mul {
                n1: c.get(2).unwrap().as_str().parse().unwrap(),
                n2: c.get(3).unwrap().as_str().parse().unwrap(),
            },
        });

    let mut enabled = true;
    let mut result: i32 = 0;
    for c in captures {
        match c {
            Capture::Do => enabled = true,
            Capture::Dont => enabled = false,
            Capture::Mul { n1, n2 } => {
                if enabled {
                    result += n1 * n2
                }
            }
        }
    }

    result
}

fn solve(lines: Vec<String>) -> i32 {
    parse_enabled_muls(lines.concat())
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
        let solution = 90044227;
        assert_eq!(result, solution);
    }
}
