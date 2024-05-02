use std::fs;

mod a1lexer;
mod a2parser;
mod a3intermediate_code_generator;
mod a4optimization;
mod a5code_generator;
mod a6code_emission;
mod interpreter;
mod symbols;

use a1lexer::*;
use a2parser::*;
use a3intermediate_code_generator::*;
use a4optimization::optimize_tac;
use a5code_generator::helpers::assemblyvec_to_string;
use a5code_generator::*;
use interpreter::run_code;

use crate::a6code_emission::code_emission;

fn main() {
    let source_code = fs::read_to_string("input.txt").unwrap();
    let turing_code = compile_debug(source_code);
    fs::write("output.txt", &turing_code).unwrap();
}

fn compile_debug(code: String) -> String {
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

    let tape = turing_compiler::create_tape(storage_size);

    let turing_contents = format!("{}\n{}", tape, turing_code.join("\n"));
    fs::write("compiler_steps/step6_turingcode.txt", &turing_contents).unwrap();
    turing_contents
}

#[cfg(test)]
mod tests {
    #[test]
    fn math1() {
        let code = "
let a = (4 + 2) * 3 + 6 - 20;
return a;";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 4);
    }

    #[test]
    fn comparison() {
        let code = "
let a = (22 - 8) * 4 + 5;
let b = a == 61;
return b;";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 1);
    }

    #[test]
    fn ifelse() {
        let code = "
let a = (22 - 8) * 4 + 5;
let b = a == 61;
let c = 0;
if b {
    c = 12;
} else {
    c = 2;
};
return c;";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 12);
    }

    #[test]
    fn whileloop() {
        let code = "
let a = 4;
let b = 2;
while (a > 0) {
    a = a - 1;
    b = b * 2;
};
return b;";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 32);
    }

    #[test]
    fn functions() {
        let code = "
fn add(b, c) {
    return b + c;
};
fn main() {
    let a = add(1,8);
    let d = add(2,3);
    let e = add(a,d);
    return e;
};";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 14);
    }

    fn fibonacci() {
        let code = "
fn fibonacci(n) {
    if n == 0 {
        return 0;
    };
    if n == 1 {
        return 1;
    };
    return fibonacci(n - 1) + fibonacci(n - 2);
};
let a = fibonacci(10);
return a;";
        let turing_code = turing_compiler::compile(code).turing_program;
        let result = crate::run_code(turing_code);
        assert_eq!(result, 55);
    }
}
