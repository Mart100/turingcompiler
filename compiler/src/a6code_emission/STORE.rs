use super::prelude::*;

pub fn store_instructions(
    instruction_counter: &u32,
    address: u32,
    working_space: String,
) -> Vec<String> {
    let part1 = "\nSTART 5 5 L TO_S1_END\n".to_string();

    let part2 = go_to_storage(address, "SET_TO_7".to_string())
        .trim()
        .to_string()
        + "\n";

    let part3 = "
# Set all the values to MovH0 untill the start of S1
SET_TO_7 0 MovH0                            L SET_TO_7
SET_TO_7 1 MovH0                            L SET_TO_7
SET_TO_7 StSep StMark                       R TO_MIDDLE_R
# Find the middle
TO_MIDDLE_R (1,0,MovH0,StartA,StSep) *      R TO_MIDDLE_R
TO_MIDDLE_R Middle Middle                   R COPY_VALUE
# Move Left until start of A
# Copy the first non-MovH0/MovH8 value
COPY_VALUE 0 MovH0                          L MOVE_ZERO
COPY_VALUE 1 MovH1                          L MOVE_ONE
COPY_VALUE (StartA,MovH0,MovH1) *           R COPY_VALUE
COPY_VALUE ABsep *                          L RESTORE_VALUE"
        .to_string()
        + "\n";

    let part4 = move_bits("StMark", "L", "TO_MIDDLE_R") + "\n";

    let part5 = "
# Restore number A, translate MovH0 to 0 and MovH1 to 1
RESTORE_VALUE MovH0 0                       L RESTORE_VALUE
RESTORE_VALUE MovH1 1                       L RESTORE_VALUE
RESTORE_VALUE StartA *                      L RESET_STORAGE_MARKER
# Reset StMark to StSep
RESET_STORAGE_MARKER (0,1,Middle,StSep) *   L RESET_STORAGE_MARKER
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
