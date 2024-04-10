use super::prelude::*;

pub fn add_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of the addition program
START 5 * R FIND_B_END

# Find the end of the second number
FIND_B_END (0,1,2,3,7,8) * R FIND_B_END
FIND_B_END 4 * L ADD_DIGIT

# Find first non-added digit
ADD_DIGIT (7,8) *   L ADD_DIGIT
ADD_DIGIT 0 7       L ADD_DIGIT_ZERO
ADD_DIGIT 1 7       L ADD_DIGIT_ONE
ADD_DIGIT 3 *       R RESTORE

# Add a zero
ADD_DIGIT_ZERO (0,1) *      L ADD_DIGIT_ZERO
ADD_DIGIT_ZERO 3 *          L ADD_DIGIT_ZERO_1
ADD_DIGIT_ZERO_1 (7,8) *    L ADD_DIGIT_ZERO_1
ADD_DIGIT_ZERO_1 0 7        L FIND_B_END
ADD_DIGIT_ZERO_1 1 8        L FIND_B_END

# Add a one
ADD_DIGIT_ONE (0,1) *   L ADD_DIGIT_ONE
ADD_DIGIT_ONE 3 3       L ADD_DIGIT_ONE_1
ADD_DIGIT_ONE_1 (7,8) * L ADD_DIGIT_ONE_1
ADD_DIGIT_ONE_1 0 8     L FIND_B_END
ADD_DIGIT_ONE_1 1 7     L ADD_DIGIT_ONE_2

ADD_DIGIT_ONE_2 0 1     R FIND_B_END
ADD_DIGIT_ONE_2 1 0     L ADD_DIGIT_ONE_2

# Return to the middle of the tape
RESTORE 7 0             R RESTORE
RESTORE 8 1             R RESTORE
RESTORE 4 *             L RESTORE_1

RESTORE_1 (0,1,2,3) *   L RESTORE_1
RESTORE_1 7 0           L RESTORE_1
RESTORE_1 8 1           L RESTORE_1
RESTORE_1 5 *           S END
";

    format_instructions(string.to_string(), *instruction_counter)
}
