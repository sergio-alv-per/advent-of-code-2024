mod io_utils;

fn find_xmases(char_matrix: Vec<Vec<char>>) -> i32 {
    let valid_xmases = [
        ['M', 'M', 'A', 'S', 'S'],
        ['S', 'M', 'A', 'S', 'M'],
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'S', 'A', 'M', 'M'],
    ];

    let mut found = 0;
    for row_start in 0..char_matrix.len() - 2 {
        for column_start in 0..char_matrix[0].len() - 2 {
            let cross_letters: [char; 5] = [
                char_matrix[row_start][column_start],
                char_matrix[row_start][column_start + 2],
                char_matrix[row_start + 1][column_start + 1],
                char_matrix[row_start + 2][column_start],
                char_matrix[row_start + 2][column_start + 2],
            ];

            if valid_xmases.contains(&cross_letters) {
                found += 1
            }
        }
    }
    found
}

#[allow(unused_variables)]
fn solve(lines: Vec<String>) -> i32 {
    let char_matrix: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    find_xmases(char_matrix.clone())
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
        let result = solve(io_utils::read_file("inputs/4.in"));
        let solution = 1864;
        assert_eq!(result, solution);
    }
}
