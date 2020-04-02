use std::collections::VecDeque;

fn is_open(c: char) -> bool {
    !c.is_ascii_digit() && c != 'a'
}

struct State {
    position: (usize, usize),
    passcode: String,
    digest: String,
}

impl State {
    fn new(position: (usize, usize), passcode: &str) -> Self {
        Self {
            position,
            passcode: String::from(passcode),
            digest: String::from(&format!("{:x}", md5::compute(passcode))[0..4]),
        }
    }

    fn get_up(&self) -> Option<State> {
        if self.position.1 == 0 {
            None
        } else if is_open(self.digest.chars().nth(0).unwrap()) {
            Some(State::new(
                (self.position.0, self.position.1 - 1),
                &(self.passcode.clone() + "U"),
            ))
        } else {
            None
        }
    }

    fn get_down(&self) -> Option<State> {
        if self.position.1 == 3 {
            None
        } else if is_open(self.digest.chars().nth(1).unwrap()) {
            Some(State::new(
                (self.position.0, self.position.1 + 1),
                &(self.passcode.clone() + "D"),
            ))
        } else {
            None
        }
    }

    fn get_left(&self) -> Option<State> {
        if self.position.0 == 0 {
            None
        } else if is_open(self.digest.chars().nth(2).unwrap()) {
            Some(State::new(
                (self.position.0 - 1, self.position.1),
                &(self.passcode.clone() + "L"),
            ))
        } else {
            None
        }
    }

    fn get_right(&self) -> Option<State> {
        if self.position.0 == 3 {
            None
        } else if is_open(self.digest.chars().nth(3).unwrap()) {
            Some(State::new(
                (self.position.0 + 1, self.position.1),
                &(self.passcode.clone() + "R"),
            ))
        } else {
            None
        }
    }
}

fn find_shortest_and_longest_path(passcode: &str) -> (String, usize) {
    let initial_state = State::new((0, 0), passcode);

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(initial_state);

    let mut shortest_path_length = std::usize::MAX;
    let mut shortest_path = String::new();
    let mut longest_path_length = 0;

    while let Some(state) = queue.pop_front() {
        if state.position == (3, 3) {
            if state.passcode.len() < shortest_path_length {
                shortest_path_length = state.passcode.len();
                shortest_path = state.passcode.clone();
            }
            if state.passcode.len() > longest_path_length {
                longest_path_length = state.passcode.len();
            }
            continue;
        }

        if let Some(up) = state.get_up() {
            queue.push_back(up);
        }
        if let Some(down) = state.get_down() {
            queue.push_back(down);
        }
        if let Some(left) = state.get_left() {
            queue.push_back(left);
        }
        if let Some(right) = state.get_right() {
            queue.push_back(right);
        }
    }

    (String::from(&shortest_path[passcode.len()..]), longest_path_length - passcode.len())
}

fn main() {
    const PASSCODE: &str = "veumntbg";
    let (shortest_path, longest_path_length) = find_shortest_and_longest_path(PASSCODE);

    assert_eq!("DDRRULRDRD", shortest_path);
    assert_eq!(536, longest_path_length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_and_longest_path() {
        let (shortest_path, longest_path_length) = find_shortest_and_longest_path("ihgpwlah");
        assert_eq!(shortest_path, "DDRRRD");
        assert_eq!(longest_path_length, 370);

        let (shortest_path, longest_path_length) = find_shortest_and_longest_path("kglvqrro");
        assert_eq!(shortest_path, "DDUDRLRRUDRD");
        assert_eq!(longest_path_length, 492);

        let (shortest_path, longest_path_length) = find_shortest_and_longest_path("ulqzkmiv");
        assert_eq!(shortest_path, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(longest_path_length, 830);
    }
}
