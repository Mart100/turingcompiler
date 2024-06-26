use std::collections::HashMap;

use crate::TACInstruction;

// Optimize the TAC instructions
pub fn optimize_tac(tac: Vec<TACInstruction>) -> Vec<TACInstruction> {
    let mut optimized_tac = Vec::<TACInstruction>::new();

    let mut variables = HashMap::<String, String>::new();

    for instruction in &tac {
        if let TACInstruction::Assignment { var_name, value } = instruction {
            if !is_temporary(var_name) && value.parse::<u8>().is_err() && value != "ret" {
                variables.insert(value.to_string(), var_name.to_string());
            }
        }
    }

    println!("Variables: {:?}", variables);

    let mut in_function_args = false;
    for instruction in tac {
        match instruction {
            TACInstruction::Assignment {
                ref var_name,
                ref value,
            } => {
                if !var_name.starts_with("arg") {
                    in_function_args = false;
                }
                println!("variables: {:?}", variables);
                println!("var_name: {}, value: {}", var_name, value);
                if variables.get(value) == Some(var_name) {
                    println!("Remove: {} = {}", var_name, value);
                    if !var_name.starts_with("arg") || in_function_args || is_temporary(value) {
                        continue;
                    }
                } else if let Some(new_var_name) = variables.get(var_name) {
                    if is_temporary(var_name) {
                        println!("Replace {} with {}", var_name, new_var_name);
                        optimized_tac.push(TACInstruction::Assignment {
                            var_name: new_var_name.clone(),
                            value: value.clone(),
                        });
                        continue;
                    }
                }

                println!("Keep");
                optimized_tac.push(instruction.clone());
            }
            TACInstruction::Function { .. } => {
                in_function_args = true;
                optimized_tac.push(instruction);
            }
            TACInstruction::BinaryOperation {
                result,
                left,
                operator,
                right,
            } => {
                let left = variables.get(&left).cloned().unwrap_or(left.clone());
                let right = variables.get(&right).cloned().unwrap_or(right.clone());
                let result = variables.get(&result).cloned().unwrap_or(result.clone());
                optimized_tac.push(TACInstruction::BinaryOperation {
                    result,
                    left,
                    operator,
                    right,
                });

                in_function_args = false;
            }
            _ => {
                in_function_args = false;
                optimized_tac.push(instruction);
            }
        }
    }

    // reset_vars(&mut optimized_tac);

    optimized_tac
}

// Remove gaps in the temporary variable numbering, and in label numbering
fn reset_vars(tac: &mut Vec<TACInstruction>) {
    let mut var_counter = 1;
    let mut temp_var_map = std::collections::HashMap::new();
    let mut label_counter = 1;
    let mut temp_label_map = std::collections::HashMap::new();
    let mut function_counter = 1;
    let mut temp_function_map = std::collections::HashMap::new();
    for instruction in tac {
        match instruction {
            TACInstruction::IfGoto { condition, label } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, condition);
                update_label(&mut temp_label_map, &mut label_counter, label);
            }
            TACInstruction::IfNotGoto { condition, label } => {
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
            TACInstruction::Return { value } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, value);
            }
            TACInstruction::FunctionCall { name, args } => {
                update_function(&mut temp_function_map, &mut function_counter, name);
                for arg in args {
                    update_temp_var(&mut temp_var_map, &mut var_counter, arg);
                }
            }
            TACInstruction::Function { name } => {
                update_function(&mut temp_function_map, &mut function_counter, name);
            }
            TACInstruction::BinaryOperation {
                result,
                left,
                operator: _,
                right,
            } => {
                update_temp_var(&mut temp_var_map, &mut var_counter, result);
                update_temp_var(&mut temp_var_map, &mut var_counter, left);
                update_temp_var(&mut temp_var_map, &mut var_counter, right);
            }
        }
    }
}

fn update_function(temp_map: &mut HashMap<String, String>, counter: &mut usize, func: &mut String) {
    let entry = temp_map.entry(func.clone()).or_insert_with(|| {
        let new_name = format!("F{}", *counter);
        *counter += 1;
        new_name
    });
    *func = entry.clone();
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
    if var.parse::<u8>().is_err() {
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
