use regex::Regex;

mod io_utils;

#[derive(Debug)]
struct ClawProblem {
    dxa: i128,
    dxb: i128,
    dya: i128,
    dyb: i128,
    x: i128,
    y: i128,
}

fn captures_1_2(haystack: &String, pattern: &Regex) -> (i128, i128) {
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

fn determinant(matrix: &[[i128; 2]; 2]) -> i128 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn inverse(matrix: &[[i128; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let det = determinant(matrix);
    if det == 0 {
        None
    } else {
        let det = det as f64;
        Some([
            [matrix[1][1] as f64 / det, -matrix[0][1] as f64 / det],
            [-matrix[1][0] as f64 / det, matrix[0][0] as f64 / det],
        ])
    }
}

fn solve_problem(problem: &ClawProblem) -> Option<i128> {
    let problem_matrix = [[problem.dxa, problem.dxb], [problem.dya, problem.dyb]];
    let true_x = 10000000000000 + problem.x;
    let true_y = 10000000000000 + problem.y;

    match inverse(&problem_matrix) {
        None => {
            // Colinear vectors, one of the dimensions is irrelevant
            // They do not appear in my input
            print!("Colinear!");
            None
        }
        Some(inverse_matrix) => {
            // Non colinear
            let a = inverse_matrix[0][0] * (true_x as f64) + inverse_matrix[0][1] * (true_y as f64);
            let b = inverse_matrix[1][0] * (true_x as f64) + inverse_matrix[1][1] * (true_y as f64);

            if a.round() as i128 * problem.dxa + b.round() as i128 * problem.dxb == true_x
                && a.round() as i128 * problem.dya + b.round() as i128 * problem.dyb == true_y
            {
                Some(3 * a.round() as i128 + b.round() as i128)
            } else {
                None
            }
        }
    }
}

fn solve(lines: Vec<String>) -> i128 {
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
        let solution = 96787395375634;
        assert_eq!(result, solution);
    }
}
