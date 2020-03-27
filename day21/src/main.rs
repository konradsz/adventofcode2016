#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

fn swap_position(password: &str, position_1: usize, position_2: usize) -> String {
    let letter_1 = password.chars().nth(position_1).unwrap();
    let letter_2 = password.chars().nth(position_2).unwrap();

    let mut new_password = String::from(password);
    new_password.replace_range(position_1..=position_1, &letter_2.to_string());
    new_password.replace_range(position_2..=position_2, &letter_1.to_string());

    new_password
}

fn swap_letters(password: &str, letter_1: char, letter_2: char) -> String {
    let position_1 = password.find(letter_1).unwrap();
    let position_2 = password.find(letter_2).unwrap();

    swap_position(password, position_1, position_2)
}

fn rotate_left(password: &str, steps: usize) -> String {
    String::from(&password[steps..]) + &password[0..steps]
}

fn rotate_right(password: &str, steps: usize) -> String {
    String::from(&password[password.len() - steps..]) + &password[0..password.len() - steps]
}

fn rotate_right_based_on(password: &str, letter: char) -> String {
    let position = password.find(letter).unwrap();
    let mut new_password = rotate_right(password, 1);
    new_password = rotate_right(&new_password, position);
    if position >= 4 {
        new_password = rotate_right(&new_password, 1);
    }

    new_password
}

fn rotate_left_based_on(password: &str, letter: char) -> String {
    let position = password.find(letter).unwrap();
    let rotates = match position {
        0 => 1,
        1 => 1,
        2 => 6,
        3 => 2,
        4 => 7,
        5 => 3,
        6 => 8,
        7 => 4,
        _ => unreachable!(),
    };

    rotate_left(password, rotates)
}

fn reverse(password: &str, from: usize, to: usize) -> String {
    let reversed = password[from..=to].chars().rev().collect::<String>();

    String::from(&password[0..from]) + &reversed + &password[to + 1..]
}

fn move_positions(password: &str, position_1: usize, position_2: usize) -> String {
    let mut new_password = String::from(password);
    let letter = password.chars().nth(position_1).unwrap();
    new_password.replace_range(position_1..=position_1, "");
    new_password.insert(position_2, letter);

    new_password
}

fn execute(instruction: &str, password: &str, inverse: bool) -> String {
    lazy_static! {
        static ref SWAP_POSITION: Regex =
            Regex::new(r"swap position (?P<p1>\d{1}) with position (?P<p2>\d{1})").unwrap();
        static ref SWAP_LETTERS: Regex =
            Regex::new(r"swap letter (?P<l1>\S{1}) with letter (?P<l2>\S{1})").unwrap();
        static ref ROTATE_LEFT: Regex = Regex::new(r"rotate left (?P<steps>\d{1})").unwrap();
        static ref ROTATE_RIGHT: Regex = Regex::new(r"rotate right (?P<steps>\d{1})").unwrap();
        static ref ROTATE_BASED: Regex =
            Regex::new(r"rotate based on position of letter (?P<l>\S{1})").unwrap();
        static ref REVERSE: Regex =
            Regex::new(r"reverse positions (?P<from>\d{1}) through (?P<to>\d{1})").unwrap();
        static ref MOVE_POSITION: Regex =
            Regex::new(r"move position (?P<p1>\d{1}) to position (?P<p2>\d{1})").unwrap();
    }

    if SWAP_POSITION.is_match(instruction) {
        let caps = SWAP_POSITION.captures(instruction).unwrap();
        swap_position(
            &password,
            caps["p1"].parse::<usize>().unwrap(),
            caps["p2"].parse::<usize>().unwrap(),
        )
    } else if SWAP_LETTERS.is_match(instruction) {
        let caps = SWAP_LETTERS.captures(instruction).unwrap();
        swap_letters(
            &password,
            caps["l1"].chars().nth(0).unwrap(),
            caps["l2"].chars().nth(0).unwrap(),
        )
    } else if ROTATE_LEFT.is_match(instruction) {
        let caps = ROTATE_LEFT.captures(instruction).unwrap();
        if !inverse {
            rotate_left(&password, caps["steps"].parse::<usize>().unwrap())
        } else {
            rotate_right(&password, caps["steps"].parse::<usize>().unwrap())
        }
    } else if ROTATE_RIGHT.is_match(instruction) {
        let caps = ROTATE_RIGHT.captures(instruction).unwrap();
        if !inverse {
            rotate_right(&password, caps["steps"].parse::<usize>().unwrap())
        } else {
            rotate_left(&password, caps["steps"].parse::<usize>().unwrap())
        }
    } else if ROTATE_BASED.is_match(instruction) {
        let caps = ROTATE_BASED.captures(instruction).unwrap();
        if !inverse {
            rotate_right_based_on(&password, caps["l"].chars().nth(0).unwrap())
        } else {
            rotate_left_based_on(&password, caps["l"].chars().nth(0).unwrap())
        }
    } else if REVERSE.is_match(instruction) {
        let caps = REVERSE.captures(instruction).unwrap();
        reverse(
            &password,
            caps["from"].parse::<usize>().unwrap(),
            caps["to"].parse::<usize>().unwrap(),
        )
    } else if MOVE_POSITION.is_match(instruction) {
        let caps = MOVE_POSITION.captures(instruction).unwrap();
        if !inverse {
            move_positions(
                &password,
                caps["p1"].parse::<usize>().unwrap(),
                caps["p2"].parse::<usize>().unwrap(),
            )
        } else {
            move_positions(
                &password,
                caps["p2"].parse::<usize>().unwrap(),
                caps["p1"].parse::<usize>().unwrap(),
            )
        }
    } else {
        panic!("unknown instruction!");
    }
}

fn execute_instructions(input: &str, password: &str, inverse: bool) -> String {
    let mut password = String::from(password);

    if !inverse {
        for line in input.lines() {
            password = execute(line, &password, inverse);
        }
    } else {
        for line in input.lines().rev() {
            password = execute(line, &password, inverse);
        }
    }
    password
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!("baecdfgh", execute_instructions(input, "abcdefgh", false));
    assert_eq!("cegdahbf", execute_instructions(input, "fbgdceah", true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrambling() {
        let mut password = String::from("abcde");
        password = swap_position(&password, 4, 0);
        assert_eq!("ebcda", &password);

        password = swap_letters(&password, 'd', 'b');
        assert_eq!("edcba", &password);

        password = reverse(&password, 0, 4);
        assert_eq!("abcde", &password);

        password = rotate_left(&password, 1);
        assert_eq!("bcdea", &password);

        password = move_positions(&password, 1, 4);
        assert_eq!("bdeac", &password);

        password = move_positions(&password, 3, 0);
        assert_eq!("abdec", &password);

        password = rotate_right_based_on(&password, 'b');
        assert_eq!("ecabd", &password);

        password = rotate_right_based_on(&password, 'd');
        assert_eq!("decab", &password);
    }
}
