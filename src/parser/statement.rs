use std::rc::Rc;

use crate::{
    lexer::token::{
        TokenTag, Token
    },
    parser::{
        ast::statement::Statement,
        token_stream::TokenStream,
        parse_error::{
            ParseError,
            ParseErrorTag::*
        }
    }
};
use super::{
    ast::statement::{
        Func,
        Group,
        ExprStatment,
        Assignment,
        Let,
        Print,
        Cond,
        Loop,
        Return
    }
};
use crate::parser::expression::expression;

/// This function implements statement parsing
/// # Arguments
/// * `tokens` - Stream of the tokens
///
/// All available statements is defeined here
///
pub fn statement(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let token = tokens.accept();

    let stmt = match token.tag {
        TokenTag::Print     => Statement::Print(print(tokens)?),
        TokenTag::If        => Statement::Cond(cond(tokens)?),
        TokenTag::Loop      => Statement::Loop(r#loop(tokens)?),
        TokenTag::LeftCurly => Statement::Group(group(tokens)?),
        TokenTag::Let       => Statement::Let(var_definition(tokens)?),
        TokenTag::Func      => Statement::Func(func_definition(tokens)?),
        TokenTag::Return    => Statement::Retrun(r#return(tokens)?),
        TokenTag::Identifier(_) => {
            if tokens.match_next(&[TokenTag::ArrowLeft]) {
                Statement::Assign(assignment(tokens)?)
            } else {
                Statement::Expr(expr_stmt(tokens)?)
            }
        },
        _ => {
            tokens.discard(); // discard the token we accepted
            Statement::Expr(expr_stmt(tokens)?)
        }
    };

    if tokens.prev().tag != TokenTag::RightCurly {
        tokens.require(&[TokenTag::Semicolon])?;
    }

    Ok(stmt)
}

/// # Rule
/// Function definition matches following grammary:
/// ```ebnf
/// func = 'func' identifier (params) -> statement;
/// ```
fn func_definition(
    tokens: &mut TokenStream
) -> Result<Func, ParseError> {
    let keyword = tokens.prev().clone();
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => return Err(ParseError {
            token: tokens.current().clone(),
            tag: ExpectedIdentifier
        })
    };

    let params = parse_params(tokens)?;

    tokens.require(&[TokenTag::ArrowRight])?;

    let body = group(tokens)?;

    Ok(Func {
        keyword: keyword,
        name: identifier,
        params: params,
        body: Rc::new(body),
    })
}

/// # Rule
/// Function params matches following grammary:
/// ```ebnf
/// params = param (',' param)*;
/// param = identifier;
/// ```
fn parse_params(tokens: &mut TokenStream) -> Result<Vec<Token>, ParseError> {
    let mut params = vec![];

    tokens.require(&[TokenTag::LeftParen])?;

    loop {
        match tokens.current().tag {
            TokenTag::Identifier(_) => {
                params.push(tokens.accept().clone());

                if tokens.current().tag != TokenTag::RightParen {
                    tokens.require(&[TokenTag::Comma])?;
                }
            },
            TokenTag::RightParen => {
                tokens.accept();
                break Ok(params);
            },
            _ => return Err(ParseError {
                token: tokens.current().clone(),
                tag: UnexpectedToken
            })
        };
    }
}

/// # Rule
/// Return statement matches following grammary:
/// ```ebnf
/// return = 'return' expression ';';
/// ```
fn r#return(tokens: &mut TokenStream) -> Result<Return, ParseError> {
    Ok(Return {
        keyword: tokens.prev().clone(),
        expr: expression(tokens)?,
    })
}

/// # Rule
/// Loop statement matches following grammary:
/// ```ebnf
/// loop = 'loop' expression group;
/// ```
fn r#loop(tokens: &mut TokenStream) -> Result<Loop, ParseError> {
    Ok(Loop {
        keyword: tokens.prev().clone(),
        condition: expression(tokens)?,
        body: Box::new(group(tokens)?),
    })
}

/// # Rule
/// Conditional statement matches following grammary:
/// ```ebnf
/// cond = 'if' expression group ('else' group)?;
/// ```
fn cond(tokens: &mut TokenStream) -> Result<Cond, ParseError> {
    let keyword = tokens.prev().clone();
    let condition = expression(tokens);
    let if_block = group(tokens);
    let mut else_block = None;

    if tokens.match_next(&[TokenTag::Else]) {
        else_block = Some(Box::new(group(tokens)?));
    }

    Ok(Cond {
        keyword: keyword,
        condition: condition?,
        if_block: Box::new(if_block?),
        else_block: else_block
    })
}

/// # Rule
/// Group statement matches following grammary:
/// ```ebnf
/// group = '{' statement* '}';
/// ```
fn group(tokens: &mut TokenStream) -> Result<Group, ParseError> {
    let lcurly = tokens.prev().clone();
    let mut group = vec![];

    tokens.require(&[TokenTag::LeftCurly])?;

    while tokens.current().tag != TokenTag::RightCurly {
        group.push(statement(tokens)?);
    }

    Ok(Group {
        lcurly: lcurly,
        stmts: group,
        rcurly: tokens.accept().clone(),
    })
}

/// # Rule
/// Print statement matches following grammary:
/// ```ebnf
/// print = 'print' expression ';';
/// ```
fn print(tokens: &mut TokenStream) -> Result<Print, ParseError> {
    Ok(Print {
        keyword: tokens.prev().clone(),
        expr: expression(tokens)?,
    })
}

/// # Rule
/// Variable definition matches following grammary:
/// ```ebnf
/// define = 'let' identifier '<-' expression;
/// ```
fn var_definition(
    tokens: &mut TokenStream
) -> Result<Let, ParseError> {
    let keyword = tokens.prev().clone();
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => {
            return Err(ParseError {
                token: tokens.current().clone(),
                tag: ExpectedIdentifier
            })
        }
    };

    let operator = tokens.require(&[TokenTag::ArrowLeft])?;

    Ok(Let {
        keyword: keyword,
        name: identifier,
        operator: operator.clone(),
        expr: expression(tokens)?,
    })
}

/// # Rule
/// Variable assignment matches following grammary:
/// ```ebnf
/// assign = identifier ('<-') expression;
/// ```
fn assignment(
    tokens: &mut TokenStream,
) -> Result<Assignment, ParseError> {
    let identifier = tokens.prev().clone();

    tokens.require(&[
        TokenTag::ArrowLeft
    ])?;

    Ok(Assignment {
        name: identifier,
        operator: tokens.prev().clone(),
        expr: expression(tokens)?
    })
}

/// # Rule
/// Expression statement matches following grammary:
/// ```ebnf
/// expr_stmt = expression;
/// ```
fn expr_stmt(
    tokens: &mut TokenStream,
) -> Result<ExprStatment, ParseError> {
    Ok(ExprStatment {
        expr: expression(tokens)?
    })
}
