use super::prelude::*;

pub fn sub_safe_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of the subtraction program
START 5 5 R FIND_B_END

# Find the end of the second number
FIND_B_END (0,1,2,3,7,8) *  R FIND_B_END
FIND_B_END 4 *              L SUB_DIGIT

# Find first non-subtracted digit
SUB_DIGIT (H0,H1) *   L SUB_DIGIT
SUB_DIGIT 0 H0           L SUB_DIGIT_ZERO
SUB_DIGIT 1 H0           L SUB_DIGIT_ONE
SUB_DIGIT ABsep *           R RESTORE_1

# Subtract a zero
SUB_DIGIT_ZERO (0,1) *      L SUB_DIGIT_ZERO
SUB_DIGIT_ZERO ABsep *      L SUB_DIGIT_ZERO_1

SUB_DIGIT_ZERO_1 (H0,H1) *    L SUB_DIGIT_ZERO_1
SUB_DIGIT_ZERO_1 0 H0            L FIND_B_END
SUB_DIGIT_ZERO_1 1 H1            R FIND_B_END

# Subtract a one
SUB_DIGIT_ONE (0,1) *   L SUB_DIGIT_ONE
SUB_DIGIT_ONE ABsep *   L SUB_DIGIT_ONE_1

SUB_DIGIT_ONE_1 (H0,H1) * L SUB_DIGIT_ONE_1
SUB_DIGIT_ONE_1 0 H1         L SUB_DIGIT_ONE_2
SUB_DIGIT_ONE_1 1 H0         R FIND_B_END

SUB_DIGIT_ONE_2 1 0         R FIND_B_END
SUB_DIGIT_ONE_2 2 *         R RESTORE
SUB_DIGIT_ONE_2 0 1         L SUB_DIGIT_ONE_2

# Return to the middle of the tape
RESTORE (0,1,H0,H1) 0     R RESTORE
RESTORE ABsep *                 R RESTORE_1

RESTORE_1 (0,1,H0,H1) *   R RESTORE_1
RESTORE_1 EndB *                L RESTORE_2

RESTORE_2 (0,1,StartA,ABsep) *  L RESTORE_2
RESTORE_2 H0 0               L RESTORE_2
RESTORE_2 H1 1               L RESTORE_2
RESTORE_2 Middle *              S END";

    format_instructions(string.to_string(), *instruction_counter)
}
