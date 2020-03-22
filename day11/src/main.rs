use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Element {
    microchip: usize,
    generator: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    elevator: usize,
    elements: Vec<Element>,
}

impl State {
    fn get_microchips_at_floor(&self, floor: usize) -> Vec<usize> {
        self.elements
            .iter()
            .enumerate()
            .filter(|(_, element)| element.microchip == floor)
            .map(|(index, _)| index)
            .collect()
    }

    fn get_generators_at_floor(&self, floor: usize) -> Vec<usize> {
        self.elements
            .iter()
            .enumerate()
            .filter(|(_, element)| element.generator == floor)
            .map(|(index, _)| index)
            .collect()
    }

    fn is_valid(&self) -> bool {
        (1..=4).all(|floor| {
            let microchips = self.get_microchips_at_floor(floor);
            let generators = self.get_generators_at_floor(floor);

            if generators.is_empty() {
                true
            } else {
                let is_unpaired_microchip = !microchips
                    .iter()
                    .all(|microchip| generators.contains(microchip));
                !is_unpaired_microchip
            }
        })
    }

    fn is_final(&self) -> bool {
        self.elements
            .iter()
            .all(|element| element.microchip == 4 && element.generator == 4)
    }
}

fn get_all_next_possible_states(current_state: &State) -> Vec<State> {
    let floor = current_state.elevator;
    let is_bottom_floor = floor == 1;
    let is_top_floor = floor == 4;

    let microchips = current_state.get_microchips_at_floor(floor);
    let generators = current_state.get_generators_at_floor(floor);

    let mut next_states = Vec::new();

    // moving one microchip
    for microchip_index in microchips.iter() {
        if !is_top_floor {
            let mut next_state = current_state.clone();
            next_state.elevator += 1;
            next_state.elements[*microchip_index].microchip += 1;
            next_states.push(next_state);
        }

        if !is_bottom_floor {
            let mut next_state = current_state.clone();
            next_state.elevator -= 1;
            next_state.elements[*microchip_index].microchip -= 1;
            next_states.push(next_state);
        }
    }

    // moving two microchips
    for microchip_indices in microchips.iter().combinations(2) {
        let microchip_index_1 = microchip_indices[0];
        let microchip_index_2 = microchip_indices[1];

        if !is_top_floor {
            let mut next_state = current_state.clone();
            next_state.elevator += 1;
            next_state.elements[*microchip_index_1].microchip += 1;
            next_state.elements[*microchip_index_2].microchip += 1;
            next_states.push(next_state);
        }

        if !is_bottom_floor {
            let mut next_state = current_state.clone();
            next_state.elevator -= 1;
            next_state.elements[*microchip_index_1].microchip -= 1;
            next_state.elements[*microchip_index_2].microchip -= 1;
            next_states.push(next_state);
        }
    }

    // moving one generator
    for generator_index in generators.iter() {
        if !is_top_floor {
            let mut next_state = current_state.clone();
            next_state.elevator += 1;
            next_state.elements[*generator_index].generator += 1;
            next_states.push(next_state);
        }

        if !is_bottom_floor {
            let mut next_state = current_state.clone();
            next_state.elevator -= 1;
            next_state.elements[*generator_index].generator -= 1;
            next_states.push(next_state);
        }
    }

    // moving two generators
    for generator_indices in generators.iter().combinations(2) {
        let generator_index_1 = generator_indices[0];
        let generator_index_2 = generator_indices[1];

        if !is_top_floor {
            let mut next_state = current_state.clone();
            next_state.elevator += 1;
            next_state.elements[*generator_index_1].generator += 1;
            next_state.elements[*generator_index_2].generator += 1;
            next_states.push(next_state);
        }

        if !is_bottom_floor {
            let mut next_state = current_state.clone();
            next_state.elevator -= 1;
            next_state.elements[*generator_index_1].generator -= 1;
            next_state.elements[*generator_index_2].generator -= 1;
            next_states.push(next_state);
        }
    }

    // moving one microchip and one generator
    for microchip_index in microchips.iter() {
        for generator_index in generators.iter() {
            if microchip_index != generator_index {
                continue;
            }
            if !is_top_floor {
                let mut next_state = current_state.clone();
                next_state.elevator += 1;
                next_state.elements[*microchip_index].microchip += 1;
                next_state.elements[*generator_index].generator += 1;
                next_states.push(next_state);
            }

            if !is_bottom_floor {
                let mut next_state = current_state.clone();
                next_state.elevator -= 1;
                next_state.elements[*microchip_index].microchip -= 1;
                next_state.elements[*generator_index].generator -= 1;
                next_states.push(next_state);
            }
        }
    }

    next_states
}

fn main() {
    let initial_state = State {
        elevator: 1,
        elements: vec![
            Element { microchip: 2, generator: 1 },
            Element { microchip: 1, generator: 1 },
            Element { microchip: 2, generator: 1 },
            Element { microchip: 1, generator: 1 },
            Element { microchip: 1, generator: 1 },
            /*Element { microchip: 1, generator: 1},
            Element { microchip: 1, generator: 1}*/
        ],
    };

    let mut states: HashMap<State, usize> = HashMap::new();
    let mut states_to_check = VecDeque::new();
    states_to_check.push_back((initial_state, 0));

    while !states_to_check.is_empty() {
        let (state, steps) = states_to_check.pop_front().unwrap();
        if state.is_final() {
            println!("{}", steps);
            break;
        }

        if states.contains_key(&state) {
            continue;
        }

        states.insert(state.clone(), steps);
        let next_possible_states = get_all_next_possible_states(&state);
        for next_state in next_possible_states
            .into_iter()
            .filter(|state| state.is_valid())
        {
            states_to_check.push_back((next_state, steps + 1));
        }
    }
}
