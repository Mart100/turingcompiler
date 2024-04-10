mod ADD;
mod ISZERO;
mod LOAD;
mod MUL;
mod STORE;
mod SUB;
mod helpers;

use crate::a5code_generator::AssemblyInstruction;

pub mod prelude {
    pub use super::helpers::*;
    pub use super::ADD::add_instructions;
    pub use super::ISZERO::iszero_instruction;
    pub use super::LOAD::load_instructions;
    pub use super::STORE::store_instructions;
    pub use super::*;
    pub use crate::symbols::{symtou8, TapeSymbols};
}

use prelude::*;

use self::{MUL::mul_instructions, SUB::sub_instructions};

// Transform Assembly Instructions into Turing Machine Tape and Instructions.
// The Turing Machine Instructions are in the following format:
// STATE, READ_SYMBOL, WRITE_SYMBOL, MOVE_DIRECTION, NEXT_STATE
// The Turing Machine Tape is a Vec of u8 numbers, and _ represents the blank symbol.

// returns (tape, instructions)
pub fn code_emission(assembly: Vec<AssemblyInstruction>) -> (Vec<String>, Vec<String>) {
    let mut tape_storage = Vec::<String>::new(); // This is the tape storage, which is used to save values.

    let start_a = symtou8(TapeSymbols::StartA).to_string();
    let ab_seperator = symtou8(TapeSymbols::ABseperator).to_string();
    let end_b = symtou8(TapeSymbols::EndB).to_string();
    let end_c = symtou8(TapeSymbols::EndC).to_string();

    let tape_working_area = format!(
        "{} 0 0 0 0 0 0 0 0 {} 0 0 0 0 0 0 0 0 {} 0 0 0 0 0 0 0 0 {}",
        start_a, ab_seperator, end_b, end_c
    )
    .split_whitespace()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut instructions = Vec::new();
    instructions.push("\nSTART 5 5 S 0END".to_string());

    let mut instruction_counter = 1 as u32;

    let storage_seperator = &symtou8(TapeSymbols::StorageSeperator).to_string();

    let start_to_end = |i: u32| format!("\n{}END 5 5 S {}START", i - 1, i);

    for instruction in assembly {
        let operation = instruction.operation.clone();
        let operand1 = instruction.operand1.clone();
        let operand2 = instruction.operand2.clone();

        match operation.as_str() {
            // Store a value in the tape storage
            "SAVE" => {
                let address = operand1.unwrap(); // This is the address to save
                let value = operand2.unwrap(); // This is the value to save
                let value_binary = format!("{:08b}", value.parse::<u8>().unwrap());

                let mut vec = value_binary
                    .split("")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                vec.push(storage_seperator.clone());

                // trim spaces on strings in vec
                vec = vec.iter().map(|s| s.trim().to_string()).collect();

                let mut new_tape_storage = vec.clone();
                new_tape_storage.extend(tape_storage);
                tape_storage = new_tape_storage;
            }
            // Load a value from the tape storage into the working area
            "LOAD" => {
                // This is the address to load from the storage
                let address = operand1.unwrap().replace("S", "").parse::<u32>().unwrap();
                // This is the working space to load into. Either A or B.
                let working_space = operand2.unwrap();

                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(load_instructions(
                    &instruction_counter,
                    address,
                    working_space,
                ));

                instruction_counter += 1;
            }
            // Add two values in the working area together
            "ADD" => {
                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(add_instructions(&instruction_counter));

                instruction_counter += 1;
            }
            // Subtract the value in B from the value in A
            "SUB" => {
                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(sub_instructions(&instruction_counter));

                instruction_counter += 1;
            }
            // Multiply B and C, and store the result in A
            "MUL" => {
                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(mul_instructions(&instruction_counter));

                instruction_counter += 1;
            }
            // Check if the value in A is zero, if it is put 0 in A, otherwise put 1 in A
            "ISZERO" => {
                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(iszero_instruction(&instruction_counter));

                instruction_counter += 1;
            }
            // Store a value from the working area into the tape storage
            "STORE" => {
                // This is the working space to load into. Either A or B.
                let working_space = operand1.unwrap();
                // This is the address to load from the storage
                let address = operand2.unwrap().replace("S", "").parse::<u32>().unwrap();

                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(start_to_end(instruction_counter));
                instructions.extend(store_instructions(
                    &instruction_counter,
                    address,
                    working_space,
                ));

                instruction_counter += 1;
            }
            _ => (),
        }
    }

    tape_storage.insert(0, storage_seperator.clone());

    let mut tape = tape_storage.clone();
    tape.push(format!("!{}", symtou8(TapeSymbols::Middle).to_string())); // Add center symbol
    tape.extend(tape_working_area);

    instructions.push(format!("{}END 5 5 S END", instruction_counter - 1));

    (tape, instructions)
}

fn get_assembly_instruction_header(instruction: &AssemblyInstruction) -> Vec<String> {
    format!("\n#\n### {}\n#\n", instruction.to_string())
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

fn format_instructions(instructions: String, instruction_counter: u32) -> Vec<String> {
    let mut string = instructions.clone();

    // if write is * write is the same as read
    // loop through all the instruction lines, if the read is a list (e.g. 0 1), split it into two lines
    let mut new_string = String::new();
    for line in string.split("\n") {
        let parts = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if parts.len() == 5 && !line.starts_with("#") {
            if parts[1].contains("(") {
                // get a list of all strings within the brackets
                let reads = parts[1].split("(").collect::<Vec<&str>>()[1]
                    .split(")")
                    .collect::<Vec<&str>>()[0]
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                for read in reads {
                    let write = if parts[2] == "*" {
                        read.clone()
                    } else {
                        parts[2].clone()
                    };
                    new_string.push_str(&format!(
                        "&{} {} {} {} &{}\n",
                        parts[0], read, write, parts[3], parts[4]
                    ));
                }
            } else {
                let read = parts[1].clone();
                let write = if parts[2] == "*" {
                    read.clone()
                } else {
                    parts[2].clone()
                };
                new_string.push_str(&format!(
                    "&{} {} {} {} &{}\n",
                    parts[0], read, write, parts[3], parts[4]
                ));
            }
        } else {
            new_string.push_str(&format!("{}\n", line));
        }
    }
    string = new_string;

    string = string
        .replace("&", &instruction_counter.to_string())
        .replace("StartA", &symtou8(TapeSymbols::StartA).to_string())
        .replace("ABsep", &symtou8(TapeSymbols::ABseperator).to_string())
        .replace("EndB", &symtou8(TapeSymbols::EndB).to_string())
        .replace("EndC", &symtou8(TapeSymbols::EndC).to_string())
        .replace("Middle", &symtou8(TapeSymbols::Middle).to_string())
        .replace("StSep", &symtou8(TapeSymbols::StorageSeperator).to_string())
        .replace("StMark", &symtou8(TapeSymbols::StorageMarker).to_string())
        .replace("MovH0", &symtou8(TapeSymbols::HasMovedHelper0).to_string())
        .replace("MovH1", &symtou8(TapeSymbols::HasMovedHelper1).to_string())
        .replace("MulH", &symtou8(TapeSymbols::MultiplyHelper).to_string())
        .replace(
            " _",
            &format!(" {}", &symtou8(TapeSymbols::Blank).to_string()),
        );

    // repeatably replace all double spaces with single spaces
    while string.contains("  ") {
        string = string.replace("  ", " ");
    }

    string.split("\n").map(|s| s.to_string()).collect()
}
