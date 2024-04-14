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
use a5code_generator::helpers::assemblyvec_to_string;
use a5code_generator::*;
use symbols::{symtou8, TapeSymbols};

use crate::a6code_emission::code_emission;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();

    let tokens = lexer(code.clone());
    let tokens_string = tokens_to_string(tokens.clone());
    fs::write("compiler_steps/step1_tokens.txt", &tokens_string).unwrap();

    let ast = parser(tokens);
    let ast_json = serde_json::to_string_pretty(&ast).unwrap();
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

    let (assembly, storage_size) = code_generator(optimized_tac.clone());
    let assembly_string = assemblyvec_to_string(assembly.clone());
    fs::write("compiler_steps/step5_assembly.txt", &assembly_string).unwrap();

    let mut turing_code = code_emission(assembly);

    // embed original program into turing code for debugging
    turing_code.insert(0, "\n# Original program".to_string());
    turing_code.insert(1, ("\n".to_string() + &code).replace("\n", "\n#program "));

    let tape = create_tape(storage_size);

    let turing_contents = format!("{}\n{}", tape, turing_code.join("\n"));
    fs::write("compiler_steps/step6_turingcode.txt", &turing_contents).unwrap();
    fs::write("output.txt", &turing_contents).unwrap();
}

fn create_tape(storage_size: i32) -> String {
    let mut tape_storage_vec = Vec::new();
    for _ in 0..storage_size {
        let binary = "0 ".repeat(8);
        tape_storage_vec.push(binary.trim().to_string());
    }
    let tape_storage = tape_storage_vec.join(" 6 ");

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
