use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use regex::Regex;

mod io_utils;

#[derive(Debug)]
struct ClawProblem {
    dxa: i32,
    dxb: i32,
    dya: i32,
    dyb: i32,
    x: i32,
    y: i32,
}

fn captures_1_2(haystack: &String, pattern: &Regex) -> (i32, i32) {
    let captures = pattern.captures(haystack).unwrap();

    (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    )
}

fn lines_to_claw_problems(lines: &Vec<String>) -> Vec<ClawProblem> {
    let find_two_numbers = Regex::new(r"[^\d\n]+(\d+)[^\d]+(\d+)$").unwrap();
    let mut problems: Vec<ClawProblem> = Vec::new();

    for line_group in lines.chunks(4) {
        let [line_a, line_b, line_xy, _] = line_group else {
            panic!("Malformed line group: {line_group:?}")
        };

        let (dxa, dya) = captures_1_2(line_a, &find_two_numbers);
        let (dxb, dyb) = captures_1_2(line_b, &find_two_numbers);
        let (x, y) = captures_1_2(line_xy, &find_two_numbers);

        problems.push(ClawProblem {
            dxa,
            dxb,
            dya,
            dyb,
            x,
            y,
        });
    }

    problems
}

fn solve_problem(problem: &ClawProblem) -> Option<i32> {
    variables! {
        vars:
            0 <= a (integer) <= 100;
            0 <= b (integer) <= 100;
    }

    let solution = vars
        .minimise(3 * a + b)
        .using(default_solver)
        .with(constraint!(a * problem.dxa + b * problem.dxb == problem.x))
        .with(constraint!(a * problem.dya + b * problem.dyb == problem.y))
        .solve()
        .ok()?;

    Some(solution.eval(3 * a + b).round() as i32)
}

fn solve(lines: Vec<String>) -> i32 {
    let problems = lines_to_claw_problems(&lines);

    problems.iter().filter_map(solve_problem).sum()
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
        let result = solve(io_utils::read_file("inputs/13.in"));
        let solution = 29023;
        assert_eq!(result, solution);
    }
}
