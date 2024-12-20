use std::collections::HashMap;

mod io_utils;

fn even_n_digits(n: usize) -> bool {
    (n.ilog10() + 1) % 2 == 0
}

fn split_number(n: usize) -> (usize, usize) {
    let half_n_digits = (n.ilog10() + 1) / 2;
    let left_part = n / 10usize.pow(half_n_digits);
    let right_part = n % 10usize.pow(half_n_digits);
    (left_part, right_part)
}

fn recursive_dp(stone: usize, blinks: i32, table: &mut HashMap<(usize, i32), u128>) -> u128 {
    if let Some(&total) = table.get(&(stone, blinks)) {
        return total;
    }

    let total = if blinks == 0 {
        1u128
    } else if stone == 0 {
        recursive_dp(1, blinks - 1, table)
    } else if even_n_digits(stone) {
        let (l, r) = split_number(stone);
        recursive_dp(l, blinks - 1, table) + recursive_dp(r, blinks - 1, table)
    } else {
        recursive_dp(stone * 2024, blinks - 1, table)
    };

    table.insert((stone, blinks), total);
    total
}

fn solve(lines: Vec<String>) -> u128 {
    let mut table: HashMap<(usize, i32), u128> = HashMap::new();

    lines[0]
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .map(|stone| recursive_dp(stone, 75, &mut table))
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
        let result = solve(io_utils::read_file("inputs/11.in"));
        let solution = 221291560078593;
        assert_eq!(result, solution);
    }
}
