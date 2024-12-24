use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

mod io_utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

enum EmptyTile {
    Empty,
    Start,
    End,
}
enum Tile {
    Border,
    Empty(EmptyTile),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    i: usize,
    j: usize,
}

fn char_to_tile(tile_char: char) -> Tile {
    match tile_char {
        '#' => Tile::Border,
        '.' => Tile::Empty(EmptyTile::Empty),
        'S' => Tile::Empty(EmptyTile::Start),
        'E' => Tile::Empty(EmptyTile::End),
        _ => panic!("Unexpected char."),
    }
}

fn tiles_to_graph(
    tiles: &Vec<Vec<Tile>>,
) -> HashMap<(Position, Direction), HashMap<(Position, Direction), i32>> {
    let mut graph: HashMap<(Position, Direction), HashMap<(Position, Direction), i32>> =
        HashMap::new();

    for (i, row) in tiles.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            if let Tile::Empty(et) = t {
                let current_pos = Position { i, j };
                let currpos_north = (current_pos, Direction::North);
                let currpos_south = (current_pos, Direction::South);
                let currpos_east = (current_pos, Direction::East);
                let currpos_west = (current_pos, Direction::West);

                // This allows starting in any direction
                let cost_between_dirs = match et {
                    EmptyTile::Start => 0,
                    _ => 1000,
                };
                graph.insert(currpos_north, HashMap::new());
                graph
                    .get_mut(&currpos_north)
                    .unwrap()
                    .insert(currpos_east, cost_between_dirs);
                graph
                    .get_mut(&currpos_north)
                    .unwrap()
                    .insert(currpos_west, cost_between_dirs);

                graph.insert(currpos_south, HashMap::new());
                graph
                    .get_mut(&currpos_south)
                    .unwrap()
                    .insert(currpos_east, cost_between_dirs);
                graph
                    .get_mut(&currpos_south)
                    .unwrap()
                    .insert(currpos_west, cost_between_dirs);

                graph.insert(currpos_east, HashMap::new());
                graph
                    .get_mut(&currpos_east)
                    .unwrap()
                    .insert(currpos_north, cost_between_dirs);
                graph
                    .get_mut(&currpos_east)
                    .unwrap()
                    .insert(currpos_south, cost_between_dirs);

                graph.insert(currpos_west, HashMap::new());
                graph
                    .get_mut(&currpos_west)
                    .unwrap()
                    .insert(currpos_north, cost_between_dirs);
                graph
                    .get_mut(&currpos_west)
                    .unwrap()
                    .insert(currpos_south, cost_between_dirs);

                let north_adj_position = Direction::North.move_pos(&current_pos);
                if let Tile::Empty(_) = tiles[north_adj_position.i][north_adj_position.j] {
                    graph
                        .get_mut(&currpos_north)
                        .unwrap()
                        .insert((north_adj_position, Direction::North), 1);
                    graph
                        .get_mut(&(north_adj_position, Direction::South))
                        .unwrap()
                        .insert(currpos_south, 1);
                }

                let west_adj_position = Direction::West.move_pos(&current_pos);
                if let Tile::Empty(_) = tiles[west_adj_position.i][west_adj_position.j] {
                    graph
                        .get_mut(&currpos_west)
                        .unwrap()
                        .insert((west_adj_position, Direction::West), 1);
                    graph
                        .get_mut(&(west_adj_position, Direction::East))
                        .unwrap()
                        .insert(currpos_east, 1);
                }
            }
        }
    }
    graph
}

fn dir(visited: &Vec<(Position, Direction)>, i: usize, j: usize) -> Option<Direction> {
    for &(pos, d) in visited {
        if pos.i == i && pos.j == j {
            return Some(d);
        }
    }
    return None;
}

fn solve(lines: Vec<String>) -> i32 {
    let tiles: Vec<Vec<Tile>> = lines
        .iter()
        .map(|l| l.chars().map(char_to_tile).collect())
        .collect();
    let graph = tiles_to_graph(&tiles);

    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;
    for (i, row) in tiles.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            match t {
                Tile::Empty(EmptyTile::Start) => {
                    start = Some(Position { i, j });
                }
                Tile::Empty(EmptyTile::End) => {
                    end = Some(Position { i, j });
                }
                _ => {}
            };
        }
    }

    let start = start.expect("No start fouond.");
    let end = end.expect("No end fouond.");

    let result = dijkstra(
        &(start, Direction::North),
        |n| graph[n].iter().map(|(k, v)| (k.clone(), v.clone())),
        |n| n.0.i == end.i && n.0.j == end.j,
    );

    let (visited, score) = result.unwrap();

    for (i, row) in tiles.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            match t {
                Tile::Empty(EmptyTile::Start) => {
                    print!("S");
                }
                Tile::Empty(EmptyTile::End) => {
                    print!("E");
                }
                Tile::Empty(EmptyTile::Empty) => match dir(&visited, i, j) {
                    Some(_) => print!("O"),
                    None => print!("."),
                },
                Tile::Border => {
                    print!("#");
                }
            };
        }
        println!("");
    }

    score
}

fn main() {
    let lines = io_utils::read_stdin();
    let solution = solve(lines);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore] // REMOVE ON PROBLEM FILES
    #[test]
    fn solution_correct() {
        let result = solve(io_utils::read_file("inputs/16.in"));
        let solution = 94444;
        assert_eq!(result, solution);
    }
}
