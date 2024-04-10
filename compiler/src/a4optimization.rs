use std::collections::HashMap;

use crate::TACInstruction;

// Optimize the TAC instructions
pub fn optimize_tac(tac: Vec<TACInstruction>) -> Vec<TACInstruction> {
    let mut optimized_tac = Vec::<TACInstruction>::new();
    let mut copy_map = HashMap::<String, String>::new();

    for instruction in tac {
        if instruction.operator.as_deref() == Some("=") && instruction.right.is_none() {
            // If the instruction is a copy, store the mapping from the result to the argument
            copy_map.insert(instruction.result.clone(), instruction.left.clone());
        } else {
            // If the instruction is not a copy, replace any copied variables with their original variables
            let arg1 = copy_map
                .get(&instruction.left)
                .unwrap_or(&instruction.left)
                .clone();
            let arg2 = instruction
                .right
                .as_ref()
                .and_then(|arg2| copy_map.get(arg2))
                .or_else(|| instruction.right.as_ref());

            optimized_tac.push(TACInstruction::new(
                instruction.result.clone(),
                arg1,
                instruction.operator.clone(),
                arg2.cloned(),
            ));
        }
    }

    // Remove gaps in the temporary variable numbering
    let mut counter = 1;
    let mut temp_map = std::collections::HashMap::new();
    for instruction in &mut optimized_tac {
        update_temp_var(&mut temp_map, &mut counter, &mut instruction.result);
        update_temp_var(&mut temp_map, &mut counter, &mut instruction.left);

        if let Some(right) = instruction.right.as_mut() {
            update_temp_var(&mut temp_map, &mut counter, right);
        }
    }

    optimized_tac
}

fn update_temp_var(
    temp_map: &mut std::collections::HashMap<String, String>,
    counter: &mut usize,
    var: &mut String,
) {
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
