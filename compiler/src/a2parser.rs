use core::fmt;
use std::iter::Peekable;

use crate::a1lexer::Token;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub operator: Option<String>,
    pub left: Option<Box<AstNode>>,
    pub right: Option<Box<AstNode>>,
    pub value: Option<String>,
    pub var_name: Option<String>,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // println!("{:?}", self);
        match &self.operator {
            Some(op) if op.as_str() == "=" => write!(
                f,
                "{} = {}",
                self.var_name.as_ref().unwrap(),
                self.right.as_ref().unwrap()
            ),
            Some(op) => {
                if let Some(left) = self.left.as_ref() {
                    if let Some(right) = self.right.as_ref() {
                        return write!(f, "({} {} {})", left, op, right);
                    }
                }
                write!(f, "{}", op)
            }
            None => write!(f, "{}", self.value.as_ref().unwrap()),
        }
    }
}

pub fn parser(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut iter = tokens.into_iter().peekable();
    let mut ast_nodes = Vec::new();

    while iter.peek().is_some() {
        ast_nodes.push(parse_statement(&mut iter));
    }

    ast_nodes
}

fn parse_statement<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let node = if let Some(token) = iter.peek() {
        match token.value.as_str() {
            "let" => {
                iter.next(); // consume "let"
                let var_name = iter.next().unwrap().value.clone(); // get variable name
                assert_eq!(iter.next().unwrap().value, "="); // consume "="
                let right = parse_expression(iter); // parse the right-hand side
                AstNode {
                    operator: Some("=".to_string()),
                    left: None,
                    right: Some(Box::new(right)),
                    value: None,
                    var_name: Some(var_name),
                }
            }
            "if" => {
                iter.next(); // consume "if"
                let condition = parse_expression(iter); // parse the condition
                assert_eq!(iter.next().unwrap().value, "{"); // consume opening bracket
                let then_branch = parse_statement(iter);
                assert_eq!(iter.next().unwrap().value, "}"); // consume closing bracket

                // Check if there is an "else" branch
                if let Some(token) = iter.peek() {
                    if token.value == "else" {
                        iter.next(); // consume "else"
                        assert_eq!(iter.next().unwrap().value, "{"); // consume opening bracket
                        let else_branch = parse_statement(iter);
                        assert_eq!(iter.next().unwrap().value, "}"); // consume closing bracket

                        AstNode {
                            operator: Some("if".to_string()),
                            left: Some(Box::new(then_branch)),
                            right: Some(Box::new(else_branch)),
                            value: None,
                            var_name: None,
                        }
                    } else {
                        AstNode {
                            operator: Some("if".to_string()),
                            left: Some(Box::new(then_branch)),
                            right: None,
                            value: None,
                            var_name: None,
                        }
                    }
                } else {
                    panic!("Unexpected end of tokens");
                }
            }
            _ => parse_expression(iter),
        }
    } else {
        panic!("Unexpected end of tokens");
    };

    // Consume the semicolon at the end of the statement
    match iter.peek() {
        Some(token) if token.value == ";" => {
            iter.next();
        }
        _ => panic!("Expected semicolon at the end of statement"),
    };

    node
}

fn parse_expression<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut node = parse_term(iter);

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            "+" | "-" | "==" | ">" | "<" => {
                let op = iter.next().unwrap().value.clone();
                let right = parse_term(iter);
                node = AstNode {
                    operator: Some(op),
                    left: Some(Box::new(node)),
                    right: Some(Box::new(right)),
                    value: None,
                    var_name: None,
                };
            }
            "let" => {
                iter.next(); // consume "let"
                let var_name = iter.next().unwrap().value.clone(); // get variable name
                assert_eq!(iter.next().unwrap().value, "="); // consume "="
                let right = parse_expression(iter); // parse the right-hand side
                node = AstNode {
                    operator: Some("=".to_string()),
                    left: None,
                    right: Some(Box::new(right)),
                    value: None,
                    var_name: Some(var_name),
                };
            }
            _ => break,
        }
    }

    node
}

fn parse_term<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut node = parse_factor(iter);

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            "*" | "/" => {
                let op = iter.next().unwrap().value.clone();
                let right = parse_factor(iter);
                node = AstNode {
                    operator: Some(op),
                    left: Some(Box::new(node)),
                    right: Some(Box::new(right)),
                    value: None,
                    var_name: None,
                };
            }
            _ => break,
        }
    }

    node
}

fn parse_factor<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    if let Some(token) = iter.next() {
        match token.value.as_str() {
            "(" => {
                let node = parse_expression(iter);
                assert_eq!(iter.next().unwrap().value, ")");
                node
            }
            num if num.parse::<u8>().is_ok() => AstNode {
                operator: None,
                left: None,
                right: None,
                value: Some(num.to_string()),
                var_name: None,
            },
            id if id.parse::<String>().is_ok() => AstNode {
                operator: None,
                left: None,
                right: None,
                value: None,
                var_name: Some(id.to_string()),
            },
            _ => panic!("Unexpected token in parse_factor: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}

pub fn format_ast_vec(ast_nodes: &Vec<AstNode>) -> String {
    ast_nodes
        .iter()
        .map(|node| format_ast("", node, 0))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_ast(name: &str, node: &AstNode, depth: usize) -> String {
    let indent = "\t".repeat(depth);
    let mut result = format!("{}\n", name);

    if let Some(var_name) = &node.var_name {
        result.push_str(&format!("{}var_name: {}\n", indent, var_name));
    }
    if let Some(op) = &node.operator {
        result.push_str(&format!("{}Operator: {}\n", indent, op));
    }
    if let Some(value) = &node.value {
        result.push_str(&format!("{}Value: {}\n", indent, value));
    }
    if let Some(left) = &node.left {
        result.push_str(&format!(
            "{}{}",
            indent,
            &format_ast("Left", &*left, depth + 1)
        ));
    }
    if let Some(right) = &node.right {
        result.push_str(&format!(
            "{}{}",
            indent,
            &format_ast("Right", &*right, depth + 1)
        ));
    }

    result
}
