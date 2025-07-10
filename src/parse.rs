use std::fmt;



#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    OpenParen,  // (
    CloseParen, // )
    OpenSquare, // [
    CloseSquare, // ]
    OpenCurly, // {
    CloseCurly, // }
    Pipe, // |
    // OpenAngle, // <
    // CloseAngle, // >
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenSquare => write!(f, "["),
            Token::CloseSquare => write!(f, "]"),
            Token::OpenCurly => write!(f, "{{"),
            Token::CloseCurly => write!(f, "}}"),
            Token::Pipe => write!(f, "|"),
            // Token::OpenAngle => write!(f, "<"),
            // Token::CloseAngle => write!(f, ">"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Var(String);
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum Expression {
    If(Box<Expression>, Box<Expression>, Box<Expression>), // x|y|z if x != 0 then y else z
    FunctionDef(Var, Vec<Var>, Box<Expression>), // define function x with y as argument and z as body
    FunctionCall(Var, Box<Expression>), // call function x with y as argument
    ListExp(Vec<Expression>), // List of expressions
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::If(cond, then_branch, else_branch) => write!(f, "if ({}) then {} else {}", cond, then_branch, else_branch),
            Expression::FunctionDef(var, args, body) => write!(f, "{}({}) {{\n{}\n}}", var.0, args.iter().map(|v| v.0.clone()).collect::<Vec<String>>().join(", "), body),
            Expression::FunctionCall(var, arg) => write!(f, "{}({})", var.0, arg),
            Expression::ListExp(exprs) => write!(f, "{}", exprs.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(";\n")),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '[' => tokens.push(Token::OpenSquare),
            ']' => tokens.push(Token::CloseSquare),
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '|' => tokens.push(Token::Pipe),
            // '<' => tokens.push(Token::OpenAngle),
            // '>' => tokens.push(Token::CloseAngle),
            _ => {
                if !c.is_whitespace() {
                    return Err(format!("Unexpected character: {}", c));
                }
            }
        }
    }
    Ok(tokens)
}

// pub fn parse(tokens: Vec<Token>) -> Result<Expression, String> {
    
// }

pub fn get_next_expression(tokens: &Vec<Token>, start: usize) -> Result<(Expression, usize), String> {
    if start >= tokens.len() {
        return Err("No more tokens to parse".to_string());
    }
    let mut index = start;

    match &tokens[index] {
        Token::OpenCurly => {
            let (var, end) = get_var_name(tokens, index+1).unwrap();
            index = end;
            if index >= tokens.len()-1 {
                return Err("Unexpected end of tokens after variable name".to_string());
            } else if tokens[index] != Token::CloseCurly {
                return Err("Expected CloseCurly after variable name".to_string());
            }

            index += 1; 
            if tokens[index] == Token::OpenParen {
                let (args, end) = get_var_list(tokens, index).unwrap();
                index = end;
                if index >= tokens.len() {
                    return Err("Unexpected end of tokens after arg list".to_string());
                } else if tokens[index] != Token::OpenSquare {
                    return Err("Expected OpenSquare after arg list".to_string());
                }
                
                let (body, end) = get_expression_list(tokens, index).unwrap();
                index = end;
                
                return Ok((Expression::FunctionDef(Var(var), args, Box::new(Expression::ListExp(body))), index));
            } else if tokens[index] == Token::OpenSquare {
                let (body, end) = get_expression_list(tokens, index).unwrap();
                index = end;
                
                return Ok((Expression::FunctionCall(Var(var), Box::new(Expression::ListExp(body))), index));
            } else {
                return Err("Expected OpenParen or OpenSquare after variable name".to_string());
            }
        }

        Token::OpenSquare => {
            let (condition, end) = get_expression_list(tokens, index).unwrap();
            index = end;

            if index == tokens.len() {
                return Ok((Expression::ListExp(condition), index));
            } else if index >= tokens.len() - 1 {
                return Err("Unexpected end of tokens after condition".to_string());
            } else if tokens[index] != Token::Pipe {
                return Err("Expected Pipe after condition".to_string());
            }
            index += 1; 

            let (then_branch, end) = get_expression_list(tokens, index).unwrap();
            index = end;

            if index >= tokens.len() - 1 {
                return Err("Unexpected end of tokens after then branch".to_string());
            } else if tokens[index] != Token::Pipe {
                return Ok((Expression::If(Box::new(Expression::ListExp(condition)), 
                                        Box::new(Expression::ListExp(then_branch)), 
                                        Box::new(Expression::ListExp(Vec::new()))), index));
            }
            index += 1;

            let (else_branch, end) = get_expression_list(tokens, index).unwrap();
            index = end;

            return Ok((Expression::If(Box::new(Expression::ListExp(condition)), 
                                        Box::new(Expression::ListExp(then_branch)), 
                                        Box::new(Expression::ListExp(else_branch))), index));
        }
        _ => return Err(format!("Unexpected token at {}", index)),
    }
}

fn get_expression_list(tokens: &Vec<Token>, start: usize) -> Result<(Vec<Expression>, usize), String> {
    let mut index = start;
    let mut expressions = Vec::new();

    if tokens[index] != Token::OpenSquare {
        return Err("Expected OpenSquare at start of expression list".to_string());
    }
    index+=1;
    while index < tokens.len() {
        match &tokens[index] {
            Token::CloseSquare => {
                index += 1;
                break;
            }
            _ => {
                let (expr, end) = get_next_expression(tokens, index).unwrap();
                expressions.push(expr);
                index = end;
                if index >= tokens.len() {
                    return Err("Expected ] at end of expression list".to_string());
                }
            }
        }
    }

    Ok((expressions, index))
}

fn get_var_name(tokens: &Vec<Token>, start: usize) -> Result<(String, usize), String> {
    if tokens[start] != Token::OpenParen {
        return Err("Expected OpenParen at start of variable name".to_string());
    }
    let mut index = start + 1;
    let mut var_name = "(".to_string();

    let mut paren_count = 1;
    while index < tokens.len() {
        match &tokens[index] {
            Token::OpenParen => {
                paren_count += 1;
                var_name.push('(');
            }
            Token::CloseParen => {
                paren_count -= 1;
                var_name.push(')');
            }
            _ => return Err("Unexpected token in variable name".to_string()),
        }
        index += 1;
        if paren_count == 0 {
            break;
        }
    }

    Ok((var_name, index))
}

fn get_var_list(tokens: &Vec<Token>, start: usize) -> Result<(Vec<Var>, usize), String> {
    if tokens[start] != Token::OpenParen {
        return Err("Expected OpenParen at start of variable list".to_string());
    }
    let mut index = start+1;
    let mut vars = Vec::new();

    while index < tokens.len() {
        match &tokens[index] {
            Token::OpenParen => {
                let (var_name, end) = get_var_name(tokens, index)?;
                vars.push(Var(var_name));
                index = end;
            }
            Token::CloseParen => {index+=1; break},
            _ => return Err("Unexpected token in variable list".to_string()),
        }
    }

    Ok((vars, index))
}

