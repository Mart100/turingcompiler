pub mod helpers;
mod variables;

use std::collections::HashMap;

use self::{helpers::helpers::*, variables::Variables};
use crate::a3intermediate_code_generator::TACInstruction;

#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    SAVE { destination: String, value: u8 },
    SET { destination: String, value: u8 },
    LOAD { destination: String, source: String }, // Storage to working area
    STORE { destination: String, source: String }, // Working area to storage
    MOVE { destination: String, source: String }, // Storage to storage
    JMP { label: String },
    JNZ { label: String },
    LABEL { label: String },
    FN { name: String },
    ADD,
    SUB,
    SUB_SAFE,
    MUL,
    NOT,
    DIV,
    ISZERO,
    ENDFN,
}

impl AssemblyInstruction {
    pub fn to_string(&self) -> String {
        match self {
            AssemblyInstruction::SAVE { destination, value } => {
                format!("SAVE {} {}", destination, value)
            }
            AssemblyInstruction::LOAD {
                destination,
                source,
            } => format!("LOAD {} {}", source, destination),
            AssemblyInstruction::STORE {
                destination,
                source,
            } => format!("STORE {} {}", destination, source),
            AssemblyInstruction::SET { destination, value } => {
                format!("SET {} {}", destination, value)
            }
            AssemblyInstruction::MOVE {
                destination,
                source,
            } => {
                format!("MOVE {} {}", destination, source)
            }
            AssemblyInstruction::JMP { label } => format!("JMP {}", label),
            AssemblyInstruction::JNZ { label } => format!("JNZ {}", label),
            AssemblyInstruction::LABEL { label } => format!("{}:", label),
            AssemblyInstruction::FN { name } => format!("{}:", name),
            AssemblyInstruction::ADD => "ADD".to_string(),
            AssemblyInstruction::SUB => "SUB".to_string(),
            AssemblyInstruction::SUB_SAFE => "SUB_SAFE".to_string(),
            AssemblyInstruction::NOT => "NOT".to_string(),
            AssemblyInstruction::MUL => "MUL".to_string(),
            AssemblyInstruction::DIV => "DIV".to_string(),
            AssemblyInstruction::ISZERO => "ISZERO".to_string(),
            AssemblyInstruction::ENDFN => "ENDFN".to_string(),
        }
    }
}

pub fn code_generator(tac: Vec<TACInstruction>) -> (Vec<AssemblyInstruction>, i32) {
    let mut variables = Variables::new();
    let mut code = Vec::new();
    let mut functions: HashMap<String, i32> = HashMap::new(); // <function_name, number_of_calls>

    for instruction in tac {
        let mut latest_func: Option<String> = None;

        match instruction {
            TACInstruction::Assignment { var_name, value } => {
                println!("var_name: {}, {}", var_name, value);
                if value.parse::<u8>().is_ok() {
                    code.push(AssemblyInstruction::SET {
                        destination: var_name.clone(),
                        value: value.parse().unwrap(),
                    });
                    variables.set(var_name.clone());
                } else {
                    variables.set(var_name.clone());
                    variables.set(value.clone());
                    code.push(AssemblyInstruction::MOVE {
                        destination: value.clone(),
                        source: var_name.clone(),
                    });
                }
            }
            TACInstruction::IfGoto { condition, label } => {
                variables.set(condition.clone());

                code.push(AssemblyInstruction::LOAD {
                    destination: "A".to_string(),
                    source: condition.clone(),
                });
                code.push(AssemblyInstruction::JNZ { label });
            }
            TACInstruction::IfNotGoto { condition, label } => {
                variables.set(condition.clone());

                code.push(AssemblyInstruction::LOAD {
                    destination: "A".to_string(),
                    source: condition.clone(),
                });
                code.push(AssemblyInstruction::ISZERO);
                code.push(AssemblyInstruction::JNZ { label });
            }
            TACInstruction::Goto { label } => {
                code.push(AssemblyInstruction::JMP { label });
            }
            TACInstruction::Label { label } => {
                code.push(AssemblyInstruction::LABEL { label });
            }
            TACInstruction::Return { value } => {
                variables.set(value.clone());
                code.push(AssemblyInstruction::LOAD {
                    destination: "A".to_string(),
                    source: value.clone(),
                });
                code.push(AssemblyInstruction::ENDFN);
            }
            TACInstruction::Function { name } => {
                latest_func = Some(name.clone());
                functions.insert(name.clone(), 0);
                code.push(AssemblyInstruction::FN { name });
            }
            TACInstruction::FunctionCall { name, args } => {
                let entry = functions.entry(name.clone()).or_insert(0);

                code.push(AssemblyInstruction::SET {
                    destination: format!("F_{}", name.clone()),
                    value: entry.clone() as u8,
                });

                code.push(AssemblyInstruction::JMP {
                    label: name.clone(),
                });

                code.push(AssemblyInstruction::LABEL {
                    label: format!("L{name}_{entry}"),
                });

                *entry += 1;
            }

            TACInstruction::BinaryOperation {
                result,
                left,
                operator,
                right,
            } => {
                let operation = operator_char_to_string(&operator);

                variables.set(result.clone());
                variables.set(left.clone());
                variables.set(right.clone());

                match operation.as_str() {
                    "ADD" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "A".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::ADD);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    "SUB" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "A".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::SUB);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    "CMP" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "A".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::SUB);
                        code.push(AssemblyInstruction::ISZERO);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    "GT" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "A".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::SUB_SAFE);
                        code.push(AssemblyInstruction::ISZERO);
                        code.push(AssemblyInstruction::NOT);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    "LT" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "A".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::SUB_SAFE);
                        code.push(AssemblyInstruction::ISZERO);
                        code.push(AssemblyInstruction::NOT);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    "MUL" => {
                        code.push(AssemblyInstruction::LOAD {
                            destination: "B".to_string(),
                            source: left.clone(),
                        });

                        code.push(AssemblyInstruction::LOAD {
                            destination: "C".to_string(),
                            source: right.clone(),
                        });

                        code.push(AssemblyInstruction::MUL);

                        code.push(AssemblyInstruction::STORE {
                            destination: result.clone(),
                            source: "A".to_string(),
                        });
                    }
                    _ => {}
                }
            }
            _ => panic!("Instruction {} not implemented", instruction.to_string()),
        }
    }

    // add functions to variables
    for (name, frequency) in functions.clone() {
        variables.add(format!("F_{name}"), frequency as u32);
    }

    println!("Functions: {:?}", functions.clone());
    println!("Variables: {:?}", variables);

    variables.calculate_addresses();

    // Assign Addresses to all variables
    for instruction in code.iter_mut() {
        match instruction {
            AssemblyInstruction::LOAD { destination, .. }
            | AssemblyInstruction::STORE { destination, .. }
            | AssemblyInstruction::SET { destination, .. }
            | AssemblyInstruction::SAVE { destination, .. }
            | AssemblyInstruction::MOVE { destination, .. } => {
                if let Some(var) = variables.get(destination) {
                    *destination = var.get_address();
                }
            }

            _ => {}
        }
        match instruction {
            AssemblyInstruction::LOAD { source, .. }
            | AssemblyInstruction::STORE { source, .. }
            | AssemblyInstruction::MOVE { source, .. } => {
                if let Some(var) = variables.get(source) {
                    *source = var.get_address();
                }
            }
            _ => {}
        }
    }

    (code, variables.count())
}
