use std::fs;

mod code_generator;
mod lexer;
mod parser;

use code_generator::code_generator;
use lexer::lexer;
use parser::parser;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();
    let tokens = lexer(&code);
    let ast = parser(tokens);
    let output = code_generator(&ast);
    fs::write("output.txt", &output.join("\n")).unwrap();
}
