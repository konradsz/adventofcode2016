use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn part_1(input: &str) -> i32 {
    let mut position = (0, 0);
    let mut current_direction = Direction::North;

    input.split(", ").for_each(|instruction| {
        let (direction, steps) = parse_instruction(instruction, current_direction);
        current_direction = direction;
        move_towards(current_direction, steps, &mut position);
    });

    position.0.abs() + position.1.abs()
}

fn part_2(input: &str) -> i32 {
    let mut position = (0, 0);
    let mut current_direction = Direction::North;
    let mut visited_locations = HashSet::new();

    input.split(", ").find(|instruction| {
        let (direction, steps) = parse_instruction(instruction, current_direction);
        current_direction = direction;
        move_until_location_visited_twice(
            current_direction,
            steps,
            &mut position,
            &mut visited_locations,
        )
    });

    position.0.abs() + position.1.abs()
}

fn parse_instruction(instruction: &str, direction: Direction) -> (Direction, i32) {
    let direction = if instruction.contains('L') {
        turn_left(direction)
    } else if instruction.contains('R') {
        turn_right(direction)
    } else {
        panic!("quo vadis?");
    };

    (
        direction,
        instruction
            .trim_matches(|c| c == 'L' || c == 'R')
            .parse::<i32>()
            .unwrap(),
    )
}

fn move_towards(direction: Direction, steps: i32, position: &mut (i32, i32)) {
    match direction {
        Direction::North => position.1 -= steps,
        Direction::East => position.0 += steps,
        Direction::South => position.1 += steps,
        Direction::West => position.0 -= steps,
    }
}

fn move_until_location_visited_twice(
    direction: Direction,
    steps: i32,
    position: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> bool {
    (0..steps).any(|_| {
        match direction {
            Direction::North => position.1 -= 1,
            Direction::East => position.0 += 1,
            Direction::South => position.1 += 1,
            Direction::West => position.0 -= 1,
        }
        !visited.insert(*position)
    })
}

fn turn_left(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(253, part_1(input));
    assert_eq!(126, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("R2, L3"), 5);
        assert_eq!(part_1("R2, R2, R2"), 2);
        assert_eq!(part_1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("R8, R4, R4, R8"), 4);
    }
}
