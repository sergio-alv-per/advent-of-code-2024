use std::collections::VecDeque;

mod io_utils;

enum Blocks {
    File { id: i128, size: i128 },
    Space { size: i128 },
}

fn recalculate_checksum_and_index(
    checksum: i128,
    index: i128,
    size: i128,
    id: i128,
) -> (i128, i128) {
    let new_checksum = checksum
        + (index..index + size)
            .map(|i| (i * id) as i128)
            .sum::<i128>();
    let new_index = index + size;
    (new_checksum, new_index)
}

fn line_to_deque(line: &str) -> VecDeque<Blocks> {
    let mut deque: VecDeque<Blocks> = VecDeque::with_capacity(line.len());
    let mut next_is_file = true;
    let mut id = 0;

    for c in line.chars() {
        let size = c.to_digit(10).expect("Found char that is not a number.") as i128;
        if next_is_file {
            deque.push_back(Blocks::File { id, size });
            next_is_file = false;
            id += 1;
        } else {
            deque.push_back(Blocks::Space { size });
            next_is_file = true;
        }
    }

    deque
}

fn solve(lines: Vec<String>) -> i128 {
    let mut deque = line_to_deque(&lines[0]);
    let mut index = 0;
    let mut checksum: i128 = 0;

    loop {
        match deque.pop_front() {
            None => {
                // No items left
                break;
            }
            Some(Blocks::File { id, size }) => {
                (checksum, index) = recalculate_checksum_and_index(checksum, index, size, id);
            }
            Some(Blocks::Space { size: space_size }) => {
                match deque.pop_back() {
                    None => {
                        // No items left at back, only front space. Break
                        break;
                    }
                    Some(Blocks::Space { .. }) => {
                        // Restart the loop, discard the back space, push back the front one.
                        deque.push_front(Blocks::Space { size: space_size });
                    }
                    Some(Blocks::File {
                        id,
                        size: file_size,
                    }) => {
                        // Space at front, file at back. Replace at front
                        if space_size < file_size {
                            // More file than space, push back remaining file
                            (checksum, index) =
                                recalculate_checksum_and_index(checksum, index, space_size, id);
                            deque.push_back(Blocks::File {
                                id,
                                size: file_size - space_size,
                            });
                        } else if space_size == file_size {
                            // Same space as file, do not push back anything
                            (checksum, index) =
                                recalculate_checksum_and_index(checksum, index, space_size, id);
                        } else {
                            // More space than file, push front remaining space
                            (checksum, index) =
                                recalculate_checksum_and_index(checksum, index, file_size, id);
                            deque.push_front(Blocks::Space {
                                size: space_size - file_size,
                            });
                        }
                    }
                }
            }
        }
    }

    checksum
}

fn main() {
    let lines = io_utils::read_stdin();
    let solution = solve(lines);
    println!("\n{solution}");
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
