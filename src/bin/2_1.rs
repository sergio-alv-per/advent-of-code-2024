use std::io;

fn report_safe(report: &String) -> bool {
    let report_numbers: Vec<i32> = report
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let differences: Vec<i32> = report_numbers.windows(2).map(|s| s[1] - s[0]).collect();

    differences.iter().all(|&x| x > 0 && x <= 3) || differences.iter().all(|&x| x < 0 && x >= -3)
}

fn main() {
    let safe = io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(report_safe)
        .count();

    println!("{safe}");
}
