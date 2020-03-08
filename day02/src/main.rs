use std::collections::HashMap;
use std::fs;

type Keypad = HashMap<(i32, i32), char>;

fn part_1(input: &str) -> String {
    let mut keypad = Keypad::new();
    keypad.insert((0, 0), '1');
    keypad.insert((1, 0), '2');
    keypad.insert((2, 0), '3');
    keypad.insert((0, 1), '4');
    keypad.insert((1, 1), '5');
    keypad.insert((2, 1), '6');
    keypad.insert((0, 2), '7');
    keypad.insert((1, 2), '8');
    keypad.insert((2, 2), '9');

    get_password(input, (1, 1), &keypad)
}

fn part_2(input: &str) -> String {
    let mut keypad = Keypad::new();
    keypad.insert((2, 0), '1');
    keypad.insert((1, 1), '2');
    keypad.insert((2, 1), '3');
    keypad.insert((3, 1), '4');
    keypad.insert((0, 2), '5');
    keypad.insert((1, 2), '6');
    keypad.insert((2, 2), '7');
    keypad.insert((3, 2), '8');
    keypad.insert((4, 2), '9');
    keypad.insert((1, 3), 'A');
    keypad.insert((2, 3), 'B');
    keypad.insert((3, 3), 'C');
    keypad.insert((2, 4), 'D');

    get_password(input, (0, 2), &keypad)
}

fn get_password(instructions: &str, mut position: (i32, i32), keypad: &Keypad) -> String {
    instructions
        .lines()
        .map(|instruction| {
            position = move_on_the_keypad(instruction, position, &keypad);
            position
        })
        .map(|position| keypad.get(&position).unwrap())
        .collect::<String>()
}

fn move_on_the_keypad(instruction: &str, mut position: (i32, i32), keypad: &Keypad) -> (i32, i32) {
    for movement in instruction.chars() {
        let new_position = match movement {
            'U' => (position.0, position.1 - 1),
            'D' => (position.0, position.1 + 1),
            'L' => (position.0 - 1, position.1),
            'R' => (position.0 + 1, position.1),
            _ => panic!(),
        };
        if keypad.contains_key(&new_position) {
            position = new_position;
        }
    }

    position
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!("95549", part_1(input));
    assert_eq!("D87AD", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "ULL\n\
                     RRDDD\n\
                     LURDL\n\
                     UUUUD";

        assert_eq!("1985", part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = "ULL\n\
                     RRDDD\n\
                     LURDL\n\
                     UUUUD";

        assert_eq!("5DB3", part_2(input));
    }
}
