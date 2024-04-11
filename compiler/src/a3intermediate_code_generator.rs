use std::collections::HashMap;

use crate::a2parser::AstNode;

// #[derive(Debug, Clone)]
// pub struct TACInstruction {
//     pub result: String,
//     pub left: String,
//     pub operator: Option<String>,
//     pub right: Option<String>,
// }

#[derive(Debug, Clone)]
pub enum TACInstruction {
    Assignment {
        var_name: String,
        value: String,
    },
    BinaryOperation {
        result: String,
        left: String,
        operator: String,
        right: String,
    },
    Label {
        label: String,
    },
    Goto {
        label: String,
    },
    IfGoto {
        condition: String,
        label: String,
    },
}

impl TACInstruction {
    pub fn to_string(&self) -> String {
        match &self {
            TACInstruction::BinaryOperation {
                result,
                left,
                operator,
                right,
            } => {
                format!("{} = {} {} {}", result, left, operator, right)
            }
            TACInstruction::Assignment { var_name, value } => {
                format!("{} = {}", var_name, value)
            }
            TACInstruction::Label { label } => {
                format!("{}:", label)
            }
            TACInstruction::Goto { label } => {
                format!("goto {}", label)
            }
            TACInstruction::IfGoto { condition, label } => {
                format!("if {} goto {}", condition, label)
            }
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
    match node {
        AstNode::Declaration(declaration) => {
            let var_name = &declaration.var_name;
            let value = &declaration.value;
            let right_tac = generate_tac(&*value, instructions, temp_counter, var_map);

            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            var_map.insert(var_name.clone(), result.clone());

            instructions.push(TACInstruction::Assignment {
                var_name: result.clone(),
                value: right_tac,
            });
            return result;
        }
        AstNode::Assignment(assignment) => {
            let var_name = &assignment.var_name;
            let value = &assignment.value;
            let right_tac = generate_tac(&*value, instructions, temp_counter, var_map);

            let result = if let Some(temp_var) = var_map.get(var_name) {
                temp_var.clone()
            } else {
                let temp_var = format!("t{}", *temp_counter);
                *temp_counter += 1;
                var_map.insert(var_name.clone(), temp_var.clone());
                temp_var
            };

            instructions.push(TACInstruction::Assignment {
                var_name: result.clone(),
                value: right_tac,
            });
            return result;
        }
        AstNode::BinaryOperation(binary_operation) => {
            let left_tac =
                generate_tac(&*binary_operation.left, instructions, temp_counter, var_map);
            let right_tac = generate_tac(
                &*binary_operation.right,
                instructions,
                temp_counter,
                var_map,
            );
            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            instructions.push(TACInstruction::BinaryOperation {
                result: result.clone(),
                left: left_tac,
                operator: binary_operation.operator.clone(),
                right: right_tac,
            });
            return result;
        }
        AstNode::Constant(constant) => {
            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            instructions.push(TACInstruction::Assignment {
                var_name: result.clone(),
                value: constant.value.clone(),
            });
            return result;
        }
        AstNode::Variable(variable) => {
            let var_name = &variable.name;
            let result = if let Some(temp_var) = var_map.get(var_name) {
                temp_var.clone()
            } else {
                let temp_var = format!("t{}", *temp_counter);
                *temp_counter += 1;
                var_map.insert(var_name.clone(), temp_var.clone());
                temp_var
            };
            return result;
        }
        AstNode::Conditional(conditional) => {
            let condition_tac =
                generate_tac(&*conditional.condition, instructions, temp_counter, var_map);

            let true_label = format!("L{}", *temp_counter);
            *temp_counter += 1;
            let false_label = format!("L{}", *temp_counter);
            *temp_counter += 1;
            let end_label = format!("L{}", *temp_counter);
            *temp_counter += 1;

            instructions.push(TACInstruction::IfGoto {
                condition: condition_tac,
                label: true_label.clone(),
            });
            instructions.push(TACInstruction::Goto {
                label: false_label.clone(),
            });

            instructions.push(TACInstruction::Label {
                label: true_label.clone(),
            });
            let then_branch_tac = generate_tac(
                &*conditional.then_branch,
                instructions,
                temp_counter,
                var_map,
            );
            instructions.push(TACInstruction::Goto {
                label: end_label.clone(),
            });

            instructions.push(TACInstruction::Label {
                label: false_label.clone(),
            });
            if let Some(else_branch) = &conditional.else_branch {
                let else_branch_tac =
                    generate_tac(&*else_branch, instructions, temp_counter, var_map);
            }

            instructions.push(TACInstruction::Label {
                label: end_label.clone(),
            });

            return "".to_string();
        }
        _ => {}
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
