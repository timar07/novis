use std::{rc::Rc};
use crate::{
    parser::ast::{
        statement::Statement,
        expression::{Expression}
    },
    interpreter::expression::expression, lexer::token::{TokenTag, Token}
};
use super::{
    runtime_error::RuntimeError::{
        self,
        *
    },
    env::Env,
    value::Value
};


pub fn statement(env: &mut Env, statement: &Statement) -> Result<(), RuntimeError> {
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
        Statement::Expression { expr } => {
            match expression(env, expr) {
                Ok(_) => Ok(()),
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
) -> Result<(), RuntimeError> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            let lval = env.get(&id);
            if lval.is_some() {
                let rval = expression(env, expr)?;

                match operator.tag {
                    TokenTag::Equal => env.set(&id, rval),
                    _ => unreachable!()
                }
            } else {
                return Err(NameNotDefined { name: id })
            }
        }
        _ => unreachable!()
    };
    Ok(())
}

fn func_definition(
    env: &mut Env,
    name: &Token,
    params: &Vec<Token>,
    body: &Rc<Statement>
) -> Result<(), RuntimeError> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            if env.get_local(&id).is_none() {
                env.define(id, Value::Function {
                    params: params.clone(),
                    name: name.clone(),
                    body: body.clone()
                });
            } else {
                return Err(FunctionRedefinition { name: id })
            }
        }
        _ => unreachable!()
    };
    Ok(())
}

fn var_definition(
    env: &mut Env,
    name: &Token,
    expr: &Box<Expression>
) -> Result<(), RuntimeError> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            if env.get_local(&id).is_none() {
                let val = expression(env, expr)?;
                env.define(id, val);
            } else {
                return Err(NameRedefinition { name: id })
            }
        }
        _ => unreachable!()
    };
    Ok(())
}

fn r#loop(
    env: &mut Env,
    condition: &Box<Expression>,
    body: &Box<Statement>
) -> Result<(), RuntimeError> {
    while expression(env, condition)?.to_boolean()? == Value::Boolean(true) {
        statement(env, body)?;
    }

    Ok(())
}

fn cond(
    env: &mut Env,
    condition: &Box<Expression>,
    if_block: &Box<Statement>,
    else_block: &Option<Box<Statement>>
) -> Result<(), RuntimeError> {
    if expression(env, condition)?.to_boolean()? == Value::Boolean(true) {
        statement(env, if_block)?;
    }

    if let Some(else_block) = else_block {
        statement(env, else_block)?;
    }

    Ok(())
}

fn group(env: &mut Env, items: &Vec<Statement>) -> Result<(), RuntimeError> {
    let mut new_env = Box::new(
        Env::local(
            Box::new(env.to_owned())
        )
    );

    for item in items {
        statement(new_env.as_mut(), item)?;
    };

    env.leave();

    Ok(())
}

fn print(env: &mut Env, expr: &Box<Expression>) -> Result<(), RuntimeError> {
    match expression(env, &expr) {
        Ok(val) => match val {
            Value::String(str) => println!("{str}"),
            Value::Number(n) => println!("{n}"),
            _ => {
                panic!();
                //     Err(RuntimeError {
                //     msg: format!("Cannot print value of type {:?}", val),
                //     info: expr
                // })
            }
        },
        Err(err) => return Err(err),
    };

    Ok(())
}
