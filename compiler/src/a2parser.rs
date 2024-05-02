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
pub enum AstNode {
    Variable {
        name: String,
    },
    Constant {
        value: String,
    },
    BinaryOperation {
        operator: String,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Conditional {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    Assignment {
        var_name: String,
        value: Box<AstNode>,
    },
    Declaration {
        var_name: String,
        value: Box<AstNode>,
    },
    Return {
        value: Box<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Function {
        name: String,
        args: Box<AstNode>,
        body: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Box<AstNode>,
    },
    Body(Vec<AstNode>),
    Arguments(Vec<AstNode>),
    CallArguments(Vec<AstNode>),
}

pub fn parser(tokens: Vec<Token>) -> AstNode {
    let mut iter = tokens.into_iter().peekable();
    let mut ast_nodes = Vec::new();

    while iter.peek().is_some() {
        ast_nodes.push(parse_statement(&mut iter));
    }

    AstNode::Body(ast_nodes)
}

fn parse_body<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut nodes = Vec::new();

    iter.next(); // consume opening bracket

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            "}" => {
                iter.next(); // consume closing bracket
                break;
            }
            _ => nodes.push(parse_statement(iter)),
        }
    }

    AstNode::Body(nodes)
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
                AstNode::Declaration {
                    value: Box::new(right),
                    var_name,
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

                        AstNode::Conditional {
                            condition: Box::new(condition),
                            then_branch: Box::new(then_branch),
                            else_branch: Some(Box::new(else_branch)),
                        }
                    } else {
                        AstNode::Conditional {
                            condition: Box::new(condition),
                            then_branch: Box::new(then_branch),
                            else_branch: None,
                        }
                    }
                } else {
                    panic!("Unexpected end of tokens");
                }
            }
            "while" => {
                iter.next(); // consumes "while"
                println!("parse_statement: {:?}", iter.peek());
                let condition = parse_expression(iter); // parse the condition
                println!("condition: {:?}", condition);
                let body = parse_body(iter); // parse the body
                println!("body: {:?}", body);
                AstNode::While {
                    condition: Box::new(condition),
                    body: Box::new(body),
                }
            }
            "fn" => {
                iter.next(); // consume "fn"
                let var_name = iter.next().unwrap().value.clone(); // get function name
                assert_eq!(iter.next().unwrap().value, "("); // consume opening parenthesis
                let args = parse_arguments(iter); // parse the arguments

                let body = parse_body(iter); // parse the body
                AstNode::Function {
                    name: var_name,
                    args: Box::new(args),
                    body: Box::new(body),
                }
            }
            "return" => {
                iter.next(); // consume "return"
                let value = parse_expression(iter); // parse the return value
                AstNode::Return {
                    value: Box::new(value),
                }
            }
            _ => {
                let var_name = iter.next().unwrap().value.clone(); // get variable name
                if iter.peek().unwrap().value == "=" {
                    iter.next(); // consume "="
                    let right = parse_expression(iter); // parse the right-hand side
                    AstNode::Assignment {
                        value: Box::new(right),
                        var_name,
                    }
                } else if iter.peek().unwrap().value == "(" {
                    iter.next(); // consume "("
                    let args = parse_call_arguments(iter); // parse the arguments
                    AstNode::FunctionCall {
                        name: var_name,
                        args: Box::new(args),
                    }
                } else {
                    println!("Unexpected token: {:?}", iter.peek());
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
            panic!("Expected semicolon at the end of statement: {:?}", node);
        }
    };

    node
}

fn parse_arguments<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut args = Vec::new();

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            ")" => {
                iter.next(); // consume closing parenthesis
                break;
            }
            "," => {
                iter.next(); // consume comma
            }
            _ => {
                args.push(iter.next().unwrap().value.clone());
            }
        }
    }

    AstNode::Arguments(
        args.into_iter()
            .map(|arg| AstNode::Variable { name: arg })
            .collect(),
    )
}

fn parse_call_arguments<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut args = Vec::new();

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            ")" => {
                iter.next(); // consume closing parenthesis
                break;
            }
            "," => {
                iter.next(); // consume comma
            }
            _ => {
                args.push(parse_expression(iter));
            }
        }
    }

    AstNode::CallArguments(args)
}

fn parse_expression<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut node = parse_term(iter);

    println!("parse_expression: {:?}", node);

    while let Some(token) = iter.peek() {
        println!("parse_expression token: {:?}", token);
        match token.value.as_str() {
            "+" | "-" | "==" | ">" | "<" => {
                let op = iter.next().unwrap().value.clone();
                let right = parse_term(iter);
                node = AstNode::BinaryOperation {
                    operator: op,
                    left: Box::new(node),
                    right: Box::new(right),
                }
            }
            "(" => {
                if let AstNode::Variable { name } = node {
                    iter.next(); // consume "("
                    let args = parse_call_arguments(iter);
                    node = AstNode::FunctionCall {
                        name: name,
                        args: Box::new(args),
                    };
                }
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
                node = AstNode::BinaryOperation {
                    operator: op,
                    left: Box::new(node),
                    right: Box::new(right),
                }
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
            num if num.parse::<u8>().is_ok() => AstNode::Constant {
                value: num.to_string(),
            },
            id if id.parse::<String>().is_ok() => AstNode::Variable {
                name: id.to_string(),
            },
            _ => panic!("Unexpected token in parse_factor: {:?}", token),
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}
