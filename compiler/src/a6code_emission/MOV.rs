use super::prelude::*;

pub fn mov_instruction(instruction_counter: &u32, label: String) -> Vec<String> {
    let string = format!(
        "
# Start of the JUMP if NOT ZERO program
START 5 5 R START
START 2 2 R A_END

# Go to end of A
A_END (0,1) * R A_END
A_END 3 * L CHECK_A

# no ones found in A, set last digit of A to 1
CHECK_A 0 1 S RETURN_ZERO
CHECK_A 1 1 S RETURN_ONE

# Return to Middle
RETURN_ZERO (0,1,2) * L RETURN_ZERO
RETURN_ZERO 5 5 S END

RETURN_ONE (0,1,2) * L RETURN_ONE
RETURN_ONE 5 5 S !LABEL_{label}\n"
    )
    .trim()
    .to_string();

    format_instructions(string, *instruction_counter)
}
