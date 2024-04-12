use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug, Default, Clone)]
enum Action {
    #[default]
    Stay,
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
struct Instruction {
    write_symbol: u32,
    action: Action,
    next_state: u32,
}

#[derive(Debug)]
struct TuringMachine {
    current_state: u32,
    current_position: usize,
    tape: Vec<u32>,
    instructions: Vec<Instruction>,
    step: u64,
    end_state: u32,
}

impl TuringMachine {
    fn from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap();
        let mut lines = contents.lines().peekable();
        let mut instructions = Vec::new();

        let mut state_mapping = HashMap::new();
        let mut state_counter: u32 = 0;

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
            .map(|s| s.replace("!", "").parse::<u32>().unwrap())
            .collect();

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            } else if line.starts_with("#") {
                continue;
            }
            let current_state = parts[0].to_string();

            let current_state_num =
                *state_mapping
                    .entry(current_state.clone())
                    .or_insert_with(|| {
                        let current = state_counter;
                        state_counter += 1;
                        current
                    });

            let next_state = parts[4].to_string();

            let next_state_num = *state_mapping.entry(next_state.clone()).or_insert_with(|| {
                let current = state_counter;
                state_counter += 1;
                current
            });

            let read_symbol;
            if parts[1].parse::<u32>().is_ok() {
                read_symbol = parts[1].parse::<u32>().unwrap();
            } else if parts[1] == "_" {
                read_symbol = 255;
            } else {
                println!("{:?}", parts);
                panic!("Invalid read symbol");
            }

            let write_symbol;
            if parts[2].parse::<u32>().is_ok() {
                write_symbol = parts[2].parse::<u32>().unwrap();
            } else if parts[2] == "_" {
                write_symbol = 255;
            } else {
                println!("{:?}", parts);
                panic!("Invalid write symbol");
            }

            let action = match parts[3] {
                "S" => Action::Stay,
                "L" => Action::Left,
                "R" => Action::Right,
                _ => panic!("Invalid action"),
            };

            let instruction = Instruction {
                write_symbol,
                action,
                next_state: next_state_num,
            };

            instructions.resize(
                ((state_counter << 8) | 255 + 1) as usize,
                Default::default(),
            );

            //instructions.insert((current_state_num, read_symbol), instruction);
            instructions[((current_state_num << 8) | read_symbol) as usize] = instruction;
        }

        Self {
            current_state: 0,
            current_position: tape_start,
            tape: initial_tape,
            instructions,
            step: 0,
            end_state: state_counter,
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
        //self.print_current_step();

        // let instruction = self
        //     .instructions
        //     .get(&(self.current_state, self.tape[self.current_position]))
        //     .unwrap();

        let instruction = &self.instructions
            [((self.current_state << 8) | self.tape[self.current_position]) as usize];

        self.tape[self.current_position] = instruction.write_symbol;
        match instruction.action {
            Action::Stay => (),
            Action::Left => {
                if self.current_position == 0 {
                    self.tape.insert(0, 255);
                } else {
                    self.current_position -= 1;
                }
            }
            Action::Right => {
                self.current_position += 1;
                if self.current_position == self.tape.len() {
                    self.tape.push(255);
                }
            }
        }
        self.current_state = instruction.next_state;
        self.step += 1;
    }

    fn run(&mut self) {
        let start = Instant::now();
        loop {
            if self.current_state == self.end_state - 1 {
                println!("End state reached. Halting.");
                break;
            }
            self.step();
        }
        let duration = start.elapsed();
        println!("Program took {:?}", duration);
    }
}

fn main() {
    let mut turing_machine = TuringMachine::from_file("input.txt");
    turing_machine.run();
    println!("Final tape: {:?}", turing_machine.tape);
    println!("Total steps: {}", turing_machine.step);
}
