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
    stop: bool,
    previously_one: bool,
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
            stop: false,
            previously_one: true,
        }
    }

    fn run(&mut self) {
        loop {
            if self.stop || self.pc >= self.program.len() as i32 {
                break;
            }

            let instruction = self.program[self.pc as usize].as_str();
            let mut parts = instruction.split(' ');

            match parts.next().unwrap() {
                "cpy" => {
                    let value = self.get_immediate_or_from_register(parts.next().unwrap());
                    let register = Computer::parse_register(parts.next().unwrap());
                    self.cpy_value(value, register);
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
                    let value = self.get_immediate_or_from_register(parts.next().unwrap());
                    let offset = parts.next().unwrap().parse::<i32>().unwrap();
                    self.jnz(value, offset);
                }
                "out" => {
                    let value = self.get_immediate_or_from_register(parts.next().unwrap());
                    self.out(value);
                }
                _ => panic!("unknown instruction"),
            }
        }
    }

    fn get_immediate_or_from_register(&self, value: &str) -> i32 {
        value
            .parse::<i32>()
            .unwrap_or_else(|_| self.get_register(Computer::parse_register(value)))
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

    fn out(&mut self, value: i32) {
        if value == 0 {
            if self.previously_one {
                self.previously_one = false;
            } else {
                self.stop = true;
            }
        } else if value == 1 {
            if self.previously_one {
                self.stop = true;
            } else {
                self.previously_one = true;
            }
        }

        self.pc += 1;
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

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let input = input.trim();

    for i in 0.. {
        println!("{}", i);
        let mut computer = Computer::new(input);
        computer.a = i;
        computer.run();
    }
}
