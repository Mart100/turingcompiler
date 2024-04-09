use core::fmt;
use std::iter::Peekable;

use crate::a1lexer::Token;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub operator: Option<String>,
    pub left: Option<Box<AstNode>>,
    pub right: Option<Box<AstNode>>,
    pub value: Option<String>,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.operator {
            Some(op) => write!(
                f,
                "({} {} {})",
                self.left.as_ref().unwrap(),
                op,
                self.right.as_ref().unwrap()
            ),
            None => write!(f, "{}", self.value.as_ref().unwrap()),
        }
    }
}

pub fn parser(tokens: Vec<Token>) -> AstNode {
    let mut iter = tokens.into_iter().peekable();
    parse_expression(&mut iter)
}

fn parse_expression<I>(iter: &mut Peekable<I>) -> AstNode
where
    I: Iterator<Item = Token>,
{
    let mut node = parse_term(iter);

    while let Some(token) = iter.peek() {
        match token.value.as_str() {
            "+" | "-" => {
                let op = iter.next().unwrap().value.clone();
                let right = parse_term(iter);
                node = AstNode {
                    operator: Some(op),
                    left: Some(Box::new(node)),
                    right: Some(Box::new(right)),
                    value: None,
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
            num => AstNode {
                operator: None,
                left: None,
                right: None,
                value: Some(num.to_string()),
            },
        }
    } else {
        panic!("Unexpected end of tokens");
    }
}
