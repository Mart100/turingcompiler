use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Action {
    Stay,
    Move(Direction),
}

#[derive(Debug)]
struct Instruction {
    write_symbol: u8,
    action: Action,
    next_state: String,
}

#[derive(Debug)]
struct TuringMachine {
    current_state: String,
    current_position: usize,
    tape: Vec<u8>,
    instructions: HashMap<(String, u8), Instruction>,
}

impl TuringMachine {
    fn from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let mut lines = contents.lines().peekable();
        let mut instructions = HashMap::new();

        let tape_start = lines
            .peek()
            .unwrap()
            .split_whitespace()
            .enumerate()
            .find(|(_, s)| s.starts_with("!"))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let initial_tape = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.replace("!", "").parse::<u8>().unwrap())
            .collect();

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            } else if line.starts_with("#") {
                continue;
            }
            let current_state = parts[0].to_string();

            let read_symbol;
            if parts[1].parse::<u8>().is_ok() {
                read_symbol = parts[1].parse::<u8>().unwrap();
            } else if parts[1] == "_" {
                read_symbol = 255;
            } else {
                panic!("Invalid read symbol");
            }

            let write_symbol;
            if parts[2].parse::<u8>().is_ok() {
                write_symbol = parts[2].parse::<u8>().unwrap();
            } else if parts[2] == "_" {
                write_symbol = 255;
            } else {
                println!("{:?}", parts);
                panic!("Invalid write symbol");
            }

            let action = match parts[3] {
                "S" => Action::Stay,
                "L" => Action::Move(Direction::Left),
                "R" => Action::Move(Direction::Right),
                _ => panic!("Invalid action"),
            };
            let next_state = parts[4].to_string();

            let instruction = Instruction {
                write_symbol,
                action,
                next_state,
            };

            instructions.insert((current_state, read_symbol), instruction);
        }

        Self {
            current_state: "START".to_string(),
            current_position: tape_start,
            tape: initial_tape,
            instructions,
        }
    }

    fn print_current_step(&self) {
        let mut tape_str = String::new();
        for (i, symbol) in self.tape.iter().enumerate() {
            if i == self.current_position {
                tape_str.push_str(&format!("[{}]", symbol));
            } else {
                tape_str.push_str(&format!(" {} ", symbol));
            }
        }
        let state_str = format!("state: {}", self.current_state);
        let padding = " ".repeat(40 - state_str.len());
        println!("{}{}tape: {}", state_str, padding, tape_str);
    }

    fn step(&mut self) {
        if self.current_state == "END" {
            return;
        }

        self.print_current_step();

        let instruction = self
            .instructions
            .get(&(self.current_state.clone(), self.tape[self.current_position]))
            .unwrap();
        self.tape[self.current_position] = instruction.write_symbol;
        match instruction.action {
            Action::Stay => (),
            Action::Move(Direction::Left) => {
                if self.current_position == 0 {
                    self.tape.insert(0, 255);
                } else {
                    self.current_position -= 1;
                }
            }
            Action::Move(Direction::Right) => {
                self.current_position += 1;
                if self.current_position == self.tape.len() {
                    self.tape.push(255);
                }
            }
        }
        self.current_state = instruction.next_state.clone();
    }

    fn run(&mut self) {
        loop {
            if self.current_state == "END" {
                println!("End state reached. Halting.");
                break;
            }
            self.step();
        }
    }
}

fn main() {
    let mut turing_machine = TuringMachine::from_file("input.txt");
    turing_machine.run();
    println!("Final tape: {:?}", turing_machine.tape);
}
