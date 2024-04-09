use crate::a3intermediate_code_generator::TACInstruction;

#[derive(Debug, Clone)]
pub struct AssemblyInstruction {
    pub operation: String,
    pub operand1: Option<String>,
    pub operand2: Option<String>,
}

impl AssemblyInstruction {
    fn new(operation: String, operand1: Option<String>, operand2: Option<String>) -> Self {
        AssemblyInstruction {
            operation,
            operand1,
            operand2,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            self.operation,
            self.operand1
                .as_ref()
                .map(|s| format!(" {}", s))
                .unwrap_or_default(),
            self.operand2
                .as_ref()
                .map(|s| format!(", {}", s))
                .unwrap_or_default(),
        )
    }
}

pub fn code_generator(tac: Vec<TACInstruction>) -> Vec<AssemblyInstruction> {
    let mut storage = Vec::new();
    let mut savescode = Vec::new();
    let mut code = Vec::new();
    let mut temp_counter = 1;

    for instruction in tac {
        let mut result = instruction.result;
        let mut left = instruction.left;
        let operator = instruction.operator;
        let mut right = instruction.right;

        if result.starts_with("t") {
            result = "S".to_string() + &result[1..];
        }

        if left.starts_with("t") {
            left = "S".to_string() + &left[1..];
        }

        if right.is_some() && right.as_ref().unwrap().starts_with("t") {
            right = Some("S".to_string() + &right.as_ref().unwrap()[1..]);
        }

        // function that creates new storage SAVE
        let mut save = |storage: &mut Vec<String>, address: String, value: String| {
            savescode.push(AssemblyInstruction::new(
                "SAVE".to_string(),
                Some(address.clone()),
                Some(value),
            ));
            storage.push(format!("S{}", address));
        };

        match operator.as_deref() {
            Some(op) => {
                let operation = operator_char_to_string(op);

                match operation.as_str() {
                    "ADD" => {
                        code.push(AssemblyInstruction::new(
                            "LOAD".to_string(),
                            Some(left.clone()),
                            Some("A".to_string()),
                        ));

                        code.push(AssemblyInstruction::new(
                            "LOAD".to_string(),
                            Some(right.unwrap().clone()),
                            Some("B".to_string()),
                        ));

                        code.push(AssemblyInstruction::new(operation, None, None));

                        code.push(AssemblyInstruction::new(
                            "STORE".to_string(),
                            Some("A".to_string()),
                            Some(result.clone()),
                        ));
                    }
                    "MUL" => {
                        code.push(AssemblyInstruction::new(
                            "LOAD".to_string(),
                            Some(left.clone()),
                            Some("B".to_string()),
                        ));

                        code.push(AssemblyInstruction::new(
                            "LOAD".to_string(),
                            Some(right.unwrap().clone()),
                            Some("C".to_string()),
                        ));

                        code.push(AssemblyInstruction::new(operation, None, None));

                        code.push(AssemblyInstruction::new(
                            "STORE".to_string(),
                            Some("A".to_string()),
                            Some(result.clone()),
                        ));
                    }
                    _ => {}
                }
            }
            None => {
                save(&mut storage, result.clone(), left.clone());
            }
            _ => (),
        }

        if operator.is_some() && !storage.contains(&result.clone()) {
            save(&mut storage, result.clone(), "0".to_string());
        }

        temp_counter += 1;
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
        _ => panic!("Unsupported operator"),
    }
}
