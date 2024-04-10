use super::prelude::*;

pub fn load_instructions(
    instruction_counter: &u32,
    address: u32,
    working_space: String,
) -> Vec<String> {
    let mut part1 = "
# Start of loading
START Middle *                          R TO_[A/B/C]_END_R
# To end of number [A/B/C]
TO_[A/B/C]_END_R (0,1,StartA) *           R TO_[A/B/C]_END_R"
        .to_string();

    if working_space == "A" {
        part1.push_str("\nTO_[A/B/C]_END_R ABsep *   L SET_TO_7\n");
    } else if working_space == "B" {
        part1.push_str("\nTO_[A/B/C]_END_R ABsep *   R TO_[A/B/C]_END_R");
        part1.push_str("\nTO_[A/B/C]_END_R EndB *    L SET_TO_7\n");
    } else if working_space == "C" {
        part1.push_str("\nTO_[A/B/C]_END_R (ABsep,EndB) *       R TO_[A/B/C]_END_R");
        part1.push_str("\nTO_[A/B/C]_END_R EndC *               L SET_TO_7\n");
    }

    let mut part2 = "
# Move to the start of number [A/B/C], and set all values to 7
SET_TO_7 0 MovH0       L SET_TO_7
SET_TO_7 1 MovH0       L SET_TO_7"
        .to_string();

    if working_space == "A" {
        part2.push_str("\nSET_TO_7 StartA * L TO_MIDDLE_L\n");
    } else if working_space == "B" {
        part2.push_str("\nSET_TO_7 ABsep * L TO_MIDDLE_L\n");
    } else if working_space == "C" {
        part2.push_str("\nSET_TO_7 EndB * L TO_MIDDLE_L\n");
    }

    let part3 = "
# Find the middle
TO_MIDDLE_L (1,0,MovH0,StartA,ABsep,EndB) * L TO_MIDDLE_L
TO_MIDDLE_L Middle Middle              L TO_S1_END"
        .trim()
        .to_string()
        + "\n";

    let part4 = go_to_storage(address, "COPY_VALUE".to_string())
        .trim()
        .to_string()
        + "\n";

    let part5 = "
# Move Left until start of S[a]
# Copy the first non-7/8 value
COPY_VALUE 0 MovH0         R MOVE_ZERO
COPY_VALUE 1 MovH1         R MOVE_ONE
COPY_VALUE (MovH0,MovH1) * L COPY_VALUE
COPY_VALUE StSep *         R RESTORE_VALUE"
        .trim()
        .to_string()
        + "\n";

    let part6 = move_bits(&working_space, "R", "TO_MIDDLE_L") + "\n";

    let part7 = "
# Restore number [A/B/C], translate 7 to 0 and 8 to 1
RESTORE_VALUE MovH0 0              R RESTORE_VALUE
RESTORE_VALUE MovH1 1              R RESTORE_VALUE
RESTORE_VALUE StSep *              S TO_START
# Go back to the start
TO_START (0,1,StSep) *             R TO_START
TO_START Middle *                  S END"
        .trim()
        .to_string()
        + "\n";

    let mut string = part1 + &part2 + &part3 + &part4 + &part5 + &part6 + &part7;

    // replace all "[A/B/C]" with A,B or C depending on the working space
    string = string.replace("[A/B/C]", &format!("{}", working_space));

    // replace all "S[A]" with "S[address]"
    string = string.replace("S[a]", &format!("S{}", address));

    format_instructions(string, *instruction_counter)
}
