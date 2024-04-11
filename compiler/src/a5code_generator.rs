use crate::a3intermediate_code_generator::TACInstruction;

#[derive(Debug, Clone)]
pub enum AssemblyInstruction {
    SAVE { destination: String, value: u8 },
    LOAD { destination: String, source: String },
    STORE { destination: String, source: String },
    JMP { label: String },
    JNZ { label: String },
    LABEL { label: String },
    ADD,
    SUB,
    MUL,
    DIV,
    ISZERO,
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
            AssemblyInstruction::JMP { label } => format!("JMP {}", label),
            AssemblyInstruction::JNZ { label } => format!("JNZ {}", label),
            AssemblyInstruction::LABEL { label } => format!("{}:", label),
            AssemblyInstruction::ADD => "ADD".to_string(),
            AssemblyInstruction::SUB => "SUB".to_string(),
            AssemblyInstruction::MUL => "MUL".to_string(),
            AssemblyInstruction::DIV => "DIV".to_string(),
            AssemblyInstruction::ISZERO => "ISZERO".to_string(),
        }
    }
}

pub fn code_generator(tac: Vec<TACInstruction>) -> Vec<AssemblyInstruction> {
    let mut storage = Vec::new();
    let mut savescode = Vec::new();
    let mut code = Vec::new();
    let mut temp_counter = 1;

    for instruction in tac {
        match instruction {
            TACInstruction::Assignment {
                mut var_name,
                value,
            } => {
                tvar_to_svar(&mut var_name);
                savescode.push(AssemblyInstruction::SAVE {
                    destination: var_name.clone(),
                    value: value.parse().unwrap(),
                });
                storage.push(var_name);
            }
            TACInstruction::IfGoto {
                mut condition,
                label,
            } => {
                tvar_to_svar(&mut condition);
                code.push(AssemblyInstruction::LOAD {
                    destination: "A".to_string(),
                    source: condition.clone(),
                });
                code.push(AssemblyInstruction::JNZ { label });
            }
            TACInstruction::Goto { label } => {
                code.push(AssemblyInstruction::JMP { label });
            }
            TACInstruction::Label { label } => {
                code.push(AssemblyInstruction::LABEL { label });
            }
            TACInstruction::BinaryOperation {
                mut result,
                mut left,
                operator,
                mut right,
            } => {
                tvar_to_svar(&mut result);
                tvar_to_svar(&mut left);
                tvar_to_svar(&mut right);

                let operation = operator_char_to_string(&operator);

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

                // if the result is not in the storage, initialize it to 0
                if !storage.contains(&result.clone()) {
                    savescode.push(AssemblyInstruction::SAVE {
                        destination: result.clone(),
                        value: 0,
                    });
                    storage.push(format!("S{}", result.clone()));
                }
            }
            _ => panic!("Instruction {} not implemented", instruction.to_string()),
        }
    }

    let finalcode: Vec<AssemblyInstruction> =
        savescode.iter().chain(code.iter()).cloned().collect();

    finalcode
}

pub fn assemblyvec_to_string(assembly: Vec<AssemblyInstruction>) -> String {
    let mut result = String::new();

    for instruction in assembly {
        result.push_str(&format!("{}\n", instruction.to_string()));
    }

    result
}

fn operator_char_to_string(op: &str) -> String {
    match op {
        "+" => "ADD".to_string(),
        "-" => "SUB".to_string(),
        "*" => "MUL".to_string(),
        "/" => "DIV".to_string(),
        "=" => "MOV".to_string(),
        "==" => "CMP".to_string(),
        _ => panic!("Unsupported operator"),
    }
}

fn tvar_to_svar(tvar: &mut String) {
    if tvar.starts_with("t") {
        *tvar = "S".to_string() + &tvar[1..];
    } else {
        *tvar = tvar.to_string();
    }
}
