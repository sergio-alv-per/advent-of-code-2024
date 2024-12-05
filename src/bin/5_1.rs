use std::collections::HashMap;
use std::io;

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

fn check_report(report: &String, rules: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    let report: Vec<i32> = report
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    for i in (0..report.len()).rev() {
        let current = report[i];
        let previous = &report[0..i];
        let not_allowed_before = rules.get(&current);
        match not_allowed_before {
            Some(not_allowed_before) => {
                for prev in previous {
                    if not_allowed_before.contains(prev) {
                        return None;
                    }
                }
            }
            None => {}
        }
    }

    let middle_page_index: usize = (report.len() - 1) / 2;
    Some(report[middle_page_index])
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let index_of_break = lines.iter().position(|s| s.is_empty()).unwrap();

    let rules_lines = &lines[0..index_of_break];
    let reports_lines = &lines[index_of_break + 1..];

    let rules = build_rules(rules_lines);

    let sum_good_reports: i32 = reports_lines
        .iter()
        .filter_map(|r| check_report(r, &rules))
        .inspect(|n| println!("{n}"))
        .sum();

    println!("{sum_good_reports}");
}
