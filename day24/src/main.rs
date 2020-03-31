use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

enum Tile {
    Wall,
    Empty,
    Poi(char),
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    poi_visited: u8,
}

impl State {
    fn new(position: (usize, usize), poi_visited: u8) -> Self {
        State {
            position,
            poi_visited,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut starting_position: (usize, usize) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x, tile) in line.chars().enumerate() {
            if tile.is_ascii_digit() {
                if tile == '0' {
                    starting_position = (x, y);
                }
                map[y].push(Tile::Poi(tile));
            } else if tile == '#' {
                map[y].push(Tile::Wall);
            } else if tile == '.' {
                map[y].push(Tile::Empty);
            }
        }
    }

    (map, starting_position)
}

fn visit_poi(map: &[Vec<Tile>], starting_position: (usize, usize), poi_to_visit: u8) -> usize {
    let get_tile = |position: (usize, usize), state: &State| -> Option<Tile> {
        match map[position.1][position.0] {
            Tile::Empty => Some(Tile::Empty),
            Tile::Poi(p) => {
                if p == '0' {
                    if state.poi_visited == 0b1111_1110 {
                        Some(Tile::Poi(p))
                    } else {
                        Some(Tile::Empty)
                    }
                } else if (state.poi_visited >> (p as u8 - b'0')) & 1 == 1 {
                    Some(Tile::Empty)
                } else {
                    Some(Tile::Poi(p))
                }
            }
            Tile::Wall => None,
        }
    };

    let initial_state = State::new(starting_position, 0);
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(initial_state.clone());

    let mut steps_taken_to_state: HashMap<State, usize> = HashMap::new();
    steps_taken_to_state.insert(initial_state, 0);

    while let Some(state) = queue.pop_front() {
        if state.poi_visited == poi_to_visit {
            return *steps_taken_to_state.get(&state).unwrap();
        }

        for direction in [(0, -1), (0, 1), (-1, 0), (1, 0)].iter() {
            let new_position = (
                (state.position.0 as i32 + direction.0) as usize,
                (state.position.1 as i32 + direction.1) as usize,
            );

            if let Some(steps) = steps_taken_to_state.get(&state) {
                let steps = *steps;
                if let Some(tile) = get_tile(new_position, &state) {
                    match tile {
                        Tile::Empty => {
                            let new_state = State::new(new_position, state.poi_visited);
                            if !steps_taken_to_state.contains_key(&new_state) {
                                steps_taken_to_state.insert(new_state.clone(), steps + 1);
                                queue.push_back(new_state.clone());
                            }
                        }
                        Tile::Poi(p) => {
                            let pois = state.poi_visited | (1 << (p as u8 - b'0'));
                            let new_state = State::new(new_position, pois);
                            steps_taken_to_state.insert(new_state.clone(), steps + 1);
                            queue.push_back(new_state.clone());
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    panic!("solution not found :(");
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let (map, starting_position) = parse_input(input);

    const PART_1_POI_TO_VISIT: u8 = 0b1111_1110;
    const PART_2_POI_TO_VISIT: u8 = 0b1111_1111;
    assert_eq!(470, visit_poi(&map, starting_position, PART_1_POI_TO_VISIT));
    assert_eq!(720, visit_poi(&map, starting_position, PART_2_POI_TO_VISIT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let input = "###########\n\
                     #0.1.....2#\n\
                     #.#######.#\n\
                     #4.......3#\n\
                     ###########";

        let (map, starting_position) = parse_input(input);
        assert_eq!(14, visit_poi(&map, starting_position, 0b0001_1110));
    }
}
