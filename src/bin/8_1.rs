use std::collections::HashMap;

mod io_utils;

fn build_freq_positions_hm(lines: &Vec<String>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut freq_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in lines.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c != '.' {
                let res = freq_positions.get_mut(&c);

                match res {
                    None => {
                        freq_positions.insert(c, vec![(i, j)]);
                    }
                    Some(vs) => {
                        vs.push((i, j));
                    }
                }
            }
        }
    }

    freq_positions
}

fn build_antinodes(pos_a: &(usize, usize), pos_b: &(usize, usize)) -> ((i32, i32), (i32, i32)) {
    let delta_i = pos_b.0 as i32 - pos_a.0 as i32;
    let delta_j = pos_b.1 as i32 - pos_a.1 as i32;

    let an1 = (pos_a.0 as i32 - delta_i, pos_a.1 as i32 - delta_j);
    let an2 = (pos_b.0 as i32 + delta_i, pos_b.1 as i32 + delta_j);
    (an1, an2)
}

fn build_antinodes_for_frequency(positions: &Vec<(usize, usize)>) -> Vec<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();

    for (i, pos_a) in positions.iter().enumerate() {
        for pos_b in positions.iter().skip(i + 1) {
            let (an1, an2) = build_antinodes(pos_a, pos_b);
            antinodes.push(an1);
            antinodes.push(an2);
        }
    }

    antinodes
}

fn in_square(&tup: &(i32, i32), rows: usize, columns: usize) -> bool {
    let (i, j) = tup;
    let rows = rows as i32;
    let columns = columns as i32;
    i >= 0 && j >= 0 && i < rows && j < columns
}

fn to_usize(tup: &(i32, i32)) -> (usize, usize) {
    (tup.0 as usize, tup.1 as usize)
}

fn build_full_antinodes(
    freq_positions: HashMap<char, Vec<(usize, usize)>>,
    rows: usize,
    columns: usize,
) -> Vec<(usize, usize)> {
    let mut antinodes: Vec<(usize, usize)> = Vec::new();

    for (_, vs) in freq_positions.iter() {
        antinodes.extend(
            build_antinodes_for_frequency(vs)
                .iter()
                .filter(|&tup| in_square(tup, rows, columns))
                .map(to_usize),
        )
    }

    antinodes.sort();
    antinodes.dedup();
    antinodes
}

fn solve(lines: Vec<String>) -> i32 {
    let rows = lines.len();
    let columns = lines[0].len();

    let freq_positions = build_freq_positions_hm(&lines);

    let fan = build_full_antinodes(freq_positions, rows, columns);
    fan.len() as i32
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
        let result = solve(io_utils::read_file("inputs/8.in"));
        let solution = 271;
        assert_eq!(result, solution);
    }
}
