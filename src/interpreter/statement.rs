use crate::{
    parser::ast::{
        statement::Statement,
        expression::Expression
    },
    interpreter::expression::expression, lexer::token::{TokenTag, Token}
};

use super::{runtime_error::RuntimeError, env::Env};


pub fn statement(env: &mut Env, statement: &Statement) -> Result<(), RuntimeError> {
    match statement {
        Statement::Print {
            expr,
        } => print(env, expr),
        Statement::Cond {
            condition,
            if_block
        } => cond(env, condition, if_block),
        Statement::Group(items) => group(env, items),
        Statement::Let {
            name,
            expr
        } => var_definition(env, name, expr)
    }
}

fn var_definition(
    env: &mut Env,
    name: &Token,
    expr: &Box<Expression>
) -> Result<(), RuntimeError> {
    match name.tag.clone() {
        TokenTag::Identifier(id) => {
            let val = expression(env, expr)?;
            env.define(id, val);
        }
        _ => unreachable!()
    };
    Ok(())
}

fn cond(
    env: &mut Env,
    condition: &Box<Expression>,
    if_block: &Box<Statement>
) -> Result<(), RuntimeError> {
    if expression(env, condition)? != 0.0 {
        statement(env, if_block)?;
    }

    Ok(())
}

fn group(env: &mut Env, items: &Vec<Statement>) -> Result<(), RuntimeError> {
    let mut new_env = Box::new(Env::local(env));

    for item in items {
        statement(new_env.as_mut(), item)?;
    };
    Ok(())
}

fn print(env: &mut Env, expr: &Box<Expression>) -> Result<(), RuntimeError> {
    match expression(env, &expr) {
        Ok(n) => println!("{n}"),
        Err(err) => return Err(err),
    };

    Ok(())
}
