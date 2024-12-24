mod io_utils;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn move_pos(&self, pos: &Position) -> Position {
        let offset = self.offset();
        Position {
            i: pos.i.saturating_add_signed(offset.0 as isize),
            j: pos.j.saturating_add_signed(offset.1 as isize),
        }
    }
}

enum Tile {
    Border,
    Empty,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    i: usize,
    j: usize,
}

fn char_to_direction(dir_char: char) -> Direction {
    match dir_char {
        '^' => Direction::North,
        '<' => Direction::West,
        '>' => Direction::East,
        'v' => Direction::South,
        _ => panic!("Unxpected char."),
    }
}

fn char_to_tile(tile_char: char) -> Tile {
    match tile_char {
        '#' => Tile::Border,
        '.' => Tile::Empty,
        'O' => Tile::Box,
        '@' => Tile::Robot,
        _ => panic!("Unexpected char."),
    }
}

fn gps(tiles: Vec<Vec<Tile>>) -> usize {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, t)| if let Tile::Box = t { 100 * i + j } else { 0 })
        })
        .sum()
}

fn solve(lines: Vec<String>) -> i32 {
    let mut input_parts = lines.split(|l| l.is_empty());
    let tiles = input_parts.next().unwrap();
    let directions = input_parts.next().unwrap();

    let mut tiles: Vec<Vec<Tile>> = tiles
        .iter()
        .map(|l| l.chars().map(char_to_tile).collect())
        .collect();
    let directions: Vec<Direction> = directions
        .iter()
        .map(|l| l.chars().map(char_to_direction))
        .flatten()
        .collect();

    let mut robot_pos: Position = Position { i: 0, j: 0 };
    for (i, row) in tiles.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            if let Tile::Robot = t {
                robot_pos = Position { i, j };
            }
        }
    }

    for d in directions {
        let mut current_pos = robot_pos;
        loop {
            current_pos = d.move_pos(&current_pos);

            match tiles[current_pos.i][current_pos.j] {
                Tile::Box => {
                    // Found box to be moved, continue
                }
                Tile::Border => {
                    // Found end of direction, can't move
                    break;
                }
                Tile::Empty => {
                    // Have to move the robot one step in the direction, and have to move
                    tiles[robot_pos.i][robot_pos.j] = Tile::Empty;
                    let adj_position = d.move_pos(&robot_pos);
                    tiles[adj_position.i][adj_position.j] = Tile::Robot;
                    if adj_position != current_pos {
                        tiles[current_pos.i][current_pos.j] = Tile::Box;
                    }
                    robot_pos = d.move_pos(&robot_pos);
                    break;
                }
                Tile::Robot => panic!("Found robot when iterating."),
            }
        }
    }

    gps(tiles) as i32
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
        let result = solve(io_utils::read_file("inputs/15.in"));
        let solution = 1538871;
        assert_eq!(result, solution);
    }
}
