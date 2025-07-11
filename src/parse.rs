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
    Inc,
    Dec,
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
            Token::Inc => write!(f, "&"),
            Token::Dec => write!(f, "*"),
            // Token::OpenAngle => write!(f, "<"),
            // Token::CloseAngle => write!(f, ">"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Var(pub String);
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum Expression {
    Val(u32), // Represents a number 
    If(Box<Expression>, Box<Expression>, Box<Expression>), // x|y|z if x != 0 then y else z
    FunctionDef(Var, Vec<Var>, Box<Expression>), // define function x with y as argument and z as body
    FunctionCall(Var, Box<Expression>), // call function x with y as argument
    ListExp(Vec<Expression>), // List of expressions
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Val(num) => write!(f, "{}", num),
            Expression::If(cond, then_branch, else_branch) => write!(f, "if ({}) then {} else {}", cond, then_branch, else_branch),

            Expression::FunctionDef(var, args, body) => {
                if args.is_empty() {
                    return write!(f, "{} {{\n{}\n}}", var.0, body)
                } else {
                    write!(f, "{}({}) {{\n{}\n}}", var.0, args.iter().map(|v| v.0.clone()).collect::<Vec<String>>().join(", "), body)
                }
            }
            
            Expression::FunctionCall(var, args) => {
                if **args == Expression::ListExp(Vec::new()) {
                    return write!(f, "{}", var.0)
                } else {
                    write!(f, "{}({})", var.0, args)
                }
            }
            Expression::ListExp(exprs) => write!(f, "{}", exprs.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(";\n")),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut index = 0;
    for c in input.chars() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '[' => tokens.push(Token::OpenSquare),
            ']' => tokens.push(Token::CloseSquare),
            '{' => tokens.push(Token::OpenCurly),
            '}' => tokens.push(Token::CloseCurly),
            '|' => tokens.push(Token::Pipe),
            '*' => tokens.push(Token::Dec),
            '&' => tokens.push(Token::Inc),
            // '<' => tokens.push(Token::OpenAngle),
            // '>' => tokens.push(Token::CloseAngle),
            _ => {
                if !c.is_whitespace() {
                    return Err(format!("Unexpected character: {} at {}", c, index));
                }
            }
        }
        index += 1;
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
            let (var, end) = get_var_name(tokens, index+1)?;
            index = end;
            if index >= tokens.len()-1 {
                return Err("Unexpected end of tokens after variable name".to_string());
            } else if tokens[index] != Token::CloseCurly {
                return Err(format!("Expected }} after variable name at {}", index));
            }

            index += 1; 
            if tokens[index] == Token::OpenParen {
                let (args, end) = get_var_list(tokens, index)?;
                index = end;
                if index >= tokens.len() {
                    return Err("Unexpected end of tokens after arg list".to_string());
                } else if tokens[index] != Token::OpenSquare {
                    return Err(format!("Expected [ after arg list at {}", index));
                }
                
                let (body, end) = get_expression_list(tokens, index)?;
                index = end;
                
                return Ok((Expression::FunctionDef(Var(var), args, Box::new(Expression::ListExp(body))), index));
            } else if tokens[index] == Token::OpenSquare {
                let (body, end) = get_expression_list(tokens, index)?;
                index = end;
                
                return Ok((Expression::FunctionCall(Var(var), Box::new(Expression::ListExp(body))), index));
            } else {
                return Err(format!("Expected ( or [ after variable name at {}", index));
            }
        }

        Token::OpenSquare => {
            if index + 1 >= tokens.len() {
                return Err("Unexpected end of tokens after [".to_string());
            }
            if tokens[index+1] == Token::OpenParen {
                let (num, end) = get_num(tokens, index)?;
                index = end;
                return Ok((Expression::Val(num), index));
            } else if tokens[index+1] == Token::CloseSquare {
                index += 2;
                return Ok((Expression::Val(0), index));
            }
            
            let (condition, end) = get_expression_list(tokens, index)?;
            index = end;

            if index == tokens.len() || tokens[index] != Token::Pipe {
                return Ok((Expression::ListExp(condition), index));
            } 
            // else if index >= tokens.len() - 1 {
            //     return Err("Unexpected end of tokens after condition".to_string());
            // } else if tokens[index] != Token::Pipe {
            //     return Err("Expected Pipe after condition".to_string());
            // }
            index += 1; 

            if index >= tokens.len() {
                return Err("Unexpected end of tokens while trying to find then branch of conditional".to_string());
            } 
            let (then_branch, end) = get_expression_list(tokens, index)?;
            index = end;

            if index >= tokens.len() || tokens[index] != Token::Pipe {
                return Ok((Expression::If(Box::new(Expression::ListExp(condition)), 
                                        Box::new(Expression::ListExp(then_branch)), 
                                        Box::new(Expression::ListExp(Vec::new()))), index));
            }
            index += 1;

            if index >= tokens.len() {
                return Err("Unexpected end of tokens while trying to find else branch of conditional".to_string());
            }

            let (else_branch, end) = get_expression_list(tokens, index)?;
            index = end;

            return Ok((Expression::If(Box::new(Expression::ListExp(condition)), 
                                        Box::new(Expression::ListExp(then_branch)), 
                                        Box::new(Expression::ListExp(else_branch))), index));
        }
        Token::Dec => {
            index+=1;
            if index >= tokens.len() {
                return Err("Unexpected end of tokens after decrement operator".to_string());
            }
            let (body, end) = get_next_expression(tokens, index)?;
            index = end;
            let body = Expression::ListExp(vec![body]);
            return Ok((Expression::FunctionCall(Var('*'.to_string()), Box::new(body)), index));
        }
        Token::Inc => {
            index+=1;
            if index >= tokens.len() {
                return Err("Unexpected end of tokens after increment operator".to_string());
            }
            let (body, end) = get_next_expression(tokens, index)?;
            index = end;
            let body = Expression::ListExp(vec![body]);
            return Ok((Expression::FunctionCall(Var('&'.to_string()), Box::new(body)), index));
        }
        _ => return Err(format!("Unexpected token at {}", index)),
    }
}

