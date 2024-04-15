use super::prelude::*;

pub fn endfn_instructions(
    instruction_counter: &u32,
    name: String,
    address: u32,
    total: u8,
) -> Vec<String> {
    let to_sn = go_to_storage(address, "SUB_0".to_string(), false);

    let mut to_labels = "".to_string();

    for i in 0..=total {
        let next_i = i + 1;
        let label = format!("LABEL_L{name}_{i}");
        to_labels.push_str(&format!(
            "\n
SUB_{i} 0 * L SUB_{i}
SUB_{i} 1 0 R GO_BACK_{i}
SUB_{i} StSep * R TO_MID_{i}

GO_BACK_{i} 0 1 R GO_BACK_{i}
GO_BACK_{i} 1 * R GO_BACK_{i}
GO_BACK_{i} StSep * L SUB_{next_i}

TO_MID_{i} (0,1,StSep) * R TO_MID_{i}
TO_MID_{i} Middle Middle S !{label}
        "
        ));
    }

    let string = format!(
        "
START 5 5 L TO_{address}_S1

{to_sn}

{to_labels}
    ",
    );

    format_instructions(string, *instruction_counter)
}
