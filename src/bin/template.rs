mod io_utils;

#[allow(unused_variables)]
fn solve(lines: Vec<String>) -> i32 {
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

    #[ignore] // REMOVE ON PROBLEM FILES
    #[test]
    fn solution_correct() {
        let result = solve(io_utils::read_file("inputs/input_file.in"));
        let solution = 0;
        assert_eq!(result, solution);
    }
}
