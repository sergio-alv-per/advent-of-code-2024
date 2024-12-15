use std::collections::BTreeSet;
use std::collections::HashMap;

mod io_utils;

fn get_adjacent(i: usize, j: usize, di: i32, dj: i32, tiles: &Vec<Vec<char>>) -> Option<char> {
    if let Some(idi) = i.checked_add_signed(di as isize) {
        if let Some(jdj) = j.checked_add_signed(dj as isize) {
            let row = tiles.get(idi)?;
            let &c = row.get(jdj)?;
            return Some(c);
        }
    }
    None
}

fn generate_graph(tiles: &Vec<Vec<char>>) -> HashMap<(usize, usize), [bool; 4]> {
    let rows = tiles.len();
    let columns = tiles[0].len();

    let mut adj_tiles: HashMap<(usize, usize), [bool; 4]> = HashMap::new();

    for i in 0..rows {
        for j in 0..columns {
            let mut adj_tile = [false, false, false, false];
            for (k, &(di, dj)) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().enumerate() {
                if let Some(c) = get_adjacent(i, j, di, dj, &tiles) {
                    let current_char = tiles[i][j];
                    if c == current_char {
                        adj_tile[k] = true;
                    }
                }
            }
            adj_tiles.insert((i, j), adj_tile);
        }
    }

    adj_tiles
}

fn solve(lines: Vec<String>) -> i32 {
    let tiles: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let adj_graph = generate_graph(&tiles);

    //let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut not_visited: BTreeSet<(usize, usize)> = BTreeSet::from_iter(adj_graph.keys().copied());

    let mut total = 0;

    while let Some((init_i, init_j)) = not_visited.first().copied() {
        let mut stack = vec![(init_i, init_j)];
        let mut area = 0;
        let mut fences = 0;
        while let Some((i, j)) = stack.pop() {
            if !not_visited.contains(&(i, j)) {
                continue;
            }

            area += 1;
            fences += adj_graph[&(i, j)].iter().filter(|&&adj| !adj).count();
            not_visited.remove(&(i, j));
            // Componentes conexas
            for (&adj, &(di, dj)) in adj_graph[&(i, j)]
                .iter()
                .zip([(1, 0), (-1, 0), (0, 1), (0, -1)].iter())
            {
                if adj {
                    let adj_i = i.saturating_add_signed(di);
                    let adj_j = j.saturating_add_signed(dj);
                    if not_visited.contains(&(adj_i, adj_j)) {
                        stack.push((adj_i, adj_j));
                    }
                }
            }
        }

        total += area * fences;
    }

    total as i32
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
        let result = solve(io_utils::read_file("inputs/12.in"));
        let solution = 1370258;
        assert_eq!(result, solution);
    }
}
