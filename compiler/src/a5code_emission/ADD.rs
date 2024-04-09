use super::prelude::*;

pub fn add_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of addition
START Middle Middle                         R FIND_SECOND_END
# Find the end of B
FIND_SECOND_END (0,1,StartA,ABsep) *        R FIND_SECOND_END
FIND_SECOND_END EndB EndB                   L SUBTRACT_ONE
# Subtract one from B
SUBTRACT_ONE 0 1                            L SUBTRACT_ONE
SUBTRACT_ONE 1 0                            L FIND_FIRST_END
SUBTRACT_ONE ABsep *                        L TO_MIDDLE
# Find the end of the A
FIND_FIRST_END (0,1) *                      L FIND_FIRST_END
FIND_FIRST_END ABsep *                      L ADD_ONE
# Add one to A
ADD_ONE 0 1                                 L FIND_SECOND_END
ADD_ONE 1 0                                 L ADD_ONE
ADD_ONE ABsep *                             L TO_MIDDLE
# Return to the middle of the tape
TO_MIDDLE (0,1,StartA) *                    L TO_MIDDLE
TO_MIDDLE Middle *                          S END
";

    format_instructions(string.to_string(), *instruction_counter)
}