fn get_expression_list(tokens: &Vec<Token>, start: usize) -> Result<(Vec<Expression>, usize), String> {
    let mut index = start;
    if index >= tokens.len() {
        return Err("Unexpected end of tokens while trying to find expression list".to_string());
    }
    let mut expressions = Vec::new();

    if tokens[index] != Token::OpenSquare {
        return Err(format!("Expected [ at start of expression list at {}", index));
    }
    index+=1;
    loop {
        if index >= tokens.len() {
            return Err(format!("Unexpected end of tokens while parsing expression list at {}", index));
        }
        match &tokens[index] {
            Token::CloseSquare => {
                index += 1;
                break;
            }
            _ => {
                let (expr, end) = get_next_expression(tokens, index)?;
                expressions.push(expr);
                index = end;
                if index >= tokens.len() {
                    return Err(format!("Expected ] at end of expression list at {}", index));
                }
            }
        }
    }

    Ok((expressions, index))
}

fn get_var_name(tokens: &Vec<Token>, start: usize) -> Result<(String, usize), String> {
    if start >= tokens.len() {
        return Err("Unexpected end of token while trying to find variable name".to_string());
    }
    if tokens[start] != Token::OpenParen {
        return Err(format!("Expected OpenParen at start of variable name at {}", start));
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
            _ => return Err(format!("Unexpected token {} in variable name at {}", tokens[index], index)),
        }
        index += 1;
        if paren_count == 0 {
            break;
        }
    }
    if paren_count != 0 {
        return Err("Unexpected end of tokens in variable name".to_string());
    }

    Ok((var_name, index))
}

fn get_var_list(tokens: &Vec<Token>, start: usize) -> Result<(Vec<Var>, usize), String> {
    if start >= tokens.len() {
        return Err("Unexpected end of token while trying to find variable list".to_string());
    }
    if tokens[start] != Token::OpenParen {
        return Err(format!("Expected ( at start of variable list at {}", start));
    }
    let mut index = start+1;
    let mut vars = Vec::new();

    loop {
        if index >= tokens.len() {
            return Err(format!("Unexpected end of tokens while parsing variable list at {}", index));
        }
        match &tokens[index] {
            Token::OpenParen => {
                let (var_name, end) = get_var_name(tokens, index)?;
                vars.push(Var(var_name));
                index = end;
            }
            Token::CloseParen => {index+=1; break},
            _ => return Err(format!("Unexpected token in variable list at {}", index)),
        }
    }

    Ok((vars, index))
}

fn get_num(tokens: &Vec<Token>, start: usize) -> Result<(u32, usize), String> {
    if start >= tokens.len()-1 {
        return Err("Unexpected end of token while trying to find number".to_string());
    }
    if tokens[start] != Token::OpenSquare  && tokens[start+1] != Token::OpenParen {
        return Err(format!("Expected [( at start of number at {}", start));
    }
    let mut index = start + 1;
    let mut num = 0;

    loop {
        if index >= tokens.len() {
            return Err(format!("Unexpected end of tokens while parsing number at {}", index));
        }
        match &tokens[index] {
            Token::OpenParen => {
                index += 1;
                if tokens[index] != Token::CloseParen {
                    return Err(format!("Expected ) after ( in number at {}", index));
                }
                num += 1;
            }
            Token::CloseSquare => {
                index+=1;
                break;
            }
            _ => return Err(format!("Unexpected token {} in number at {}", tokens[index], index)),
        }
        index += 1;
    }

    Ok((num, index))
}