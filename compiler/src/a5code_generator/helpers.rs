pub use super::*;

pub fn assemblyvec_to_string(assembly: Vec<AssemblyInstruction>) -> String {
    let mut result = String::new();

    for instruction in assembly {
        result.push_str(&format!("{}\n", instruction.to_string()));
    }

    result
}

pub fn operator_char_to_string(op: &str) -> String {
    match op {
        "+" => "ADD".to_string(),
        "-" => "SUB".to_string(),
        "*" => "MUL".to_string(),
        "/" => "DIV".to_string(),
        "=" => "MOV".to_string(),
        "==" => "CMP".to_string(),
        ">" => "GT".to_string(),
        ">=" => "GE".to_string(),
        "<" => "LT".to_string(),
        "<=" => "LE".to_string(),
        _ => panic!("Unsupported operator"),
    }
}

pub fn tvar_to_svar(tvar: &mut String) {
    if tvar.starts_with("t") {
        *tvar = "S".to_string() + &tvar[1..];
    } else {
        *tvar = tvar.to_string();
    }
}
