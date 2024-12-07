mod io_utils;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Sum,
    Product,
    Concatenate,
}

fn concat(a: &u128, b: &u128) -> u128 {
    a * 10u64.pow(b.ilog10() + 1) as u128 + b
}

fn equation_solvable(&objective: &u128, numbers: &Vec<u128>) -> bool {
    let mut stack = vec![
        vec![Operator::Sum],
        vec![Operator::Product],
        vec![Operator::Concatenate],
    ];

    while !stack.is_empty() {
        let node_operations = stack.pop().unwrap();
        let initial = numbers[0];
        let partial_result = numbers.iter().skip(1).zip(node_operations.iter()).fold(
            initial,
            |acc, (curr, oper)| match oper {
                Operator::Sum => acc + curr,
                Operator::Product => acc * curr,
                Operator::Concatenate => concat(&acc, curr),
            },
        );

        if node_operations.len() < numbers.len() - 1 {
            if partial_result <= objective {
                let mut next_node_sum = node_operations.clone();
                let mut next_node_prod = node_operations.clone();
                let mut next_node_conc = node_operations.clone();

                next_node_sum.push(Operator::Sum);
                next_node_prod.push(Operator::Product);
                next_node_conc.push(Operator::Concatenate);

                stack.push(next_node_sum);
                stack.push(next_node_prod);
                stack.push(next_node_conc);
            }
        } else {
            if partial_result == objective {
                return true;
            }
        }
    }

    false
}

fn line_to_objective_and_equation(line: &String) -> (u128, Vec<u128>) {
    let (objective, numbers) = line.split_once(": ").unwrap();
    let objective: u128 = objective.parse().unwrap();
    let numbers: Vec<u128> = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
    (objective, numbers)
}

fn solve(lines: Vec<String>) -> u128 {
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
        let solution = 145397611075341;
        assert_eq!(result, solution);
    }
}
