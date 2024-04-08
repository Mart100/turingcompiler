use crate::a2parser::AstNode;

#[derive(Debug, Clone)]
pub struct TACInstruction {
    pub result: String,
    pub left: String,
    pub operator: Option<String>,
    pub right: Option<String>,
}

impl TACInstruction {
    fn new(result: String, left: String, operator: Option<String>, right: Option<String>) -> Self {
        TACInstruction {
            result,
            left,
            operator,
            right,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.operator {
            Some(op) => format!(
                "{} = {} {} {}",
                self.result,
                self.left,
                op,
                self.right.as_ref().unwrap()
            ),
            None => format!("{} = {}", self.result, self.left),
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
    match &node.operator {
        Some(op) => {
            let left_tac = generate_tac(&*node.left.as_ref().unwrap(), instructions, temp_counter);
            let right_tac =
                generate_tac(&*node.right.as_ref().unwrap(), instructions, temp_counter);
            let result = format!("t{}", *temp_counter);
            *temp_counter += 1;
            instructions.push(TACInstruction::new(
                result.clone(),
                left_tac,
                Some(op.clone()),
                Some(right_tac),
            ));
            result
        }
        None => {
            if let Some(value) = &node.value {
                let result = format!("t{}", *temp_counter);
                *temp_counter += 1;
                instructions.push(TACInstruction::new(
                    result.clone(),
                    value.to_string(),
                    None,
                    None,
                ));
                result
            } else {
                panic!("Unsupported AST node");
            }
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
