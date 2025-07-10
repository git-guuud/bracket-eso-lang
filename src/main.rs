// bracket only valid chars -> ()[]{}|
// var/func names -> valid bracket nestings only eg: (), (()), (()()), (()(()))
// numbers -> can represent as [] nesting inside () eg: 0 -> [] (equivalent to group of no statements), 
// 1 -> [()], 2 -> [()()], 3 -> [()()()], etc.
// Operators: *(eqivalent to --), &(equivalent to ++)
// {x}[] get value of x 
// {x}()[y] set var x to y 
// [x] | [y] | [z] if x != 0 then y else z 
// {x}(y)[z] define function x with y as argument and z as body 
// {x}[y] call function x with y as argument 
//
// 
// you can use [] to group up statements
// value of a group of statements is the last statement in the group
// functions/variables must be defined before use
// only single argument functions
// functions can be self-recursive
// local functions and global variables not allowed

use std::{collections::HashMap, fs};

mod parse;
mod eval;



fn main() {
    let code = fs::read_to_string("./mul.txt").expect("Failed to read code.txt");
    let tokens = parse::tokenize(&code).expect("Failed to tokenize code");
    println!("{}", tokens.iter().map(|t| t.to_string()).collect::<Vec::<String>>().join(""));
    
    let mut funcs = eval::FunctionTable{
        functions: HashMap::new(),
    };
    let mut vars = eval::VarTable {
        variables: HashMap::new(),
    };
    let mut index = 0;
    while index < tokens.len() {
        let (exp, end) = parse::get_next_expression(&tokens, index).expect("Failed to parse code");
        index = end;
        println!("{}", exp);
        let x = eval::eval_expression(&exp, &mut funcs, &mut vars).unwrap();
        println!("Result: {}", x);
    }
}
