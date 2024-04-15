use super::prelude::*;

pub fn set_instructions(instruction_counter: &u32, address: u32, value: [bool; 8]) -> Vec<String> {
    let part1 = format!("\nSTART 5 5 L TO_{address}_S1\n");

    let part2 = go_to_storage(address, "RESET_S".to_string(), false)
        .trim()
        .to_string()
        + "\n";

    let part3 = "
# Reset to 0
RESET_S 0 * L RESET_S
RESET_S 1 0 L RESET_S
RESET_S 6 * R SET_1D\n";

    let mut part4 = "".to_string();

    for i in 0..8 {
        let state1 = format!("SET_{}D", i + 1);
        let mut state2 = format!("SET_{}D", i + 2);

        if i == 7 {
            state2 = "RETURN".to_string();
        }
        if value[i] {
            part4.push_str(&format!("{state1} 0 1 R {state2}\n"));
        } else {
            part4.push_str(&format!("{state1} 0 0 R {state2}\n"));
        }
    }
    part4 = part4.trim().to_string() + "\n";

    let part5 = "
# Return to middle
RETURN (0,1,StSep) *    R RETURN
RETURN Middle *         S END"
        .to_string();

    let string = part1 + &part2 + &part3 + &part4 + &part5;

    format_instructions(string, *instruction_counter)
}
