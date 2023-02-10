use std::{rc::Rc};
use crate::{
    parser::ast::{
        statement::Statement,
        expression::{Expression}
    },
    interpreter::expression::expression, lexer::token::{TokenTag, Token}
};
use super::{
    runtime_error::InterpreterException::{
        self,
        *
    },
    runtime_error::{
        RuntimeError,
        RuntimeErrorTag::*
    },
    env::Env,
    value::Value, utils::check_condition
};


pub fn statement(env: &mut Env, statement: &Statement) -> Result<Value, InterpreterException> {
    match statement {
        Statement::Print {
            expr,
        } => print(env, expr),
        Statement::Cond {
            condition,
            if_block,
            else_block
        } => cond(env, condition, if_block, else_block),
        Statement::Loop {
            condition,
            body
        } => r#loop(env, condition, body),
        // Statement::Repeat {
        //     times,
        //     body
        // } => repeat(env, times, body),
        Statement::Assignment {
            operator,
            name,
            expr
         } => assignment(env, name, operator, expr),
        Statement::Group(items) => group(env, items),
        Statement::Let {
            name,
            expr
        } => var_definition(env, name, expr),
        Statement::Func {
            name,
            params,
            body
        } => func_definition(env, name, params, body),
        Statement::Return {
            expr
        } => r#return(env, expr),
        Statement::Expression { expr } => {
            match expression(env, expr) {
                Ok(_) => Ok(Value::Null),
                Err(error) => Err(error),
            }
        }
    }
}

fn assignment(
    env: &mut Env,
    name: &Token,
    operator: &Token,
    expr: &Box<Expression>
) -> Result<Value, InterpreterException> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            let lval = env.get(&id);
            if lval.is_some() {
                let rval = expression(env, expr)?;

                match operator.tag {
                    TokenTag::Equal => env.set(&id, rval).unwrap(),
                    _ => unreachable!()
                }
            } else {
                return Err(Fatal(
                    RuntimeError {
                        tag: NameNotDefined { name: id }
                    }
                ))
            }
        }
        _ => unreachable!()
    };
    Ok(Value::Null)
}

fn func_definition(
    env: &mut Env,
    name: &Token,
    params: &Vec<Token>,
    body: &Rc<Statement>
) -> Result<Value, InterpreterException> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            env.define(&id, Value::Function {
                params: params.clone(),
                name: name.clone(),
                body: body.clone()
            }).unwrap();
        }
        _ => unreachable!()
    };
    Ok(Value::Null)
}

fn r#return(
    env: &mut Env,
    expr: &Box<Expression>,
) -> Result<Value, InterpreterException> {
    Err(Return(expression(env, expr)?))
}

fn var_definition(
    env: &mut Env,
    name: &Token,
    expr: &Box<Expression>
) -> Result<Value, InterpreterException> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            let val = expression(env, expr)?;
            env.define(&id, val).unwrap();
        }
        _ => unreachable!()
    };
    Ok(Value::Null)
}

// fn repeat(
//     env: &mut Env,
//     times: &Box<Expression>,
//     body: &Box<Statement>
// ) -> Result<Value, InterpreterException> {
//     let mut amount;

//     match expression(env, times)? {
//         Value::Number(n) => {
//             amount = n as usize;
//         },
//         _ => return Err(Fatal(ExpectedToBeNumber))
//     }

//     for _ in 0..amount {
//         statement(env, body)?;
//     }

//     Ok(Value::Null)
// }

fn r#loop(
    env: &mut Env,
    condition: &Box<Expression>,
    body: &Box<Statement>
) -> Result<Value, InterpreterException> {
    while expression(env, condition)?.to_boolean().unwrap() == Value::Boolean(true) {
        statement(env, body)?;
    }

    Ok(Value::Null)
}

fn cond(
    env: &mut Env,
    condition: &Box<Expression>,
    if_block: &Box<Statement>,
    else_block: &Option<Box<Statement>>
) -> Result<Value, InterpreterException> {
    if check_condition(env, condition).unwrap() {
        statement(env, if_block)?;
    } else if let Some(else_block) = else_block {
        statement(env, else_block)?;
    }

    Ok(Value::Null)
}

fn group(env: &mut Env, items: &Vec<Statement>) -> Result<Value, InterpreterException> {
    let new_env = env.enter();

    for item in items {
        match statement(new_env, item) {
            Ok(return_value) => {
                match return_value {
                    Value::Null => continue,
                    _ => return Ok(return_value)
                }
            },
            Err(err) => {
                return Err(err)
            }
        }
    };

    Ok(Value::Null)
}

fn print(env: &mut Env, expr: &Box<Expression>) -> Result<Value, InterpreterException> {
    match expression(env, &expr) {
        Ok(val) => println!("{}", val.to_string()?),
        Err(err) => return Err(err),
    };

    Ok(Value::Null)
}
