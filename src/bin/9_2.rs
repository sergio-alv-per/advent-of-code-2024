mod io_utils;

#[derive(Debug, Clone, Copy)]
enum Blocks {
    File { id: i128, size: i128 },
    Space { size: i128 },
}

fn line_to_vec(line: &str) -> Vec<Blocks> {
    let mut deque: Vec<Blocks> = Vec::with_capacity(line.len());
    let mut next_is_file = true;
    let mut id = 0;

    for c in line.chars() {
        let size = c.to_digit(10).expect("Found char that is not a number.") as i128;
        if next_is_file {
            deque.push(Blocks::File { id, size });
            next_is_file = false;
            id += 1;
        } else {
            deque.push(Blocks::Space { size });
            next_is_file = true;
        }
    }

    deque
}

fn compute_checksum(blocks: &Vec<Blocks>) -> i128 {
    blocks
        .iter()
        .fold((0, 0), |(checksum, index), block| match block {
            Blocks::Space { size } => (checksum, index + size),
            Blocks::File { id, size } => (
                checksum + (index..index + size).map(|i| i * id).sum::<i128>(),
                index + size,
            ),
        })
        .0
}

fn solve(lines: Vec<String>) -> i128 {
    let mut blocks_vec = line_to_vec(&lines[0]);
    let mut current_block_i: usize = blocks_vec.len() - 1;

    while current_block_i > 0 {
        let current_block = blocks_vec[current_block_i];
        match current_block {
            Blocks::Space { .. } => current_block_i -= 1,
            Blocks::File {
                id,
                size: file_size,
            } => {
                for j in 0..current_block_i {
                    let previous_block = blocks_vec[j];
                    match previous_block {
                        Blocks::Space { size: space_size } if space_size >= file_size => {
                            blocks_vec[j] = Blocks::File {
                                id,
                                size: file_size,
                            };

                            blocks_vec[current_block_i] = Blocks::Space { size: file_size };

                            if space_size > file_size {
                                blocks_vec.insert(
                                    j + 1,
                                    Blocks::Space {
                                        size: space_size - file_size,
                                    },
                                );

                                current_block_i += 1;
                            }

                            break;
                        }
                        _ => {}
                    }
                }

                current_block_i -= 1
            }
        }
    }
    compute_checksum(&blocks_vec)
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
        let result = solve(io_utils::read_file("inputs/input_file.in"));
        let solution = 0;
        assert_eq!(result, solution);
    }
}
