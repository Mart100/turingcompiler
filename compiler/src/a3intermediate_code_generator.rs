use crate::a2parser::AstNode;

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
    IfNotGoto {
        condition: String,
        label: String,
    },
    Return {
        value: String,
    },
    FunctionCall {
        name: String,
        args: Vec<String>,
    },
    Function {
        name: String,
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
                format!("{result} = {left} {operator} {right}")
            }
            TACInstruction::Assignment { var_name, value } => {
                format!("{var_name} = {value}")
            }
            TACInstruction::Label { label } => {
                format!("{label}:")
            }
            TACInstruction::Goto { label } => {
                format!("goto {label}")
            }
            TACInstruction::IfGoto { condition, label } => {
                format!("if {condition} goto {label}")
            }
            TACInstruction::IfNotGoto { condition, label } => {
                format!("if !{condition} goto {label}")
            }
            TACInstruction::Return { value } => {
                format!("return {value}")
            }
            TACInstruction::Function { name } => {
                format!("{name}:")
            }
            TACInstruction::FunctionCall { name, args } => {
                format!("call {name} {}", args.join(", "))
            }
        }
    }
}

pub fn tac_generator(ast: &AstNode) -> Vec<TACInstruction> {
    let mut instructions = Vec::new();
    let mut temp_counter = 1;

    generate_tac(ast, &mut instructions, &mut temp_counter);

    instructions
}

fn generate_tac(
    node: &AstNode,
    instructions: &mut Vec<TACInstruction>,
    temp_counter: &mut u32,
) -> String {
    match node {
        AstNode::Body(nodes) => {
            for node in nodes {
                generate_tac(node, instructions, temp_counter);
            }
            return "".to_string();
        }
        AstNode::Declaration { var_name, value } => {
            let right_tac = generate_tac(&*value, instructions, temp_counter);

            instructions.push(TACInstruction::Assignment {
                var_name: var_name.clone(),
                value: right_tac,
            });

            return var_name.to_string();
        }
        AstNode::Assignment { var_name, value } => {
            let right_tac = generate_tac(&*value, instructions, temp_counter);

            instructions.push(TACInstruction::Assignment {
                var_name: var_name.clone(),
                value: right_tac,
            });
            return var_name.to_string();
        }
        AstNode::BinaryOperation {
            left,
            right,
            operator,
        } => {
            let left_tac = generate_tac(&*left, instructions, temp_counter);
            let right_tac = generate_tac(&*right, instructions, temp_counter);
            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            instructions.push(TACInstruction::BinaryOperation {
                result: result.clone(),
                left: left_tac,
                operator: operator.clone(),
                right: right_tac,
            });
            return result;
        }
        AstNode::Constant { value } => {
            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            instructions.push(TACInstruction::Assignment {
                var_name: result.clone(),
                value: value.clone(),
            });
            return result;
        }
        AstNode::Variable { name } => {
            return name.clone();
        }
        AstNode::Return { value } => {
            let value = generate_tac(&*value, instructions, temp_counter);
            instructions.push(TACInstruction::Return { value });
            return "".to_string();
        }
        AstNode::While { condition, body } => {
            let start_label = format!("L{}", *temp_counter);
            *temp_counter += 1;
            let end_label = format!("L{}", *temp_counter);
            *temp_counter += 1;

            instructions.push(TACInstruction::Label {
                label: start_label.clone(),
            });

            let condition_tac = generate_tac(&*condition, instructions, temp_counter);

            instructions.push(TACInstruction::IfNotGoto {
                condition: condition_tac,
                label: end_label.clone(),
            });

            generate_tac(&*body, instructions, temp_counter);

            instructions.push(TACInstruction::Goto {
                label: start_label.clone(),
            });

            instructions.push(TACInstruction::Label {
                label: end_label.clone(),
            });

            return "".to_string();
        }
        AstNode::FunctionCall { name, args } => {
            let mut arg_tac = Vec::new();
            let node = &**args;

            if let AstNode::CallArguments(arg_nodes) = node {
                for arg in arg_nodes {
                    arg_tac.push(generate_tac(arg, instructions, temp_counter));
                }
            } else {
                panic!("Expected AstNode::Arguments");
            }

            for (i, arg) in arg_tac.iter().enumerate() {
                instructions.push(TACInstruction::Assignment {
                    var_name: format!("arg{}", i),
                    value: arg.clone(),
                });
            }

            instructions.push(TACInstruction::FunctionCall {
                args: arg_tac,
                name: name.clone(),
            });

            return "ret".to_string();
        }
        AstNode::Function { name, args, body } => {
            let mut arg_tac = Vec::new();
            let node = &**args;

            if let AstNode::Arguments(arg_nodes) = node {
                for arg in arg_nodes {
                    arg_tac.push(generate_tac(arg, instructions, temp_counter));
                }
            } else {
                panic!("Expected AstNode::Arguments");
            }

            instructions.push(TACInstruction::Function { name: name.clone() });

            for (i, arg) in arg_tac.iter().enumerate() {
                instructions.push(TACInstruction::Assignment {
                    var_name: format!("arg{}", i),
                    value: arg.clone(),
                });
            }

            generate_tac(&*body, instructions, temp_counter);

            return "".to_string();
        }
        AstNode::Conditional {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_tac = generate_tac(&*condition, instructions, temp_counter);

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
            generate_tac(&*then_branch, instructions, temp_counter);

            if let Some(else_branch) = &else_branch {
                instructions.push(TACInstruction::Goto {
                    label: end_label.clone(),
                });

                instructions.push(TACInstruction::Label {
                    label: false_label.clone(),
                });

                generate_tac(&*else_branch, instructions, temp_counter);

                instructions.push(TACInstruction::Label {
                    label: end_label.clone(),
                });
            } else {
                instructions.push(TACInstruction::Label {
                    label: false_label.clone(),
                });
            }

            return "".to_string();
        }
        _ => {
            panic!("Unexpected node: {:?}", node);
        }
    }
}

pub fn tacvec_to_string(tac: &Vec<TACInstruction>) -> String {
    let mut result = String::new();
    for instruction in tac {
        result.push_str(&instruction.to_string());
        result.push('\n');
    }
    result
}
