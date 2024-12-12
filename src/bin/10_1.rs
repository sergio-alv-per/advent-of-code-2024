mod io_utils;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct GridPos {
    i: usize,
    j: usize,
}

struct Marcher {
    origin: GridPos,
    current: GridPos,
}

fn get_adj(gp: &GridPos, di: i32, dj: i32, grid: &Vec<Vec<u32>>) -> Option<(u32, GridPos)> {
    let adj_i: usize = (gp.i as i32 + di).try_into().ok()?;
    let adj_j: usize = (gp.j as i32 + dj).try_into().ok()?;

    let row: &Vec<u32> = grid.get(adj_i)?;
    let adj_val = row.get(adj_j).copied()?;

    Some((adj_val, GridPos { i: adj_i, j: adj_j }))
}

fn get_reachable_starting_from(gp: &GridPos, grid: &Vec<Vec<u32>>) -> i32 {
    let mut stack = Vec::new();
    stack.push(Marcher {
        origin: GridPos { i: gp.i, j: gp.j },
        current: GridPos { i: gp.i, j: gp.j },
    });

    let mut visited: Vec<GridPos> = Vec::new();
    let mut peaks = 0;

    while let Some(marcher) = stack.pop() {
        for (di, dj) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some((adj_val, adj_gp)) = get_adj(&marcher.current, di, dj, &grid) {
                if !visited.contains(&adj_gp) {
                    let current_val = grid[marcher.current.i][marcher.current.j];
                    if adj_val == current_val + 1 {
                        visited.push(adj_gp);
                        match adj_val {
                            9 => {
                                peaks += 1;
                            }
                            _ => {
                                stack.push(Marcher {
                                    origin: marcher.origin,
                                    current: adj_gp,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    peaks
}

fn solve(lines: Vec<String>) -> i32 {
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total_peaks = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == 0 {
                total_peaks += get_reachable_starting_from(&GridPos { i, j }, &grid);
            }
        }
    }

    total_peaks
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
        let result = solve(io_utils::read_file("inputs/10.in"));
        let solution = 587;
        assert_eq!(result, solution);
    }
}
