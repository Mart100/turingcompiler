#[derive(Debug)]
pub enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Number,
}

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
}

pub fn lexer(code: &str) -> Vec<Token> {
    // For simplicity, consider each line as a single statement and split it by spaces.
    code.lines()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|word| match word {
                    "let" => Token {
                        type_: TokenType::Keyword,
                        value: word.to_string(),
                    },
                    "=" | "+" => Token {
                        type_: TokenType::Operator,
                        value: word.to_string(),
                    },
                    _ if word.parse::<u8>().is_ok() => Token {
                        type_: TokenType::Number,
                        value: word.to_string(),
                    },
                    _ => Token {
                        type_: TokenType::Identifier,
                        value: word.to_string(),
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
