use std::rc::Rc;

use crate::{
    lexer::token::{
        TokenTag
    },
    parser::{
        ast::statement::Statement,
        token_stream::TokenStream,
        parse_error::ParseError
    }
};
use super::{expression::expression};

/// This function implements statement parsing
/// # Arguments
/// * `tokens` - Stream of the tokens
///
/// All available statements is defeined here
///
pub fn statement(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let token = tokens.accept();

    let stmt = match token.tag {
        TokenTag::Print => print(tokens),
        TokenTag::If => cond(tokens),
        TokenTag::Loop => r#loop(tokens),
        TokenTag::LeftCurly => group(tokens),
        TokenTag::Let => var_definition(tokens),
        TokenTag::Func => func_definition(tokens),
        _ => {
            if tokens.current().tag == TokenTag::LeftParen {
                tokens.discard(); // push token back
                expr_stmt(tokens)
            } else {
                assignment(tokens)
            }
        }
    };

    // dbg!(&stmt);

    match tokens.require(&[TokenTag::Semicolon]) {
        Err(_) => {
            if tokens.prev().tag == TokenTag::RightCurly {
                return stmt;
            }

            Err(ParseError {
                token: tokens.current().clone(),
                msg: "Expected semicolon after statement".into()
            })
        },
        _ => stmt
    }
}

/// # Rule
/// Function definition matches following grammary:
/// ```
/// func = 'func' identifier (params) -> statement;
/// params = param (',' param)*;
/// param = identifier;
/// ```
fn func_definition(
    tokens: &mut TokenStream
) -> Result<Statement, ParseError> {
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => return Err(ParseError {
            token: tokens.current().clone(),
            msg: "Expected identifier".to_string()
        })
    };

    tokens.require(&[TokenTag::LeftParen])?;

    let mut params = vec![];

    // loop {
    //     match tokens.current().tag {
    //         TokenTag::Identifier(_) => params.push(tokens.accept().clone()),
    //         TokenTag::RightParen => {
    //             tokens.accept();
    //             break;
    //         },
    //         _ => return Err(ParseError {
    //             msg: format!("Unexpected token `{:?}`", tokens.current().tag),
    //             token: tokens.current().clone()
    //         })
    //     };
    // }

    tokens.require(&[TokenTag::RightParen])?;
    tokens.require(&[TokenTag::ArrowRight])?;

    let body = statement(tokens)?;

    Ok(Statement::Func {
        name: identifier,
        params: params,
        body: Rc::new(body)
    })
}

/// # Rule
/// Loop statement matches following grammary:
/// ```
/// loop = 'loop' expression group;
/// ```
fn r#loop(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let condition = expression(tokens);
    let body = statement(tokens);

    Ok(Statement::Loop {
        condition: condition?,
        body: Box::new(body?)
    })
}

/// # Rule
/// Conditional statement matches following grammary:
/// ```ebnf
/// cond = 'if' expression group ('else' group)?;
/// ```
fn cond(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let condition = expression(tokens);
    let if_block = statement(tokens);
    let mut else_block = None;

    if tokens.match_next(&[TokenTag::Else]) {
        else_block = Some(Box::new(statement(tokens)?));
    }

    Ok(Statement::Cond {
        condition: condition?,
        if_block: Box::new(if_block?),
        else_block: else_block
    })
}

/// # Rule
/// Group statement matches following grammary:
/// ```ebnf
/// group = '{' statement* '}' ';';
/// ```
fn group(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let mut group = vec![];

    while tokens.current().tag != TokenTag::RightCurly {
        group.push(statement(tokens)?);
    }

    tokens.accept();

    Ok(Statement::Group(group))
}

/// # Rule
/// Print statement matches following grammary:
/// ```ebnf
/// print = 'print' expression ';';
/// ```
fn print(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    Ok(Statement::Print {
        expr: expression(tokens)?,
    })
}

/// # Rule
/// Variable definition matches following grammary:
/// ```
/// define = 'let' identifier '=' expression ';';
/// ```
fn var_definition(
    tokens: &mut TokenStream
) -> Result<Statement, ParseError> {
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => return Err(ParseError {
            token: tokens.current().clone(),
            msg: "Expected identifier name after `let` keyword".to_string()
        })
    };

    tokens.require(&[TokenTag::Equal])?;

    Ok(Statement::Let {
        name: identifier,
        expr: expression(tokens)?
    })
}

/// # Rule
/// Variable assignment matches following grammary:
/// ```
/// assign = identifier '=' expression ';';
/// ```
fn assignment(
    tokens: &mut TokenStream,
) -> Result<Statement, ParseError> {
    let identifier = tokens.prev().clone();

    tokens.require(&[TokenTag::Equal])?;

    Ok(Statement::Assignment {
        name: identifier,
        expr: expression(tokens)?
    })
}

/// # Rule
/// Expression statement matches following grammary:
/// ```
/// expr_stmt = expression ';';
/// ```
fn expr_stmt(
    tokens: &mut TokenStream,
) -> Result<Statement, ParseError> {
    Ok(Statement::Expression {
        expr: expression(tokens)?
    })
}
