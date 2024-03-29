use crate::{
    parser::ast::expression::{PrimaryNode, Expression},
    interpreter::{
        runtime_exception::{
            InterpreterException::{
                self,
                *
            },
            RuntimeError,
            RuntimeErrorTag::*
        },
        value::Value,
        env::Env,
        statement::Executable,
    },
    lexer::token::{
        TokenTag,
        Token
    },
    errors::Span
};

use super::evaluatable::Evaluatable;

impl Evaluatable for PrimaryNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self {
            PrimaryNode::Literal(token) => literal(token),
            PrimaryNode::Paren {
                lparen: _,
                rparen: _,
                expr
            } => paren(env, expr),
            PrimaryNode::Identifier(token) => identifier(env, token),
            PrimaryNode::Call {
                name,
                args,
                rparen: _
            } => call(name, env, args),
        }
    }
}

/// Evaluate function call
fn call(name: &Token, env: &mut Env, args: &Vec<Box<Expression>>) -> Result<Value, InterpreterException> {
    match &name.tag {
        TokenTag::Identifier(s) => match env.get(&s) {
            Some(Value::Function {
                params,
                name:_,
                body
            }) => {
                let mut global_env = env.to_owned();
                let closure = &mut global_env.enter();

                for i in 0..params.len() {
                    match &params[i].tag {
                        TokenTag::Identifier(name) => {
                            let definition_result = closure.define(
                                &name,
                                args[i].eval(&mut env.to_owned())?
                            );

                            match definition_result {
                                Err(err_tag) => {
                                    return Err(InterpreterException::Fatal(
                                        RuntimeError {
                                            span: params[i].clone().into(),
                                            tag: err_tag
                                        }
                                    ))
                                },
                                _ => ()
                            }
                        },
                        _ => unreachable!()
                    }
                }

                match body.as_ref().run(closure) {
                    Err(InterpreterException::Return(value)) => {
                        closure.leave();
                        *env = global_env;
                        return Ok(value.val);
                    }
                    _ => { Ok(Value::Null) }
                }
            }
            // Value is not a functional
            Some(_) => return Err(Fatal(
                RuntimeError {
                    span: Span::from(name.clone()),
                    tag: ObjectIsNotCallable
                }
            )),
            // Nothing found
            None => return Err(Fatal(
                RuntimeError {
                    span: Span::from(name.clone()),
                    tag: FunctionNotDefined { name: name.get_lexeme() }
                }
            ))
        }
        _ => unreachable!()
    }
}

/// Evaluate identifier
fn identifier(env: &mut Env, token: &Token) -> Result<Value, InterpreterException> {
    match &token.tag {
        TokenTag::Identifier(name) => match env.get(&name) {
            Some(val) => return Ok(val.clone()),
            None => return Err(Fatal(
                RuntimeError {
                    span: Span::from(token.clone()),
                    tag: NameNotDefined {
                        name: name.clone()
                    }
                }
            ))
        },
        _ => unreachable!()
    }
}

/// Evaluate literal value
fn literal(token: &Token) -> Result<Value, InterpreterException> {
    let value = match token.tag.clone() {
        TokenTag::Number(n) => Value::Number(n),
        TokenTag::String(s) => Value::String(Box::new(s.into())),
        _ => unreachable!()
    };

    Ok(value)
}

/// Evaluate parenthesized expression
fn paren(env: &mut Env, expr: &Box<Expression>) -> Result<Value, InterpreterException> {
    Ok(expr.eval(env)?)
}
