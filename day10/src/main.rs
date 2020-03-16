use regex::Regex;
use std::collections::HashMap;
use std::fs;

enum Destination {
    Bot(usize),
    Output(usize),
}

struct Low(Destination);
struct High(Destination);

struct Instruction {
    bot: usize,
    low: Low,
    high: High,
}

impl Instruction {
    fn new(bot: usize, low: Low, high: High) -> Self {
        Instruction { bot, low, high }
    }
}

#[derive(Default)]
struct Bot {
    microchip_1: Option<usize>,
    microchip_2: Option<usize>,
}

impl Bot {
    fn set_value(&mut self, value: usize) {
        if self.microchip_1.is_none() {
            self.microchip_1 = Some(value);
        } else if self.microchip_2.is_none() {
            self.microchip_2 = Some(value);
        }
    }

    fn can_proceed(&self) -> bool {
        self.microchip_1.is_some() && self.microchip_2.is_some()
    }

    fn get_low_high(&mut self) -> (usize, usize) {
        let value_1 = self.microchip_1.unwrap();
        let value_2 = self.microchip_2.unwrap();
        self.microchip_1 = None;
        self.microchip_2 = None;

        if value_1 < value_2 {
            (value_1, value_2)
        } else {
            (value_2, value_1)
        }
    }
}

fn execute(
    instruction: &Instruction,
    bots: &mut HashMap<usize, Bot>,
    outputs: &mut HashMap<usize, usize>,
) {
    let (low_value, high_value) = {
        let bot = bots.entry(instruction.bot).or_default();
        if !bot.can_proceed() {
            return;
        }

        bot.get_low_high()
    };

    let low_destination_bot = &instruction.low.0;
    let high_destination_bot = &instruction.high.0;

    match low_destination_bot {
        Destination::Bot(number) => {
            let bot = bots.entry(*number).or_default();
            bot.set_value(low_value);
        }
        Destination::Output(number) => {
            let output = outputs.entry(*number).or_default();
            *output = low_value;
        }
    }

    match high_destination_bot {
        Destination::Bot(number) => {
            let bot = bots.entry(*number).or_default();
            bot.set_value(high_value);
        }
        Destination::Output(number) => {
            let output = outputs.entry(*number).or_default();
            *output = high_value;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    let set_value_regex = Regex::new(r"^value (?P<value>\d+) goes to bot (?P<bot>\d+)$").unwrap();
    let pass_value_regex =
        Regex::new(r"^bot (?P<bot>\d+) gives low to (?P<low_dest>\S+) (?P<low_number>\d+) and high to (?P<high_dest>\S+) (?P<high_number>\d+)$").unwrap();

    let mut instructions = Vec::new();
    for line in input.lines() {
        if set_value_regex.is_match(line) {
            let caps = set_value_regex.captures(line).unwrap();
            let value = caps["value"].parse::<usize>().unwrap();
            let bot = caps["bot"].parse::<usize>().unwrap();

            let entry = bots.entry(bot).or_default();
            entry.set_value(value);
        } else if pass_value_regex.is_match(line) {
            let caps = pass_value_regex.captures(line).unwrap();
            let bot = caps["bot"].parse::<usize>().unwrap();
            let low_number = caps["low_number"].parse::<usize>().unwrap();
            let high_number = caps["high_number"].parse::<usize>().unwrap();

            let low = match &caps["low_dest"] {
                "bot" => Low(Destination::Bot(low_number)),
                "output" => Low(Destination::Output(low_number)),
                _ => panic!("unknown destination!"),
            };

            let high = match &caps["high_dest"] {
                "bot" => High(Destination::Bot(high_number)),
                "output" => High(Destination::Output(high_number)),
                _ => panic!("unknown destination!"),
            };

            instructions.push(Instruction::new(bot, low, high));
        }
    }

    let mut bot_comparing_17_61 = 0;
    while bots.values().any(|bot| bot.can_proceed()) {
        for instruction in instructions.iter() {
            execute(instruction, &mut bots, &mut outputs);

            for (number, bot) in bots.iter() {
                if bot.microchip_1 == Some(17) && bot.microchip_2 == Some(61)
                    || bot.microchip_2 == Some(17) && bot.microchip_1 == Some(61)
                {
                    bot_comparing_17_61 = *number;
                }
            }
        }
    }

    let output_0 = outputs.get(&0).unwrap();
    let output_1 = outputs.get(&1).unwrap();
    let output_2 = outputs.get(&2).unwrap();

    assert_eq!(27, bot_comparing_17_61);
    assert_eq!(13_727, output_0 * output_1 * output_2);
}
