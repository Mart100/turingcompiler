use std::collections::HashMap;

use crate::a2parser::AstNode;

#[derive(Debug, Clone)]
pub struct TACInstruction {
    pub result: String,
    pub left: String,
    pub operator: Option<String>,
    pub right: Option<String>,
}

impl TACInstruction {
    pub fn new(
        result: String,
        left: String,
        operator: Option<String>,
        right: Option<String>,
    ) -> Self {
        TACInstruction {
            result,
            left,
            operator,
            right,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.operator {
            Some(op) => {
                if let Some(right) = &self.right {
                    format!("{} = {} {} {}", self.result, self.left, op, right)
                } else {
                    format!("{} = {}", self.result, self.left)
                }
            }
            None => format!("{} = {}", self.result, self.left),
        }
    }
}

pub fn tac_generator(ast: &Vec<AstNode>) -> Vec<TACInstruction> {
    let mut instructions = Vec::new();
    let mut temp_counter = 1;
    let mut var_map = HashMap::new();

    for node in ast {
        generate_tac(node, &mut instructions, &mut temp_counter, &mut var_map);
    }

    instructions
}

fn generate_tac(
    node: &AstNode,
    instructions: &mut Vec<TACInstruction>,
    temp_counter: &mut u32,
    var_map: &mut HashMap<String, String>,
) -> String {
    // println!("{:?}", node);
    // if the node is an assignment
    if node.operator.as_deref() == Some("=") && node.left.is_none() {
        if let Some(var_name) = &node.var_name {
            if let Some(right) = &node.right {
                let right_tac = generate_tac(&*right, instructions, temp_counter, var_map);

                let result = format!("t{}", *temp_counter);
                *temp_counter += 1;
                var_map.insert(var_name.clone(), result.clone());

                // println!("! {:?}", var_map);

                instructions.push(TACInstruction::new(
                    result.clone(),
                    right_tac,
                    Some("=".to_string()),
                    None,
                ));
                return result;
            }
        }
    }

    // if the node is an operator
    if let Some(left) = &node.left {
        let left_tac = generate_tac(&*left, instructions, temp_counter, var_map);
        if let Some(right) = &node.right {
            let right_tac = generate_tac(&*right, instructions, temp_counter, var_map);
            if let Some(op) = &node.operator {
                let result = format!("t{}", *temp_counter);
                *temp_counter += 1;
                instructions.push(TACInstruction::new(
                    result.clone(),
                    left_tac,
                    Some(op.clone()),
                    Some(right_tac),
                ));
                return result;
            }
        }
    }

    // if the node is a value
    if let Some(value) = &node.value {
        let result = format!("t{}", *temp_counter);
        *temp_counter += 1;
        instructions.push(TACInstruction::new(
            result.clone(),
            value.to_string(),
            None,
            None,
        ));
        return result;
    }

    if node.operator.is_none()
        && node.left.is_none()
        && node.right.is_none()
        && node.value.is_none()
    {
        if let Some(var_name) = &node.var_name {
            // println!("!! {:?} {:?}", var_map, var_name);

            let result = if let Some(temp_var) = var_map.get(var_name) {
                // If var_name is already in var_map, use the existing temporary variable
                temp_var.clone()
            } else {
                // If var_name is not in var_map, create a new temporary variable
                let temp_var = format!("t{}", *temp_counter);
                *temp_counter += 1;
                var_map.insert(var_name.clone(), temp_var.clone()); // store the variable in the variable map
                temp_var
            };
            return result;
        }
    }

    println!("{:?}", node);
    panic!("Unsupported AST node");
}

pub fn tacvec_to_string(tac: &Vec<TACInstruction>) -> String {
    let mut result = String::new();
    for instruction in tac {
        result.push_str(&instruction.to_string());
        result.push('\n');
    }
    result
}
