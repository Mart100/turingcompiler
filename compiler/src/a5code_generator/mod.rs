pub mod helpers;
mod variables;

use std::collections::HashMap;

use self::{helpers::helpers::*, variables::Variables};
use crate::a3intermediate_code_generator::TACInstruction;

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblyInstruction {
    SET {
        destination: String,
        value: u8,
    },
    LOAD {
        destination: String,
        source: String,
    }, // Storage to working area
    STORE {
        destination: String,
        source: String,
    }, // Working area to storage
    MOVE {
        destination: String,
        source: String,
    }, // Storage to storage
    JMP {
        label: String,
    },
    JNZ {
        label: String,
    },
    LABEL {
        label: String,
    },
    FN {
        name: String,
    },
    ADD,
    SUB,
    SUBSAFE,
    MUL,
    NOT,
    ISZERO,
    ENDFN {
        name: String,
        address: String,
        total: u8,
    },
}

impl AssemblyInstruction {
    pub fn to_string(&self) -> String {
        match self {
            AssemblyInstruction::LOAD {
                destination,
                source,
            } => format!("LOAD {source} {destination}"),
            AssemblyInstruction::STORE {
                destination,
                source,
            } => format!("STORE {destination} {source}"),
            AssemblyInstruction::SET { destination, value } => {
                format!("SET {destination} {value}")
            }
            AssemblyInstruction::MOVE {
                destination,
                source,
            } => {
                format!("MOVE {source} {destination}")
            }
            AssemblyInstruction::JMP { label } => format!("JMP {}", label),
            AssemblyInstruction::JNZ { label } => format!("JNZ {}", label),
            AssemblyInstruction::LABEL { label } => format!("{}:", label),
            AssemblyInstruction::FN { name } => format!("{}:", name),
            AssemblyInstruction::ADD => "ADD".to_string(),
            AssemblyInstruction::SUB => "SUB".to_string(),
            AssemblyInstruction::SUBSAFE => "SUB_SAFE".to_string(),
            AssemblyInstruction::NOT => "NOT".to_string(),
            AssemblyInstruction::MUL => "MUL".to_string(),
            AssemblyInstruction::ISZERO => "ISZERO".to_string(),
            AssemblyInstruction::ENDFN {
                total,
                name,
                address,
            } => format!("ENDFN {total} {name} {address}"),
        }
    }
}

pub fn code_generator(tac: Vec<TACInstruction>) -> (Vec<AssemblyInstruction>, i32) {
    let mut variables = Variables::new();
    let mut code = Vec::new();
    let mut functions: HashMap<String, i32> = HashMap::new(); // <function_name, number_of_calls>
    let mut latest_func: String = "main".to_string();

    for instruction in tac {
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
                        destination: var_name.clone(),
                        source: value.clone(),
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
            TACInstruction::Function { name } => {
                latest_func = name.clone();
                functions.insert(name.clone(), 0);
                code.push(AssemblyInstruction::FN { name });
            }
            TACInstruction::Return { value } => {
                let fn_name = latest_func.clone();
                variables.set(value.clone());

                if fn_name == "main" {
                    code.push(AssemblyInstruction::LOAD {
                        destination: "A".to_string(),
                        source: value.clone(),
                    });
                } else {
                    code.push(AssemblyInstruction::MOVE {
                        destination: "ret".to_string(),
                        source: value.clone(),
                    });
                    code.push(AssemblyInstruction::ENDFN {
                        name: fn_name.clone(),
                        address: format!("F_{fn_name}"),
                        total: 0,
                    });
                }
            }
            TACInstruction::FunctionCall { name, args: _ } => {
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

                        code.push(AssemblyInstruction::SUBSAFE);
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

                        code.push(AssemblyInstruction::SUBSAFE);
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

    // add ret variable, set frequency high so it will be at the start of the storage
    variables.add("ret".to_string(), 1000);

    println!("Functions: {:?}", functions.clone());
    println!("Variables: {:?}", variables);

    variables.calculate_addresses();

    // Assign Addresses to all variables
    for instruction in code.iter_mut() {
        match instruction {
            AssemblyInstruction::LOAD { destination, .. }
            | AssemblyInstruction::STORE { destination, .. }
            | AssemblyInstruction::SET { destination, .. }
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
        match instruction {
            AssemblyInstruction::ENDFN {
                name,
                address,
                total,
            } => {
                if let Some(var) = variables.get(address) {
                    *address = var.get_address();
                }
                let entry = functions.get(&name.clone()).unwrap();
                *total = *entry as u8;
            }
            _ => {}
        }
    }

    (code, variables.count())
}
