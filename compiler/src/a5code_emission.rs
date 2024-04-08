use crate::{
    a4code_generator::AssemblyInstruction,
    symbols::{symtou8, TapeSymbols},
};

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
    let tape_working_area = format!(
        "{} 0 0 0 0 0 0 0 0 {} 0 0 0 0 0 0 0 0 {}",
        start_a, ab_seperator, end_b
    )
    .split_whitespace()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut instructions = Vec::new();
    instructions.push("\nSTART 5 5 S 0START".to_string());

    let mut instruction_counter = 0 as u32;

    let storage_seperator = &symtou8(TapeSymbols::StorageSeperator).to_string();

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
                instructions.push(format!(
                    "\n{}END 5 5 S {}START",
                    instruction_counter,
                    instruction_counter + 1
                ));
                instructions.extend(load_instructions(
                    &instruction_counter,
                    address,
                    working_space,
                ));

                instruction_counter += 1;
            }
            // Add two values in the working area together
            "ADD" => {
                let operand1 = operand1.unwrap();
                let operand2 = operand2.unwrap();

                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(format!(
                    "\n{}END 5 5 S {}START",
                    instruction_counter,
                    instruction_counter + 1
                ));
                instructions.extend(addition_instructions(&instruction_counter));

                instruction_counter += 1;
            }
            // Store a value from the working area into the tape storage
            "STORE" => {
                // This is the working space to load into. Either A or B.
                let working_space = operand1.unwrap();
                // This is the address to load from the storage
                let address = operand2.unwrap().replace("S", "").parse::<u32>().unwrap();

                instructions.extend(get_assembly_instruction_header(&instruction));
                instructions.push(format!(
                    "\n{}END 5 5 S {}START",
                    instruction_counter,
                    instruction_counter + 1
                ));
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
    format!("\n#\n# {}\n#\n", instruction.to_string())
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

fn store_instructions(
    instruction_counter: &u32,
    address: u32,
    working_space: String,
) -> Vec<String> {
    let mut part1 = "\n&START 5 5 L &TO_S1_END\n".to_string();

    let mut part2 = go_to_storage(address, "SET_TO_7".to_string(), false)
        .trim()
        .to_string()
        + "\n";

    let mut part3 = "
# Set all the values to 7 untill the start of S1
&SET_TO_7 0 7 L &SET_TO_7
&SET_TO_7 1 7 L &SET_TO_7
&SET_TO_7 6 9 R &TO_MIDDLE_R
# Find the middle
&TO_MIDDLE_R 7 7 R &TO_MIDDLE_R
&TO_MIDDLE_R 1 1 R &TO_MIDDLE_R
&TO_MIDDLE_R 0 0 R &TO_MIDDLE_R
&TO_MIDDLE_R 2 2 R &TO_MIDDLE_R
&TO_MIDDLE_R 6 6 R &TO_MIDDLE_R
&TO_MIDDLE_R 5 5 R &COPY_VALUE
# Move Left until start of A
# Copy the first non-7/8 value
&COPY_VALUE 0 7 L &MOVE_ZERO
&COPY_VALUE 1 8 L &MOVE_ONE
&COPY_VALUE 2 2 R &COPY_VALUE
&COPY_VALUE 7 7 R &COPY_VALUE
&COPY_VALUE 8 8 R &COPY_VALUE
&COPY_VALUE 3 3 L &RESTORE_VALUE
# Move a 0 to the end of number A
&MOVE_ZERO 0 0 L &MOVE_ZERO
&MOVE_ZERO 1 1 L &MOVE_ZERO
&MOVE_ZERO 2 2 L &MOVE_ZERO
&MOVE_ZERO 3 3 L &MOVE_ZERO
&MOVE_ZERO 5 5 L &MOVE_ZERO
&MOVE_ZERO 6 6 L &MOVE_ZERO
&MOVE_ZERO 7 7 L &MOVE_ZERO
&MOVE_ZERO 8 8 L &MOVE_ZERO
&MOVE_ZERO 9 9 R &MOVE_ZERO_2
# Move a 0 to the left until the first 7, and replace it
&MOVE_ZERO_2 0 0 R &MOVE_ZERO_2
&MOVE_ZERO_2 1 1 R &MOVE_ZERO_2
&MOVE_ZERO_2 7 0 S &TO_MIDDLE_R
# Move a 1 to the start of the first number
&MOVE_ONE 0 0 L &MOVE_ONE
&MOVE_ONE 1 1 L &MOVE_ONE
&MOVE_ONE 2 2 L &MOVE_ONE
&MOVE_ONE 3 3 L &MOVE_ONE
&MOVE_ONE 5 5 L &MOVE_ONE
&MOVE_ONE 6 6 L &MOVE_ONE
&MOVE_ONE 7 7 L &MOVE_ONE
&MOVE_ONE 8 8 L &MOVE_ONE
&MOVE_ONE 9 9 R &MOVE_ONE_2
# Move a 1 to the left until the first 7, and replace it
&MOVE_ONE_2 0 0 R &MOVE_ONE_2
&MOVE_ONE_2 1 1 R &MOVE_ONE_2
&MOVE_ONE_2 7 1 S &TO_MIDDLE_R
# Restore number A, translate 7 to 0 and 8 to 1
&RESTORE_VALUE 7 0 L &RESTORE_VALUE
&RESTORE_VALUE 8 1 L &RESTORE_VALUE
&RESTORE_VALUE 2 2 L &RESET_STORAGE_MARKER
# Reset 9 to 6
&RESET_STORAGE_MARKER 5 5 L &RESET_STORAGE_MARKER
&RESET_STORAGE_MARKER 0 0 L &RESET_STORAGE_MARKER
&RESET_STORAGE_MARKER 1 1 L &RESET_STORAGE_MARKER
&RESET_STORAGE_MARKER 6 6 L &RESET_STORAGE_MARKER
&RESET_STORAGE_MARKER 9 6 R &RETURN_TO_MIDDLE
# Return to middle
&RETURN_TO_MIDDLE 0 0 R &RETURN_TO_MIDDLE
&RETURN_TO_MIDDLE 1 1 R &RETURN_TO_MIDDLE
&RETURN_TO_MIDDLE 6 6 R &RETURN_TO_MIDDLE
&RETURN_TO_MIDDLE 5 5 S &END
"
    .trim()
    .to_string();

    let string = part1 + &part2 + &part3;
    format_instructions(string, *instruction_counter)
}

fn go_to_storage(address: u32, next_state: String, mark: bool) -> String {
    let mut string = "".to_string();

    let marknum = if mark {
        symtou8(TapeSymbols::StorageMarker).to_string()
    } else {
        symtou8(TapeSymbols::StorageSeperator).to_string()
    };

    if address == 1 {
        string.push_str(&format!(
            "
# To end of S1
&TO_S1_END StSep {} L &{}",
            marknum, next_state,
        ));
    } else {
        string.push_str(
            "
# To end of S1
&TO_S1_END StSep StSep L &TO_S2_END",
        );

        for i in 2..=address {
            let i_str = format!("S{}", i.to_string());
            let next_i_str = format!("S{}", (i + 1).to_string());

            string.push_str(&format!(
                "
# To end of {0}
&TO_{0}_END 0 0 L &TO_{0}_END
&TO_{0}_END 1 1 L &TO_{0}_END",
                i_str
            ));

            if i == address {
                string.push_str(&format!(
                    "\n&TO_{}_END StSep {} L &{}\n",
                    i_str, marknum, next_state
                ));
            } else {
                string.push_str(&format!(
                    "\n&TO_{}_END StSep StSep L &TO_{}_END",
                    i_str, next_i_str
                ));
            }
        }
    }

    string
}

fn load_instructions(
    instruction_counter: &u32,
    address: u32,
    working_space: String,
) -> Vec<String> {
    let mut part1 = "
# Start of loading
&START Middle Middle                    R &TO_[A/B]_END_R
# To end of number [A/B]
&TO_[A/B]_END_R 0 0                     R &TO_[A/B]_END_R
&TO_[A/B]_END_R 1 1                     R &TO_[A/B]_END_R
&TO_[A/B]_END_R StartA StartA           R &TO_[A/B]_END_R"
        .to_string();

    if working_space == "A" {
        part1.push_str("\n&TO_[A/B]_END_R ABsep ABsep   L &SET_TO_7");
    } else if working_space == "B" {
        part1.push_str("\n&TO_[A/B]_END_R ABsep ABsep   R &TO_[A/B]_END_R");
        part1.push_str("\n&TO_[A/B]_END_R EndB EndB     L &SET_TO_7");
    }

    let mut part2 = "
# Move to the start of number [A/B], and set all values to 7
&SET_TO_7 0 MovH0       L &SET_TO_7
&SET_TO_7 1 MovH0       L &SET_TO_7"
        .to_string();

    if working_space == "A" {
        part2.push_str("\n&SET_TO_7 StartA StartA L &TO_MIDDLE_L");
    } else if working_space == "B" {
        part2.push_str("\n&SET_TO_7 ABsep ABsep L &TO_MIDDLE_L");
    }

    let mut part3 = "
# Find the middle
&TO_MIDDLE_L MovH0 MovH0                L &TO_MIDDLE_L
&TO_MIDDLE_L 1 1                        L &TO_MIDDLE_L
&TO_MIDDLE_L 0 0                        L &TO_MIDDLE_L
&TO_MIDDLE_L StartA StartA              L &TO_MIDDLE_L
&TO_MIDDLE_L ABsep ABsep                L &TO_MIDDLE_L
&TO_MIDDLE_L Middle Middle              L &TO_S1_END"
        .trim()
        .to_string()
        + "\n";

    let mut part4 = go_to_storage(address, "COPY_VALUE".to_string(), false)
        .trim()
        .to_string()
        + "\n";

    let mut part5 = "

# Move Left until start of S[a]
# Copy the first non-7/8 value
&COPY_VALUE 0 MovH0         R &MOVE_ZERO
&COPY_VALUE 1 MovH1         R &MOVE_ONE
&COPY_VALUE MovH0 MovH0     L &COPY_VALUE
&COPY_VALUE MovH1 MovH1     L &COPY_VALUE
&COPY_VALUE StSep StSep     R &RESTORE_VALUE"
        .trim()
        .to_string()
        + "\n";

    let mut part6 = "".to_string();

    for i in [0, 1] {
        // bit to move ZERO or ONE
        let mut string = "".to_string();
        let state = if i == 0 { "&MOVE_ZERO" } else { "&MOVE_ONE" };
        string.push_str(&format!("\n# Move a {} to the end of number [A/B]", i));
        string.push_str(&format!(
            "
{0} 0 0             R {0}
{0} 1 1             R {0}
{0} StartA StartA   R {0}",
            state
        ));

        if working_space == "A" {
            string.push_str(&format!(
                "
{0} ABsep ABsep             L {0}_2
{0} EndB EndB               R {0}",
                state
            ));
        } else if working_space == "B" {
            string.push_str(&format!(
                "
{0} ABsep ABsep             R {0}
{0} EndB EndB               L {0}_2",
                state
            ));
        }

        string.push_str(&format!(
            "
{0} Middle Middle           R {0}
{0} StSep StSep             R {0}
{0} MovH0 MovH0             R {0}
{0} MovH1 MovH1             R {0}",
            state
        ));

        string.push_str(&format!(
            "
# Move a {1} to the left until the first 7, and replace it
{0}_2 0 0               L {0}_2
{0}_2 1 1               L {0}_2
{0}_2 MovH0 {1}         S &TO_MIDDLE_L",
            state, i
        ));

        part6.push_str(&string);
    }

    let mut part7 = "
# Restore number [A/B], translate 7 to 0 and 8 to 1
&RESTORE_VALUE MovH0 0              R &RESTORE_VALUE
&RESTORE_VALUE MovH1 1              R &RESTORE_VALUE
&RESTORE_VALUE StSep StSep          S &TO_START
# Go back to the start
&TO_START StSep StSep               R &TO_START
&TO_START 0 0                       R &TO_START
&TO_START 1 1                       R &TO_START
&TO_START Middle Middle             S &END
"
    .trim()
    .to_string()
        + "\n";

    part1 = part1.trim().to_string() + "\n";
    part2 = part2.trim().to_string() + "\n";
    part4 = part4.trim().to_string() + "\n";
    part6 = part6.trim().to_string() + "\n";

    let mut string = part1 + &part2 + &part3 + &part4 + &part5 + &part6 + &part7;

    // replace all " A/B" with " A" or " B" depending on the working space
    string = string.replace("[A/B]", &format!("{}", working_space));

    // replace all "S[A]" with "S[address]"
    string = string.replace("S[a]", &format!("S{}", address));

    format_instructions(string, *instruction_counter)
}

fn addition_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of addition
&START Middle Middle R &FIND_SECOND_END
# Find the end of B
&FIND_SECOND_END 0 0                        R &FIND_SECOND_END
&FIND_SECOND_END 1 1                        R &FIND_SECOND_END
&FIND_SECOND_END StartA StartA              R &FIND_SECOND_END
&FIND_SECOND_END ABsep ABsep                R &FIND_SECOND_END
&FIND_SECOND_END EndB EndB                  L &SUBTRACT_ONE
# Subtract one from B
&SUBTRACT_ONE 0 1                           L &SUBTRACT_ONE
&SUBTRACT_ONE 1 0                           L &FIND_FIRST_END
&SUBTRACT_ONE ABsep ABsep                   L &TO_MIDDLE
# Find the end of the A
&FIND_FIRST_END 0 0                         L &FIND_FIRST_END
&FIND_FIRST_END 1 1                         L &FIND_FIRST_END
&FIND_FIRST_END ABsep ABsep                 L &ADD_ONE
# Add one to A
&ADD_ONE 0 1                                L &FIND_SECOND_END
&ADD_ONE 1 0                                L &ADD_ONE
&ADD_ONE ABsep ABsep                        L &TO_MIDDLE
# Return to the middle of the tape
&TO_MIDDLE 0 0                              L &TO_MIDDLE
&TO_MIDDLE 1 1                              L &TO_MIDDLE
&TO_MIDDLE StartA StartA                    L &TO_MIDDLE
&TO_MIDDLE Middle Middle                    S &END
";

    format_instructions(string.to_string(), *instruction_counter)
}

fn format_instructions(instructions: String, instruction_counter: u32) -> Vec<String> {
    let mut string = instructions.clone();
    string = string
        .replace("&", &instruction_counter.to_string())
        .replace("StartA", &symtou8(TapeSymbols::StartA).to_string())
        .replace("ABsep", &symtou8(TapeSymbols::ABseperator).to_string())
        .replace("EndB", &symtou8(TapeSymbols::EndB).to_string())
        .replace("Middle", &symtou8(TapeSymbols::Middle).to_string())
        .replace("StSep", &symtou8(TapeSymbols::StorageSeperator).to_string())
        .replace("StMark", &symtou8(TapeSymbols::StorageMarker).to_string())
        .replace("MovH0", &symtou8(TapeSymbols::HasMovedHelper0).to_string())
        .replace("MovH1", &symtou8(TapeSymbols::HasMovedHelper1).to_string())
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
