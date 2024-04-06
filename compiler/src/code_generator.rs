use crate::parser::AstNode;

pub fn code_generator(ast: &AstNode) -> Vec<String> {
    let mut code = Vec::new();
    generate_code(ast, &mut code);
    code
}

fn generate_code(node: &AstNode, code: &mut Vec<String>) {
    match &node.operator {
        Some(operator) => {
            // Generate code for the operator
            code.push(generate_instruction(operator));

            // Recursively generate code for the left and right children
            if let Some(left) = &node.left {
                generate_code(&*left, code);
            }
            if let Some(right) = &node.right {
                generate_code(&*right, code);
            }
        }
        None => {
            // Generate code for a value
            if let Some(value) = &node.value {
                code.push(generate_instruction(value));
            }
        }
    }
}

fn generate_instruction(node: &str, state_counter: &mut u32) -> String {
    match node {
        "+" => {
            let instructions = vec![];
            instructions.join("\n")
        }
        _ if node.parse::<u8>().is_ok() => {
            // store number in binary
            let binary = format!("{:08b}", node.parse::<u8>().unwrap());

            format!("STATE{} 0 0 L STATE{}", state_counter, state_counter + 1)
        }
        _ => panic!("Unknown node: {}", node),
    }
}
