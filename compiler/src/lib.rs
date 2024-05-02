mod a1lexer;
mod a2parser;
mod a3intermediate_code_generator;
mod a4optimization;
mod a5code_generator;
mod a6code_emission;
mod interpreter;
mod symbols;

use a2parser::*;
use a3intermediate_code_generator::*;
use a4optimization::optimize_tac;
use a5code_generator::helpers::assemblyvec_to_string;
use a5code_generator::*;
use a6code_emission::code_emission;
use serde::Serialize;
use symbols::{symtou8, TapeSymbols};

#[derive(Clone, Serialize)]
pub struct Token {
    pub type_: String,
    pub value: String,
}

pub struct CompilationResult {
    pub tokens: Vec<a1lexer::Token>,
    pub tokens_string: String,
    pub ast: AstNode,
    pub ast_string: String,
    pub tac: Vec<TACInstruction>,
    pub tac_string: String,
    pub optimized_tac: Vec<TACInstruction>,
    pub optimized_tac_string: String,
    pub assembly: Vec<AssemblyInstruction>,
    pub assembly_string: String,
    pub turing_program: String,
}

pub fn lexer(code: String) -> Vec<Token> {
    let tokens = a1lexer::lexer(code);

    // transform tokens from Vec<a1lexer::Token> to Vec<Token>
    let tokens = tokens
        .iter()
        .map(|token| Token {
            type_: format!("{:?}", token.type_),
            value: token.value.clone(),
        })
        .collect();
    tokens
}

pub fn lexer_json(code: String) -> String {
    let tokens = lexer(code);
    serde_json::to_string(&tokens).unwrap()
}

pub fn compile(code: &str) -> CompilationResult {
    let tokens = a1lexer::lexer(code.to_string());
    let tokens_string = a1lexer::tokens_to_string(tokens.clone());

    let ast = a2parser::parser(tokens.clone());
    let ast_string = serde_json::to_string_pretty(&ast).unwrap();

    let tac = tac_generator(&ast);
    let tac_string = tacvec_to_string(&tac);

    let optimized_tac = optimize_tac(tac.clone());
    let optimized_tac_string = tacvec_to_string(&optimized_tac);

    let (assembly, storage_size) = code_generator(optimized_tac.clone());
    let assembly_string = assemblyvec_to_string(assembly.clone());

    let mut turing_instructions: Vec<String> = code_emission(assembly.clone());
    let tape = create_tape(storage_size);

    // embed original program into turing code for debugging
    turing_instructions.insert(0, "\n# Original program".to_string());
    turing_instructions.insert(1, ("\n".to_string() + &code).replace("\n", "\n#program "));

    let turing_contents = format!("{}\n{}", tape, turing_instructions.join("\n"));

    CompilationResult {
        tokens,
        tokens_string,
        ast,
        ast_string,
        tac,
        tac_string,
        optimized_tac,
        optimized_tac_string,
        assembly,
        assembly_string,
        turing_program: turing_contents,
    }
}

pub fn create_tape(storage_size: i32) -> String {
    let mut tape_storage_vec = Vec::new();

    for _ in 0..storage_size {
        let binary = "0 ".repeat(8);
        tape_storage_vec.push(binary.trim().to_string());
    }
    let tape_storage = format!("6 {} 6", tape_storage_vec.join(" 6 "));

    let start_a = symtou8(TapeSymbols::StartA).to_string();
    let end_a = symtou8(TapeSymbols::EndA).to_string();
    let end_b = symtou8(TapeSymbols::EndB).to_string();
    let end_c = symtou8(TapeSymbols::EndC).to_string();
    let middle = symtou8(TapeSymbols::Middle).to_string();

    let tape_working_area = format!(
        "{start_a} 0 0 0 0 0 0 0 0 {end_a} 0 0 0 0 0 0 0 0 {end_b} 0 0 0 0 0 0 0 0 {end_c}"
    );

    format!("{tape_storage} !{middle} {tape_working_area}")
}
