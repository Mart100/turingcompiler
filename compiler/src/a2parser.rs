use core::fmt;
use std::iter::Peekable;

use serde::Serialize;

use crate::a1lexer::Token;

// pub struct AstNode {
//     pub operator: Option<String>,
//     pub left: Option<Box<AstNode>>,
//     pub right: Option<Box<AstNode>>,
//     pub value: Option<String>,
//     pub var_name: Option<String>,
// }

#[derive(Debug, Serialize)]
pub struct VariableNode {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ConditionalNode {
    pub condition: Box<AstNode>,
    pub then_branch: Box<AstNode>,
    pub else_branch: Option<Box<AstNode>>,
}

#[derive(Debug, Serialize)]
pub struct ConstantNode {
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct BinaryOperationNode {
    pub operator: String,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Debug, Serialize)]
pub struct AssignmentNode {
    pub var_name: String,
    pub value: Box<AstNode>,
}

#[derive(Debug, Serialize)]
pub struct DeclarationNode {
    pub var_name: String,
    pub value: Box<AstNode>,
}

#[derive(Debug, Serialize)]
pub enum AstNode {
    Variable(VariableNode),
    Constant(ConstantNode),
    BinaryOperation(BinaryOperationNode),
    Conditional(ConditionalNode),
    Assignment(AssignmentNode),
    Declaration(DeclarationNode),
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
                AstNode::Declaration(DeclarationNode {
                    value: Box::new(right),
                    var_name,
                })
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

                        AstNode::Conditional(ConditionalNode {
                            condition: Box::new(condition),
                            then_branch: Box::new(then_branch),
                            else_branch: Some(Box::new(else_branch)),
                        })
                    } else {
                        AstNode::Conditional(ConditionalNode {
                            condition: Box::new(condition),
                            then_branch: Box::new(then_branch),
                            else_branch: None,
                        })
                    }
                } else {
                    panic!("Unexpected end of tokens");
                }
            }
            _ => {
                let var_name = iter.next().unwrap().value.clone(); // get variable name
                if iter.peek().unwrap().value == "=" {
                    iter.next(); // consume "="
                    let right = parse_expression(iter); // parse the right-hand side
                    AstNode::Assignment(AssignmentNode {
                        value: Box::new(right),
                        var_name,
                    })
                } else {
                    parse_expression(iter)
                }
            }
        }
    } else {
        panic!("Unexpected end of tokens");
    };

    // Consume the semicolon at the end of the statement
    match iter.peek() {
        Some(token) if token.value == ";" => {
            iter.next();
        }
        _ => {
            println!("{:?}", iter.collect::<Vec<_>>());
            panic!("Expected semicolon at the end of statement")
        }
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
                node = AstNode::BinaryOperation(BinaryOperationNode {
                    operator: op,
                    left: Box::new(node),
                    right: Box::new(right),
                });
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
                node = AstNode::BinaryOperation(BinaryOperationNode {
                    operator: op,
                    left: Box::new(node),
                    right: Box::new(right),
                });
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
            num if num.parse::<u8>().is_ok() => AstNode::Constant(ConstantNode {
                value: num.to_string(),
            }),
            id if id.parse::<String>().is_ok() => AstNode::Variable(VariableNode {
                name: id.to_string(),
            }),
            _ => panic!("Unexpected token in parse_factor: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}
