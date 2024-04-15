use super::prelude::*;

pub fn mul_instructions(instruction_counter: &u32) -> Vec<String> {
    let string = "
# Start of the binary multiplication
START Middle * R CLEAR_A

# Clear A
CLEAR_A 1 0             R CLEAR_A
CLEAR_A (0,StartA) *    R CLEAR_A
CLEAR_A ABsep *         R FIND_B_END

# Find the end of the B
FIND_B_END (0,1,StartA,ABsep,H0,H1) * R FIND_B_END
FIND_B_END EndB *                           L CHECK_B

# Check last digit of B
# If last digit is 0, multiply C by 2
# If last digit is 1, Add C to A and then multiply C by 2
CHECK_B H2 *  L CHECK_B
CHECK_B 0 H2  R SHIFT_C
CHECK_B 1 H2  R ADD_C_TO_A_0
CHECK_B ABsep * R CLEAR_B

## Multiply C by shifting each bit to the left

# First go to end of C
SHIFT_C (0,1,H2,EndB) *   R SHIFT_C
SHIFT_C EndC *              L SHIFT_C_ZERO

# Place a 0 and shift left
SHIFT_C_ZERO 0 *    L SHIFT_C_ZERO
SHIFT_C_ZERO 1 0    L SHIFT_C_ONE
SHIFT_C_ZERO EndB * L CHECK_B

# Place a 1 and shift left
SHIFT_C_ONE 0 1     L SHIFT_C_ZERO
SHIFT_C_ONE 1 *     L SHIFT_C_ONE
SHIFT_C_ONE EndB *  L CHECK_B

## Add C to A

# First go to end of C
ADD_C_TO_A_0 (0,1,ABsep,EndB,H0,H1,H2) *    R ADD_C_TO_A_0
ADD_C_TO_A_0 EndC *                                 L ADD_C_TO_A_1

# Find first non-moved digit of C, and replace 0 with H0 and 1 with H1
ADD_C_TO_A_1 0 H0            L ADD_C_TO_A_ZERO
ADD_C_TO_A_1 1 H1            L ADD_C_TO_A_ONE
ADD_C_TO_A_1 (H0,H1) *    L ADD_C_TO_A_1
ADD_C_TO_A_1 EndB *             R ADD_C_TO_A_RESTORE_0

# Move a 0 from C to end of A
ADD_C_TO_A_ZERO (0,1,EndB,H0,H1,H2) *   L ADD_C_TO_A_ZERO
ADD_C_TO_A_ZERO ABsep *                         L ADD_C_TO_A_ZERO_1

# Set the 0 in A to H0, except if its a 1
ADD_C_TO_A_ZERO_1 H0 *   L ADD_C_TO_A_ZERO_1
ADD_C_TO_A_ZERO_1 H1 *   L ADD_C_TO_A_ZERO_1
ADD_C_TO_A_ZERO_1 0 H0   R ADD_C_TO_A_0
ADD_C_TO_A_ZERO_1 1 H1   R ADD_C_TO_A_0

# Move a 1 from C to end of A
ADD_C_TO_A_ONE (0,1,EndB,H0,H1,H2) *    L ADD_C_TO_A_ONE
ADD_C_TO_A_ONE ABsep *                          L ADD_C_TO_A_ONE_1

# Set the 1 in A to H1, unless if its already a 1
ADD_C_TO_A_ONE_1 H0 *    L ADD_C_TO_A_ONE_1
ADD_C_TO_A_ONE_1 H1 *    L ADD_C_TO_A_ONE_1
ADD_C_TO_A_ONE_1 0 H1    R ADD_C_TO_A_0
ADD_C_TO_A_ONE_1 1 H0    L ADD_C_TO_A_ONE_1_MOVE

# Move the 1 to the next digit, if its a 1, make it 0 and move to the next digit
ADD_C_TO_A_ONE_1_MOVE 0 1   R ADD_C_TO_A_0
ADD_C_TO_A_ONE_1_MOVE 1 0   L ADD_C_TO_A_ONE_1_MOVE

# Restore the number in C
ADD_C_TO_A_RESTORE_0 H0 0    R ADD_C_TO_A_RESTORE_0
ADD_C_TO_A_RESTORE_0 H1 1    R ADD_C_TO_A_RESTORE_0
ADD_C_TO_A_RESTORE_0 EndC  *    L ADD_C_TO_A_RESTORE_1

# Move to the start of A
ADD_C_TO_A_RESTORE_1 (0,1,ABsep,EndB,H0,H1,H2) *    L ADD_C_TO_A_RESTORE_1
ADD_C_TO_A_RESTORE_1 StartA *                               R ADD_C_TO_A_RESTORE_2

# Restore the number in A, and move to the end of B
ADD_C_TO_A_RESTORE_2 (0,1,H2,ABsep) *     R ADD_C_TO_A_RESTORE_2
ADD_C_TO_A_RESTORE_2 H0 0                    R ADD_C_TO_A_RESTORE_2
ADD_C_TO_A_RESTORE_2 H1 1                    R ADD_C_TO_A_RESTORE_2
ADD_C_TO_A_RESTORE_2 EndB *                     L SHIFT_C

# Clear B, set all to 0
CLEAR_B H2 0   R CLEAR_B
CLEAR_B EndB *   L RETURN

# Return to the start of the program
RETURN (0,1,StartA,ABsep) *    L RETURN
RETURN Middle *                 S END"
        .to_string();

    format_instructions(string, *instruction_counter)
}
