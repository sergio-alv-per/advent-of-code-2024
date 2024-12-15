mod io_utils;

fn solve(lines: Vec<String>) -> u128 {
    let mut stones: Vec<u128> = lines[0].split(" ").map(|s| s.parse().unwrap()).collect();

    for _ in 0..25 {
        let mut updated_stones = Vec::new();

        for &stone in stones.iter() {
            if stone == 0 {
                updated_stones.push(1);
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let half_n_digits = (stone.ilog10() + 1) / 2;
                let left_stone = stone / 10u128.pow(half_n_digits);
                let right_stone = stone % 10u128.pow(half_n_digits);
                updated_stones.push(left_stone);
                updated_stones.push(right_stone);
            } else {
                updated_stones.push(stone * 2024);
            }
        }

        stones = updated_stones;
    }

    stones.len() as u128
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
        let solution = 186203;
        assert_eq!(result, solution);
    }
}
