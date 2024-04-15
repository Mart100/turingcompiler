use super::prelude::*;

pub fn store_instructions(
    instruction_counter: &u32,
    address: u32,
    working_space: String,
) -> Vec<String> {
    let part1 = format!("\nSTART 5 5 L TO_{address}_S1\n");

    let part2 = go_to_storage(address, "SET_TO_7".to_string(), true)
        .trim()
        .to_string()
        + "\n";

    let part3 = "
# Set all the values to H0 untill the start of S1
SET_TO_7 0 H0                            L SET_TO_7
SET_TO_7 1 H0                            L SET_TO_7
SET_TO_7 StSep *                     R TO_MIDDLE_R
# Find the middle
TO_MIDDLE_R (1,0,H0,StartA,StSep,StMark) *      R TO_MIDDLE_R
TO_MIDDLE_R Middle Middle                       R A_END

# to end of A
A_END (0,1,StartA,H0,H1) *                        R A_END
A_END ABsep *                               L COPY_VALUE

# Move Left until end of A
# Copy the first non-H0/MovH8 value
COPY_VALUE 0 H0                             L MOVE_ZERO
COPY_VALUE 1 H1                             L MOVE_ONE
COPY_VALUE (EndA,H0,H1) *                   L COPY_VALUE
COPY_VALUE StartA *                         R RESTORE_VALUE"
        .to_string()
        + "\n";

    let part4 = move_bits("StMark", "L", "TO_MIDDLE_R") + "\n";

    let part5 = "
# Restore number A, translate H0 to 0 and H1 to 1
RESTORE_VALUE H0 0                          R RESTORE_VALUE
RESTORE_VALUE H1 1                          R RESTORE_VALUE
RESTORE_VALUE ABsep *                      L RESET_STORAGE_MARKER
# Reset StMark to StSep
RESET_STORAGE_MARKER (0,1,Middle,StSep,StartA) *   L RESET_STORAGE_MARKER
RESET_STORAGE_MARKER StMark StSep           R RETURN_TO_MIDDLE
# Return to middle
RETURN_TO_MIDDLE (0,1,StSep) *              R RETURN_TO_MIDDLE
RETURN_TO_MIDDLE Middle Middle              S END
"
    .trim()
    .to_string();

    let string = part1 + &part2 + &part3 + &part4 + &part5;
    format_instructions(string, *instruction_counter)
}
