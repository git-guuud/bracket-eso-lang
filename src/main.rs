// bracket only valid chars -> ()[]{}|
// var/func names -> valid bracket nestings only eg: (), (()), (()()), (()(()))
// numbers -> ?
// {x}[] get value of x
// {x}()[y] set var x to y
// [x] | [y] | [z] if x != 0 then y else z
// {x}(y)[z] define function x with y as argument and z as body 
// {x}[y] call function x with y as argument 
// 
// you can use [] to group up statements
// functions/variables must be defined before use
// only single argument functions
// functions can be self-recursive
// local functions and global variables not allowed

use std::fs;

mod parse;



fn main() {
    let code = fs::read_to_string("./code.txt").expect("Failed to read code.txt");
    let tokens = parse::tokenize(&code).expect("Failed to tokenize code");
    let exp = parse::get_next_expression(&tokens, 0).expect("Failed to parse code");
    println!("{}", exp.0);
}
