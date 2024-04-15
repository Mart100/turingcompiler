use super::prelude::*;

pub fn move_instruction(instruction_counter: &u32, source: u32, destination: u32) -> Vec<String> {
    let dir;
    println!("source: {}, destination: {}", source, destination);
    if source > destination {
        dir = "R";
    } else {
        dir = "L"
    }

    let to_des = go_to_storage(destination, "SET_TO_H0".to_string(), true);
    let to_source = go_to_storage(source, "COPY_VALUE".to_string(), false);

    let move_bits = move_bits("StMark", dir, "TO_MIDDLE_R");

    let string = format!(
        "
START 5 5 L TO_{destination}_S1

# Go to destination Sn
{to_des}

# Set all the values to H0 untill the start of Sn, and Mark end of Sn with StMark
SET_TO_H0 (0,1) H0                          L SET_TO_H0
SET_TO_H0 StSep *                           R TO_MIDDLE_R

# Find the middle
TO_MIDDLE_R (1,0,H0,H1,StSep,StMark) *         R TO_MIDDLE_R
TO_MIDDLE_R Middle Middle                   L TO_{source}_S1

# Go to source Sn
{to_source}

# Move Left until start of A
# Copy the first non-H0/MovH8 value
COPY_VALUE 0 H0                             S MOVE_ZERO
COPY_VALUE 1 H1                             S MOVE_ONE
COPY_VALUE (StartA,H0,H1) *                 L COPY_VALUE
COPY_VALUE (StSep,StMark) *                 R RESTORE_VALUE

{move_bits}

# Restore source, translate H0 to 0 and H1 to 1
RESTORE_VALUE H0 0                          R RESTORE_VALUE
RESTORE_VALUE H1 1                          R RESTORE_VALUE
RESTORE_VALUE StSep *                       L RESET_STORAGE_MARKER
# Reset StMark to StSep
RESET_STORAGE_MARKER (0,1,StSep) *      {dir} RESET_STORAGE_MARKER
RESET_STORAGE_MARKER StMark StSep           S RETURN_TO_MIDDLE
# Return to middle
RETURN_TO_MIDDLE (0,1,StSep) *              R RETURN_TO_MIDDLE
RETURN_TO_MIDDLE Middle Middle              S END


"
    );
    format_instructions(string, *instruction_counter)
}
