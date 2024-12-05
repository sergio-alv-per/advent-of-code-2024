use std::io;

fn find_horizontal(char_matrix: Vec<Vec<char>>) -> i32 {
    let mut found = 0;
    for row in char_matrix {
        for start in 0..row.len() - 3 {
            if row[start] == 'X'
                && row[start + 1] == 'M'
                && row[start + 2] == 'A'
                && row[start + 3] == 'S'
            {
                found += 1
            }
        }

        for start in 3..row.len() {
            if row[start] == 'X'
                && row[start - 1] == 'M'
                && row[start - 2] == 'A'
                && row[start - 3] == 'S'
            {
                found += 1
            }
        }
    }

    found
}

fn transpose<T>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = mat[0].len();
    let mut iters: Vec<_> = mat.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_vertical(char_matrix: Vec<Vec<char>>) -> i32 {
    find_horizontal(transpose(char_matrix))
}

fn find_diagonal_2nd_quadrant(char_matrix: Vec<Vec<char>>) -> i32 {
    let mut found = 0;
    for row_start in 0..char_matrix.len() - 3 {
        for column_start in 0..char_matrix[0].len() - 3 {
            if char_matrix[row_start][column_start] == 'X'
                && char_matrix[row_start + 1][column_start + 1] == 'M'
                && char_matrix[row_start + 2][column_start + 2] == 'A'
                && char_matrix[row_start + 3][column_start + 3] == 'S'
            {
                found += 1
            }
        }
    }

    for row_start in 3..char_matrix.len() {
        for column_start in 3..char_matrix[0].len() {
            if char_matrix[row_start][column_start] == 'X'
                && char_matrix[row_start - 1][column_start - 1] == 'M'
                && char_matrix[row_start - 2][column_start - 2] == 'A'
                && char_matrix[row_start - 3][column_start - 3] == 'S'
            {
                found += 1
            }
        }
    }

    found
}

fn reverse_rows(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    mat.iter()
        .map(|v| {
            let mut v = v.clone();
            v.reverse();
            v
        })
        .collect()
}

fn find_diagonal_1st_quadrant(char_matrix: Vec<Vec<char>>) -> i32 {
    find_diagonal_2nd_quadrant(reverse_rows(char_matrix))
}

fn main() {
    let char_matrix: Vec<Vec<char>> = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|s| s.chars().collect())
        .collect();

    let xmases = find_horizontal(char_matrix.clone())
        + find_vertical(char_matrix.clone())
        + find_diagonal_1st_quadrant(char_matrix.clone())
        + find_diagonal_2nd_quadrant(char_matrix.clone());

    println!("{xmases}");
}
