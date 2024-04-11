use std::collections::HashMap;

use crate::TACInstruction;

// Optimize the TAC instructions
pub fn optimize_tac(tac: Vec<TACInstruction>) -> Vec<TACInstruction> {
    let mut optimized_tac = Vec::<TACInstruction>::new();
    let mut copy_map = HashMap::<String, String>::new();

    for instruction in tac {
        match instruction {
            TACInstruction::Assignment { var_name, value } => {
                if value.starts_with('t') {
                    copy_map.insert(var_name, value);
                } else {
                    optimized_tac.push(TACInstruction::Assignment { var_name, value });
                }
            }
            TACInstruction::BinaryOperation {
                result,
                left,
                operator,
                right,
            } => {
                let arg1 = copy_map.get(&left).unwrap_or(&left).clone();
                let arg2 = copy_map.get(&right).unwrap_or(&right).clone();

                optimized_tac.push(TACInstruction::BinaryOperation {
                    result,
                    left: arg1,
                    operator,
                    right: arg2,
                });
            }
            TACInstruction::Label { label } => {
                optimized_tac.push(TACInstruction::Label { label });
            }
            TACInstruction::Goto { label } => {
                optimized_tac.push(TACInstruction::Goto { label });
            }
            TACInstruction::IfGoto { condition, label } => {
                let arg1 = copy_map.get(&condition).unwrap_or(&condition).clone();
                optimized_tac.push(TACInstruction::IfGoto {
                    condition: arg1,
                    label,
                });
            }
        }
    }

    // Remove gaps in the temporary variable numbering, and in label numbering
    let mut var_counter = 1;
    let mut temp_var_map = std::collections::HashMap::new();
    let mut label_counter = 1;
    let mut temp_label_map = std::collections::HashMap::new();
    for instruction in &mut optimized_tac {
        match instruction {
            TACInstruction::IfGoto { condition, label } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, condition);
                update_label(&mut temp_label_map, &mut label_counter, label);
            }
            TACInstruction::Goto { label } => {
                update_label(&mut temp_label_map, &mut label_counter, label);
            }
            TACInstruction::Label { label } => {
                update_label(&mut temp_label_map, &mut label_counter, label);
            }
            TACInstruction::Assignment { var_name, value } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, var_name);
                update_temp_var(&mut temp_var_map, &mut var_counter, value);
            }
            TACInstruction::BinaryOperation {
                result,
                left,
                operator,
                right,
            } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, result);
                update_temp_var(&mut temp_var_map, &mut var_counter, left);
                update_temp_var(&mut temp_var_map, &mut var_counter, right);
            }
            _ => {}
        }
    }

    optimized_tac
}

fn update_label(temp_map: &mut HashMap<String, String>, counter: &mut usize, label: &mut String) {
    let entry = temp_map.entry(label.clone()).or_insert_with(|| {
        let new_name = format!("L{}", *counter);
        *counter += 1;
        new_name
    });
    *label = entry.clone();
}

fn update_temp_var(temp_map: &mut HashMap<String, String>, counter: &mut usize, var: &mut String) {
    if is_temporary(var) {
        let entry = temp_map.entry(var.clone()).or_insert_with(|| {
            let new_name = format!("t{}", *counter);
            *counter += 1;
            new_name
        });
        *var = entry.clone();
    }
}

fn is_temporary(var: &str) -> bool {
    var.starts_with('t') && var[1..].chars().all(char::is_numeric)
}
