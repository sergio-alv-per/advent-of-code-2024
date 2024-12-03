use std::io;

fn report_safe(report: String) -> bool {
    let report_numbers: Vec<i32> = report
        .split(" ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let differences: Vec<i32> = report_numbers.windows(2).map(|s| s[1] - s[0]).collect();

    differences.iter().all(|&x| x > 0 && x <= 3) || differences.iter().all(|&x| x < 0 && x >= -3)
}

fn main() {
    let safe: i32 = io::stdin()
        .lines()
        .filter(|l| l.is_ok())
        .map(|x| x.unwrap())
        .map(report_safe)
        .map(|b| if b { 1 } else { 0 })
        .sum();

    println!("{safe}");
}
