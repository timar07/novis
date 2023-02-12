use crate::{
    interpreter::{
        runtime_error::{InterpreterException},
        value::Value,
        env::Env,
        expression::{
            Evaluatable
        },
        utils::check_condition
    },
    parser::ast::statement::{
        Print,
        Return,
        Group,
        Let,
        Loop,
        Cond,
        Func,
        Assignment,
        Statement,
        ExprStatment
    },
    lexer::token::TokenTag
};

use super::Executable;


impl Executable for Statement {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self {
            Statement::Group(group) => group.run(env),
            Statement::Retrun(ret) => ret.run(env),
            Statement::Print(print) => print.run(env),
            Statement::Let(r#let) => r#let.run(env),
            Statement::Func(func) => func.run(env),
            Statement::Cond(cond) => cond.run(env),
            Statement::Loop(r#loop) => r#loop.run(env),
            Statement::Assign(assign) => assign.run(env),
            Statement::Expr(expr) => expr.run(env),
            Statement::Noop => Ok(Value::Null)
        }
    }
}

impl Executable for Func {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self.name.tag.clone() {
            TokenTag::Identifier(id) => {
                env.define(&id, Value::Function {
                    params: self.params.clone(),
                    name: self.name.clone(),
                    body: self.body.clone()
                }).unwrap();
            }
            _ => unreachable!()
        };
        Ok(Value::Null)
    }
}

impl Executable for ExprStatment {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self.expr.eval(env) {
            Ok(_) => Ok(Value::Null),
            Err(error) => Err(error),
        }
    }
}

impl Executable for Assignment {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self.name.tag.clone() {
            TokenTag::Identifier(id) => {
                let lval = env.get(&id);
                if lval.is_some() {
                    let rval = self.expr.eval(env)?;

                    match self.operator.tag {
                        TokenTag::Equal => env.set(&id, rval).unwrap(),
                        _ => unreachable!()
                    }
                } else {
                    todo!()
                }
            }
            _ => unreachable!()
        };
        Ok(Value::Null)
    }
}

impl Executable for Let {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self.name.tag.clone() {
            TokenTag::Identifier(id) => {
                let val = self.expr.eval(env)?;
                env.define(&id, val).unwrap();
            }
            _ => unreachable!()
        };

        Ok(Value::Null)
    }
}

impl Executable for Loop {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        while check_condition(env, &self.condition)? {
            self.body.run(env)?;
        }

        Ok(Value::Null)
    }
}

impl Executable for Cond {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        if check_condition(env, &self.condition).unwrap() {
            self.if_block.run(env)?;
        } else if let Some(else_block) = &self.else_block {
            else_block.run(env)?;
        }

        Ok(Value::Null)
    }
}

impl Executable for Print {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self.expr.eval(env) {
            Ok(val) => println!("{}", val.to_string()?),
            Err(err) => return Err(err),
        };

        Ok(Value::Null)
    }
}

impl Executable for Return {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        Err(InterpreterException::Return(self.expr.eval(env)?))
    }
}

impl Executable for Group {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        let new_env = env.enter();

        for item in &self.stmts {
            match item.run(new_env) {
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
}
