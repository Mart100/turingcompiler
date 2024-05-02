use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct CompileResult {
    pub tokens: String,
    pub ast: String,
    pub tac: String,
    pub optimized_tac: String,
    pub assembly: String,
    pub turing_program: String,
}

#[wasm_bindgen]
pub fn compile(code: &str) -> CompileResult {
    let result = turing_compiler::compile(code);
    CompileResult {
        tokens: result.tokens_string,
        ast: result.ast_string,
        tac: result.tac_string,
        optimized_tac: result.optimized_tac_string,
        assembly: result.assembly_string,
        turing_program: result.turing_program,
    }
}

#[wasm_bindgen]
pub fn lexer(code: &str) -> String {
    console_error_panic_hook::set_once();
    turing_compiler::lexer_json(code.to_string())
}
