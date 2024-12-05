use std::collections::HashMap;
mod io_utils;

fn build_rules(rules_lines: &[String]) -> HashMap<i32, Vec<i32>> {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for l in rules_lines {
        let (n1, n2) = l.split_once('|').unwrap();
        let n1: i32 = n1.parse().unwrap();
        let n2: i32 = n2.parse().unwrap();

        if rules.contains_key(&n1) {
            rules.get_mut(&n1).unwrap().push(n2);
        } else {
            rules.insert(n1, vec![n2]);
        }
    }

    rules
}

fn check_elem_in_report(
    current_i: usize,
    report: &Vec<i32>,
    rules: &HashMap<i32, Vec<i32>>,
) -> Result<(), usize> {
    let current_elem = report[current_i];
    let not_allowed_before = rules.get(&current_elem);

    match not_allowed_before {
        None => Ok(()),
        Some(not_allowed_before) => {
            for prev_j in (0..current_i).rev() {
                let prev_elem = report[prev_j];
                if not_allowed_before.contains(&prev_elem) {
                    return Err(prev_j);
                }
            }
            return Ok(());
        }
    }
}

fn check_report(report: &String, rules: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    let mut report: Vec<i32> = report
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut current_i = report.len() - 1;

    let mut some_swap = false;

    while current_i > 0 {
        let res = check_elem_in_report(current_i, &report, rules);

        match res {
            Ok(_) => {
                current_i -= 1;
            }
            Err(j) => {
                some_swap = true;
                report.swap(current_i, j);
            }
        }
    }

    if some_swap {
        let middle_page_index: usize = (report.len() - 1) / 2;
        Some(report[middle_page_index])
    } else {
        None
    }
}

fn solve(lines: Vec<String>) -> i32 {
    let index_of_break = lines.iter().position(|s| s.is_empty()).unwrap();

    let rules_lines = &lines[0..index_of_break];
    let reports_lines = &lines[index_of_break + 1..];

    let rules = build_rules(rules_lines);

    reports_lines
        .iter()
        .filter_map(|r| check_report(r, &rules))
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
        let result = solve(io_utils::read_file("inputs/5.in"));
        let solution = 6311;
        assert_eq!(result, solution);
    }
}
