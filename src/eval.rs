use crate::parse::{Expression, Var};
use std::{collections::HashMap};

pub struct VarTable {
    pub variables: HashMap<Var, u32>,
}

#[derive(Clone)]
pub struct Function {
    args: Vec<Var>,
    body: Expression,
}

pub struct FunctionTable {
    pub functions: HashMap<Var, Function>,
}

// pub struct CallStack {
//     stack: Vec<VarTable>,
// }

pub fn eval_expression(
    exp: &Expression, 
    funcs: &mut FunctionTable, 
    vars: &mut VarTable, 
    // call_stack: &mut CallStack 
) -> Result<u32, String> {
    match exp {
        Expression::Val(num) => Ok(*num),

        Expression::If(cond_exp, then_exp, else_exp) => {
            let cond_val = eval_expression(cond_exp.as_ref(), funcs, vars)?;
            if cond_val != 0 {
                eval_expression(then_exp.as_ref(), funcs, vars)
            } else {
                eval_expression(else_exp.as_ref(), funcs, vars)
            }
        }

        Expression::FunctionDef(var, args, body) => {
            if args.len() == 0 {
                let result = eval_expression(body, funcs, vars)?;
                vars.variables.entry(var.clone()).or_insert_with(|| { result });
                return Ok(result)
            }
            else {
                funcs.functions.insert(var.clone(), Function {
                    args: args.clone(),
                    body: *(*body).clone(),
                });
                return Ok(0)
            }
        }

        Expression::FunctionCall(var, args) => {
            if (var.0 == "*" || var.0 == "&") && let Expression::ListExp(args) = *args.clone() {
                if args.len() != 1 {
                    return Err(format!("{} operator expects exactly one argument, got {}", var.0, args.len()));
                }
                let arg_val = eval_expression(&args[0], funcs, vars)?;
                if var.0 == "*" {
                    if arg_val == 0 {
                        return Err("Cannot decrement zero".to_string());
                    }
                    return Ok(arg_val.saturating_sub(1));
                } else {
                    return Ok(arg_val.saturating_add(1));
                }
            }
            
            if *(*args) == Expression::ListExp(vec![]) {
                let val = vars.variables.get(&var);
                if val.is_none() {
                    return Err(format!("Variable {} not defined", var.0));
                }
                return Ok(*val.unwrap());
            } else {
                let func = funcs.functions.get(var).cloned();
                if func.is_none() {
                    return Err(format!("Function {} not defined", var.0));
                } 
                let func = func.unwrap();
                if let Expression::ListExp(params) = *args.clone() {

                    if params.len() != func.args.len() {
                        return Err(format!("Function {} expects {} arguments, got {}", var.0, func.args.len(), params.len()));
                    }

                    let mut local_vars = Box::new(VarTable { variables: HashMap::new() });
                    for (i, arg) in params.iter().enumerate() {
                        let arg_val = eval_expression(arg, funcs, vars)?;
                        local_vars.variables.insert(func.args[i].clone(), arg_val);
                    }

                    return eval_expression(&func.body, funcs, local_vars.as_mut());
                } else {
                    return Err(format!("Expected a list of arguments for function call, got {}", args));
                }
            }
        }

        Expression::ListExp(list) => {
            let mut result = 0;
            for item in list {
                result = eval_expression(item, funcs, vars)?;
            }
            Ok(result)
        }

        Expression::TryCatch(try_block, catch_block) => {
            match eval_expression(try_block, funcs, vars) {
                Ok(result) => Ok(result),
                Err(err) => {
                    crate::logOutput(format!("Warning!: Error caught in try block: {}\n", err).as_str());
                    eval_expression(&catch_block, funcs, vars)
                }
            }
        }

        Expression::PrintChar(exp) => {
            let val = eval_expression(exp, funcs, vars)?;
            crate::logOutput(format!("{}", char::from_u32(val).unwrap_or('?')).as_str());
            Ok(val)
        }

        Expression::PrintNum(exp) => {
            let val = eval_expression(exp, funcs, vars)?;
            crate::logOutput(format!("{}\n", val).as_str());
            Ok(val)
        }
    }
}
                       