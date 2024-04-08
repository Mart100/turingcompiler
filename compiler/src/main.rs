use std::fs;

mod a1lexer;
mod a2parser;
mod a3intermediate_code_generator;
mod a4code_generator;
mod a5code_emission;
mod symbols;

use a1lexer::*;
use a2parser::*;
use a3intermediate_code_generator::*;
use a4code_generator::*;

use crate::a5code_emission::code_emission;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();

    let tokens = lexer(&code);
    let tokens_string = tokens_to_string(tokens.clone());
    fs::write("compiler_steps/step1_tokens.txt", &tokens_string).unwrap();

    let ast = parser(tokens);
    let ast_string = format!("{:?}", ast);
    fs::write("compiler_steps/step2_ast.txt", &ast_string).unwrap();

    let tac = tac_generator(&ast);
    let tac_string = tacvec_to_string(&tac);
    fs::write("compiler_steps/step3_tac.txt", &tac_string).unwrap();

    let assembly = code_generator(tac.clone());
    let assembly_string = assemblyvec_to_string(assembly.clone());
    fs::write("compiler_steps/step4_assembly.txt", &assembly_string).unwrap();

    let (mut start_tape, code) = code_emission(assembly);
    start_tape.retain(|s| !s.is_empty());

    let turing_contents = format!("{}\n{}", start_tape.join(" "), code.join("\n"));
    fs::write("compiler_steps/step5_turingcode.txt", &turing_contents).unwrap();
    fs::write("output.txt", &turing_contents).unwrap();
}
