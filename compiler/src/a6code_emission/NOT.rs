use super::prelude::*;

pub fn not_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of NOT program
START 5 5           R FIND_A_END

# Find end of A
FIND_A_END (0,1,StartA) *   R FIND_A_END
FIND_A_END ABsep *          L FLIP_DIGIT

# Flip the last digit of A
FLIP_DIGIT 0 1      L RETURN
FLIP_DIGIT 1 0      L RETURN

# Return to the middle of the tape
RETURN (0,1,StartA) *   L RETURN
RETURN 5 *              S END
";

    format_instructions(string.to_string(), *instruction_counter)
}
