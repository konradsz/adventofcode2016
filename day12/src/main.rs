use std::fs;

enum Register {
    A,
    B,
    C,
    D,
}

struct Computer {
    program: Vec<String>,
    pc: i32,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Computer {
    fn new(program: &str) -> Self {
        Computer {
            program: program.lines().map(String::from).collect(),
            pc: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() as i32 {
                break;
            }

            let instruction = self.program[self.pc as usize].clone();
            let mut parts = instruction.split(' ');

            match parts.next().unwrap() {
                "cpy" => {
                    let value = parts.next().unwrap();
                    let register = Computer::parse_register(parts.next().unwrap());
                    if value.parse::<i32>().is_ok() {
                        self.cpy_value(value.parse::<i32>().unwrap(), register);
                    } else {
                        let value = self.get_register(Computer::parse_register(value));
                        self.cpy_value(value, register);
                    }
                }
                "inc" => {
                    let register = Computer::parse_register(parts.next().unwrap());
                    self.inc(register);
                }
                "dec" => {
                    let register = Computer::parse_register(parts.next().unwrap());
                    self.dec(register);
                }
                "jnz" => {
                    let value = parts.next().unwrap();
                    let offset = parts.next().unwrap().parse::<i32>().unwrap();
                    if value.parse::<i32>().is_ok() {
                        self.jnz(value.parse::<i32>().unwrap(), offset);
                    } else {
                        let value = self.get_register(Computer::parse_register(value));
                        self.jnz(value, offset);
                    }
                }
                _ => panic!("unknown instruction"),
            }
        }
    }

    fn cpy_value(&mut self, value: i32, destination: Register) {
        match destination {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
        }
        self.pc += 1;
    }

    fn inc(&mut self, destination: Register) {
        match destination {
            Register::A => self.a += 1,
            Register::B => self.b += 1,
            Register::C => self.c += 1,
            Register::D => self.d += 1,
        }
        self.pc += 1;
    }

    fn dec(&mut self, destination: Register) {
        match destination {
            Register::A => self.a -= 1,
            Register::B => self.b -= 1,
            Register::C => self.c -= 1,
            Register::D => self.d -= 1,
        }
        self.pc += 1;
    }

    fn jnz(&mut self, value: i32, offset: i32) {
        if value != 0 {
            self.pc += offset;
        } else {
            self.pc += 1;
        }
    }

    fn get_register(&self, register: Register) -> i32 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }

    fn parse_register(register: &str) -> Register {
        match register {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => panic!("unknown register"),
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut computer = Computer::new(input);
    computer.run();

    computer.a
}

fn part_2(input: &str) -> i32 {
    let mut computer = Computer::new(input);
    computer.c = 1;
    computer.run();

    computer.a
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    assert_eq!(318_009, part_1(input));
    assert_eq!(9_227_663, part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer() {
        let program = "cpy 41 a\n\
                       inc a\n\
                       inc a\n\
                       dec a\n\
                       jnz a 2\n\
                       dec a";
        let mut computer = Computer::new(program);
        computer.run();

        assert_eq!(42, computer.a);
    }
}
