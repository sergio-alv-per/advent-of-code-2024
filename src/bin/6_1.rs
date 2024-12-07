mod io_utils;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
enum Square {
    Free,
    Visited,
    Obstacle,
}

#[derive(Debug)]
struct Position(usize, usize);

#[derive(Debug)]
struct LabMap {
    map: Vec<Vec<Square>>,
    rows: usize,
    cols: usize,
    guard_position: Position,
    guard_direction: Direction,
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
                        Square::Visited
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

fn solve(lines: Vec<String>) -> i32 {
    let mut lab_map = input_to_map(&lines);

    loop {
        let Position(i, j) = lab_map.guard_position;

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
            lab_map.map[next_i][next_j] = Square::Visited;
        }
    }

    lab_map
        .map
        .iter()
        .map(|row| row.iter().filter(|&s| *s == Square::Visited).count() as i32)
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
        let result = solve(io_utils::read_file("inputs/6.in"));
        let solution = 5551;
        assert_eq!(result, solution);
    }
}
