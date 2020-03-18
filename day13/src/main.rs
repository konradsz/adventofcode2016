use std::collections::HashMap;
use std::collections::VecDeque;

fn count_ones_in_binary_representation(mut number: usize) -> usize {
    let mut count = 0;
    while number > 0 {
        if number & 0b1 == 1 {
            count += 1;
        }
        number >>= 1;
    }
    count
}

fn find_empty_spaces(width: usize, height: usize, favorite_number: usize) -> Vec<(usize, usize)> {
    let mut maze = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let field = x * x + 3 * x + 2 * x * y + y + y * y + favorite_number;
            if count_ones_in_binary_representation(field) % 2 == 0 {
                maze.push((x, y));
            }
        }
    }

    maze
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_one_step(
    position: (usize, usize),
    direction: Direction,
    empty_spaces: &[(usize, usize)],
) -> Option<(usize, usize)> {
    let new_position = match direction {
        Direction::Up => (position.0, position.1.saturating_sub(1)),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Down => (position.0, position.1 + 1),
        Direction::Left => (position.0.saturating_sub(1), position.1),
    };

    if empty_spaces.contains(&new_position) {
        Some(new_position)
    } else {
        None
    }
}

fn find_shortest_path(
    empty_spaces: &[(usize, usize)],
    target: (usize, usize),
) -> Option<(usize, usize)> {
    let mut visited = HashMap::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back(((1, 1), 0));

    while !to_visit.is_empty() {
        let (current_position, steps_taken) = to_visit.pop_front().unwrap();

        if current_position == target {
            let unique_positions = visited.values().filter(|steps| **steps <= 50).count();
            return Some((steps_taken, unique_positions));
        }

        let entry = visited.entry(current_position).or_insert(steps_taken);
        if *entry < steps_taken {
            continue;
        } else {
            *entry = steps_taken;
        }

        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .for_each(|dir| {
            if let Some(next_position) = move_one_step(current_position, *dir, &empty_spaces) {
                to_visit.push_back((next_position, steps_taken + 1));
            }
        });
    }

    None
}

fn main() {
    const FAVORITE_NUMBER: usize = 1358;
    const TARGET: (usize, usize) = (31, 39);

    let empty_spaces = find_empty_spaces(50, 50, FAVORITE_NUMBER);
    let (steps_taken_to_target, unique_positions) =
        find_shortest_path(&empty_spaces, TARGET).unwrap();

    assert_eq!(96, steps_taken_to_target);
    assert_eq!(141, unique_positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_path() {
        let empty_spaces = find_empty_spaces(10, 7, 10);
        let (steps_taken_to_target, _) = find_shortest_path(&empty_spaces, (7, 4)).unwrap();

        assert_eq!(11, steps_taken_to_target);
    }
}
