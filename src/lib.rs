use wasm_bindgen::prelude::*;
use std::collections::HashMap;

mod parse;
mod eval;

#[wasm_bindgen(module = "/site/rust_call.js")]
extern "C" {
    pub fn logOutput(s: &str);
}


#[wasm_bindgen]
pub fn eval_unwrapped(code: String) -> String {
    eval_all(code).unwrap_or_else(|err| format!("Error: {}", err))
}

pub fn eval_all(code: String) -> Result<String, String> {
    let mut output = String::new();
    let tokens = parse::tokenize(&code)?;
    // println!("{}", tokens.iter().map(|t| t.to_string()).collect::<Vec::<String>>().join(""));
    
    let mut funcs = eval::FunctionTable{
        functions: HashMap::new(),
    };
    let mut vars = eval::VarTable {
        variables: HashMap::new(),
    };
    let mut index = 0;
    while index < tokens.len() {
        let (exp, end) = parse::get_next_expression(&tokens, index)?;
        index = end; 
        // println!("{}", exp); 
        let x = eval::eval_expression(&exp, &mut funcs, &mut vars)?; 
        output.push_str(format!("Result: {}", x).as_str()); 
    }
    logOutput(format!("Code: {}\n", code).as_str());
    Ok(output)
}
