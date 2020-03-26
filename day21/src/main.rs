use regex::Regex;
use std::fs;
/*struct Scrambler {
    password: String,
}

impl Scrambler {

}*/

fn swap_position(password: &str, position_1: usize, position_2: usize) -> String {
    let letter_1 = password.chars().nth(position_1).unwrap();
    let letter_2 = password.chars().nth(position_2).unwrap();
    /*let mut new_password = String::with_capacity(password.len());
    new_password.push_str(&password[0..pos_1]);
    new_password.push(char_2);
    new_password.push_str(&password[pos_1 + 1..pos_2]);
    new_password.push(char_1);
    new_password.push_str(&password[pos_2 + 1..]);*/
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
    //String::from(&password[steps..]) + &String::from(&password[0..steps])
    let s = steps % password.len();
    String::from(&password[s..]) + &password[0..s]
}

fn rotate_right(password: &str, steps: usize) -> String {
    //String::from(&password[password.len() - steps..]) + &String::from(&password[0..password.len() - steps])
    let s = steps % password.len();
    String::from(&password[password.len() - s..]) + &password[0..password.len() - s]
}

fn count_right_rotation(password: &str, letter: char) -> usize {
    let position = password.find(letter).unwrap();

    if position >= 4 {
        position + 2
    } else {
        position + 1
    }
}

fn rotate_right_based_on(password: &str, letter: char) -> String {
    let position = password.find(letter).unwrap();
    let mut new_password = rotate_right(password, 1);
    new_password = rotate_right(&new_password, position);
    if position >= 4 {
        new_password = rotate_right(&new_password, 1);
    }

    new_password
    /*let rotations = count_right_rotation(password, letter);
    let mut new_password = String::from(password);
    rotate_right(&new_password, rotations)*/
}

fn rotate_left_based_on(password: &str, letter: char) -> String {
    let position = password.find(letter).unwrap();
    let mut new_password = rotate_left(password, 1);
    new_password = rotate_left(&new_password, position);
    if position >= 4 {
        new_password = rotate_left(&new_password, 1);
    }

    new_password
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

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let swap_position_re = Regex::new(r"swap position (?P<p1>\d{1}) with position (?P<p2>\d{1})").unwrap();
    let swap_letter_re = Regex::new(r"swap letter (?P<l1>\S{1}) with letter (?P<l2>\S{1})").unwrap();
    let rotate_left_re = Regex::new(r"rotate left (?P<steps>\d{1})").unwrap();
    let rotate_right_re = Regex::new(r"rotate right (?P<steps>\d{1})").unwrap();
    let rotate_right_based_on_re = Regex::new(r"rotate based on position of letter (?P<l>\S{1})").unwrap();
    let reverse_re = Regex::new(r"reverse positions (?P<from>\d{1}) through (?P<to>\d{1})").unwrap();
    let move_positions_re = Regex::new(r"move position (?P<p1>\d{1}) to position (?P<p2>\d{1})").unwrap();

    let mut password = String::from("abcdefgh");
    for line in input.lines() {
        if swap_position_re.is_match(line) {
            let caps = swap_position_re.captures(line).unwrap();
            password = swap_position(&password, caps["p1"].parse::<usize>().unwrap(),caps["p2"].parse::<usize>().unwrap());
        } else if swap_letter_re.is_match(line) {
            let caps = swap_letter_re.captures(line).unwrap();
            password = swap_letters(&password, caps["l1"].chars().nth(0).unwrap(), caps["l2"].chars().nth(0).unwrap());
        } else if rotate_left_re.is_match(line) {
            let caps = rotate_left_re.captures(line).unwrap();
            password = rotate_left(&password, caps["steps"].parse::<usize>().unwrap());
        } else if rotate_right_re.is_match(line) {
            let caps = rotate_right_re.captures(line).unwrap();
            password = rotate_right(&password, caps["steps"].parse::<usize>().unwrap());
        } else if rotate_right_based_on_re.is_match(line) {
            let caps = rotate_right_based_on_re.captures(line).unwrap();
            password = rotate_right_based_on(&password, caps["l"].chars().nth(0).unwrap());
        } else if reverse_re.is_match(line) {
            let caps = reverse_re.captures(line).unwrap();
            password = reverse(&password, caps["from"].parse::<usize>().unwrap(), caps["to"].parse::<usize>().unwrap());
        } else if move_positions_re.is_match(line) {
            let caps = move_positions_re.captures(line).unwrap();
            password = move_positions(&password, caps["p1"].parse::<usize>().unwrap(), caps["p2"].parse::<usize>().unwrap());
        }
    }

    println!("{}", password);

    /*let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let swap_position_re = Regex::new(r"swap position (?P<p1>\d{1}) with position (?P<p2>\d{1})").unwrap();
    let swap_letter_re = Regex::new(r"swap letter (?P<l1>\S{1}) with letter (?P<l2>\S{1})").unwrap();
    let rotate_left_re = Regex::new(r"rotate left (?P<steps>\d{1})").unwrap();
    let rotate_right_re = Regex::new(r"rotate right (?P<steps>\d{1})").unwrap();
    let rotate_right_based_on_re = Regex::new(r"rotate based on position of letter (?P<l>\S{1})").unwrap();
    let reverse_re = Regex::new(r"reverse positions (?P<from>\d{1}) through (?P<to>\d{1})").unwrap();
    let move_positions_re = Regex::new(r"move position (?P<p1>\d{1}) to position (?P<p2>\d{1})").unwrap();

    let mut password = String::from("decab");
    for line in input.lines().rev() {
        if swap_position_re.is_match(line) {
            let caps = swap_position_re.captures(line).unwrap();
            password = swap_position(&password, caps["p1"].parse::<usize>().unwrap(),caps["p2"].parse::<usize>().unwrap());
        } else if swap_letter_re.is_match(line) {
            let caps = swap_letter_re.captures(line).unwrap();
            password = swap_letters(&password, caps["l1"].chars().nth(0).unwrap(), caps["l2"].chars().nth(0).unwrap());
        } else if rotate_left_re.is_match(line) {
            let caps = rotate_left_re.captures(line).unwrap();
            password = rotate_right(&password, caps["steps"].parse::<usize>().unwrap());
        } else if rotate_right_re.is_match(line) {
            let caps = rotate_right_re.captures(line).unwrap();
            password = rotate_left(&password, caps["steps"].parse::<usize>().unwrap());
        } else if rotate_right_based_on_re.is_match(line) {
            let caps = rotate_right_based_on_re.captures(line).unwrap();
            let count = count_right_rotation(&password, caps["l"].chars().nth(0).unwrap());
            if count < 5 {
                password = rotate_left(&password, count - 2);

            } else {
                password = rotate_left(&password, count - 1);
            }
        } else if reverse_re.is_match(line) {
            let caps = reverse_re.captures(line).unwrap();
            password = reverse(&password, caps["from"].parse::<usize>().unwrap(), caps["to"].parse::<usize>().unwrap());
        } else if move_positions_re.is_match(line) {
            let caps = move_positions_re.captures(line).unwrap();
            password = move_positions(&password, caps["p2"].parse::<usize>().unwrap(), caps["p1"].parse::<usize>().unwrap());
        }
    }

    println!("{}", password);*/

    /*let p = rotate_right_based_on("axxxxxxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xaxxxxxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxaxxxxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxxaxxxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxxxaxxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxxxxaxx", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxxxxxax", 'a');
    println!("{}",p);
    let p = rotate_right_based_on("xxxxxxxa", 'a');
    println!("{}",p);*/
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
