use crate::lexer::Token;

#[derive(Debug)]
pub struct AstNode {
    pub operator: Option<String>,
    pub left: Option<Box<AstNode>>,
    pub right: Option<Box<AstNode>>,
    pub value: Option<String>,
}

pub fn parser(tokens: Vec<Token>) -> AstNode {
    // For simplicity, create an AstNode for each line (statement).
    // This is a very basic parser and needs to be improved for more complex scenarios.
    AstNode {
        operator: tokens.get(1).map(|token| token.value.clone()),
        left: tokens.get(0).map(|token| {
            Box::new(AstNode {
                operator: None,
                left: None,
                right: None,
                value: Some(token.value.clone()),
            })
        }),
        right: tokens.get(2).map(|token| {
            Box::new(AstNode {
                operator: None,
                left: None,
                right: None,
                value: Some(token.value.clone()),
            })
        }),
        value: None,
    }
}
