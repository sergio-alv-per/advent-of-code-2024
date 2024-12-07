mod io_utils;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Sum,
    Product,
}

fn equation_solvable(&objective: &i64, numbers: &Vec<i64>) -> bool {
    let mut stack = vec![vec![Operator::Sum], vec![Operator::Product]];

    while !stack.is_empty() {
        let node_operations = stack.pop().unwrap();
        let initial = numbers[0];
        let partial_result = numbers.iter().skip(1).zip(node_operations.iter()).fold(
            initial,
            |acc, (curr, oper)| match oper {
                Operator::Sum => acc + curr,
                Operator::Product => acc * curr,
            },
        );

        if node_operations.len() < numbers.len() - 1 {
            if partial_result <= objective {
                let mut next_node_sum = node_operations.clone();
                let mut next_node_prod = node_operations.clone();

                next_node_sum.push(Operator::Sum);
                next_node_prod.push(Operator::Product);

                stack.push(next_node_sum);
                stack.push(next_node_prod);
            }
        } else {
            // println!("{numbers:?} | {node_operations:?}, {partial_result}, {objective}");
            if partial_result == objective {
                return true;
            }
        }
    }

    false
}

fn line_to_objective_and_equation(line: &String) -> (i64, Vec<i64>) {
    let (objective, numbers) = line.split_once(": ").unwrap();
    let objective: i64 = objective.parse().unwrap();
    let numbers: Vec<i64> = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
    (objective, numbers)
}

fn solve(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(line_to_objective_and_equation)
        .filter(|(o, nums)| equation_solvable(o, nums))
        .map(|(o, _)| o)
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
        let result = solve(io_utils::read_file("inputs/7.in"));
        let solution = 1620690235709;
        assert_eq!(result, solution);
    }
}
