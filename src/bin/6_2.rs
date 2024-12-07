mod io_utils;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Square {
    Free,
    Visited([bool; 4]), // each element in the array indicates direction
    Obstacle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(usize, usize);

#[derive(Debug, Clone)]
struct LabMap {
    map: Vec<Vec<Square>>,
    rows: usize,
    cols: usize,
    guard_position: Position,
    guard_direction: Direction,
}

enum LabMapResult {
    Loops,
    Exits(Option<Vec<Position>>),
}

fn input_to_map(lines: &Vec<String>) -> LabMap {
    let mut guard_position = Position(0, 0);
    let lab_map: Vec<Vec<Square>> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Square::Free,
                    '#' => Square::Obstacle,
                    '^' => {
                        guard_position = Position(i, j);
                        Square::Free
                    }
                    _ => panic!("Unexpected square"),
                })
                .collect()
        })
        .collect();

    LabMap {
        rows: lab_map.len(),
        cols: lab_map[0].len(),
        map: lab_map,
        guard_direction: Direction::North,
        guard_position,
    }
}

fn falls_into_loop(mut lab_map: LabMap, keep_visited: bool) -> LabMapResult {
    loop {
        let Position(i, j) = lab_map.guard_position;
        let dir = lab_map.guard_direction;

        match lab_map.map[i][j] {
            Square::Visited(dirs) => {
                if dirs[dir as usize] {
                    // Already visited this square in this direction, found loop!
                    return LabMapResult::Loops;
                } else {
                    let mut new_dirs = [false, false, false, false];
                    new_dirs[lab_map.guard_direction as usize] = true;

                    for (i, &d) in dirs.iter().enumerate() {
                        new_dirs[i] = new_dirs[i] || d;
                    }
                    lab_map.map[i][j] = Square::Visited(new_dirs);
                }
            }
            Square::Free => {
                let mut new_dirs = [false, false, false, false];
                new_dirs[lab_map.guard_direction as usize] = true;
                lab_map.map[i][j] = Square::Visited(new_dirs);
            }
            _ => {}
        }

        if (i == 0 && lab_map.guard_direction == Direction::North)
            || (i == lab_map.rows - 1 && lab_map.guard_direction == Direction::South)
            || (j == 0 && lab_map.guard_direction == Direction::West)
            || (j == lab_map.cols - 1 && lab_map.guard_direction == Direction::East)
        {
            break;
        }

        let direction_vector = match lab_map.guard_direction {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };

        let next_i: usize = ((i as i32) + direction_vector.0).try_into().unwrap();
        let next_j: usize = ((j as i32) + direction_vector.1).try_into().unwrap();

        let next_square = &lab_map.map[next_i][next_j];

        if *next_square == Square::Obstacle {
            lab_map.guard_direction = match lab_map.guard_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        } else {
            lab_map.guard_position = Position(next_i, next_j);
        }
    }

    if !keep_visited {
        LabMapResult::Exits(None)
    } else {
        let visited_positions: Vec<Position> = lab_map
            .map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, sq)| match sq {
                        Square::Visited(_) => Some(Position(i, j)),
                        _ => None,
                    })
                    .collect::<Vec<Position>>()
            })
            .collect::<Vec<Vec<Position>>>()
            .concat();

        LabMapResult::Exits(Some(visited_positions))
    }
}

fn solve(lines: Vec<String>) -> i32 {
    let lab_map = input_to_map(&lines);
    let mut loops = 0;

    let visited_positions = match falls_into_loop(lab_map.clone(), true) {
        LabMapResult::Exits(vs) => vs.unwrap(),
        _ => panic!("Loop on initial position."),
    };

    for Position(i, j) in visited_positions {
        if Position(i, j) != lab_map.guard_position {
            let mut lm2 = lab_map.clone();
            lm2.map[i][j] = Square::Obstacle;
            match falls_into_loop(lm2, false) {
                LabMapResult::Loops => loops += 1,
                _ => {}
            }
        }
    }
    loops
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
        let result = solve(io_utils::read_file("inputs/6.in"));
        let solution = 1939;
        assert_eq!(result, solution);
    }
}
