use super::prelude::*;

pub fn iszero_instruction(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of the ISZERO program
START 5 5 R START
START 2 2 R CHECK_A

# Check for ones in A
CHECK_A 0 * R CHECK_A
CHECK_A 1 0 R FOUND_ONE
CHECK_A 3 * L CHECK_A_1

# no ones found in A, set last digit of A to 1
CHECK_A_1 0 1 L RETURN

# Found a 1 in A
FOUND_ONE 0 * R FOUND_ONE
FOUND_ONE 1 0 R FOUND_ONE
FOUND_ONE 3 * L FOUND_ONE_1

# Found a 1 in A, return
FOUND_ONE_1 0 0 L RETURN

# Return to middle
RETURN (0,1,2) * L RETURN
RETURN 5 5 S END"
        .to_string();

    format_instructions(string, *instruction_counter)
}
