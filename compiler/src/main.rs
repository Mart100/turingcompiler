use std::fs;

mod a1lexer;
mod a2parser;
mod a3intermediate_code_generator;
mod a4optimization;
mod a5code_generator;
mod a6code_emission;
mod symbols;

use a1lexer::*;
use a2parser::*;
use a3intermediate_code_generator::*;
use a4optimization::optimize_tac;
use a5code_generator::*;

use crate::a6code_emission::code_emission;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();

    let tokens = lexer(code.clone());
    let tokens_string = tokens_to_string(tokens.clone());
    fs::write("compiler_steps/step1_tokens.txt", &tokens_string).unwrap();

    let ast = parser(tokens);
    let ast_json = serde_json::to_string(&ast).unwrap();
    fs::write("compiler_steps/step2_ast.json", format!("{}", &ast_json)).unwrap();

    let tac = tac_generator(&ast);
    let tac_string = tacvec_to_string(&tac);
    fs::write("compiler_steps/step3_tac.txt", &tac_string).unwrap();

    let optimized_tac = optimize_tac(tac.clone());
    let optimized_tac_string = tacvec_to_string(&optimized_tac);
    fs::write(
        "compiler_steps/step4_optimized_tac.txt",
        &optimized_tac_string,
    )
    .unwrap();

    let assembly = code_generator(optimized_tac.clone());
    let assembly_string = assemblyvec_to_string(assembly.clone());
    fs::write("compiler_steps/step5_assembly.txt", &assembly_string).unwrap();

    let (mut start_tape, mut turing_code) = code_emission(assembly);
    start_tape.retain(|s| !s.is_empty());

    // embed original program into turing code for debugging
    turing_code.insert(0, "\n# Original program".to_string());
    turing_code.insert(1, ("\n".to_string() + &code).replace("\n", "\n#p "));

    let turing_contents = format!("{}\n{}", start_tape.join(" "), turing_code.join("\n"));
    fs::write("compiler_steps/step6_turingcode.txt", &turing_contents).unwrap();
    fs::write("output.txt", &turing_contents).unwrap();
}
