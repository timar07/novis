use std::rc::Rc;
use crate::{
    lexer::token::{
        TokenTag, Token
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
        // TokenTag::Repeat => repeat(tokens),
        TokenTag::LeftCurly => group(tokens),
        TokenTag::Let => var_definition(tokens),
        TokenTag::Func => func_definition(tokens),
        TokenTag::Return => r#return(tokens),
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

            Err(ParseError::ExpectedSemicolon {
                token: tokens.current().clone(),
            })
        },
        _ => stmt
    }
}

/// # Rule
/// Function definition matches following grammary:
/// ```
/// func = 'func' identifier (params) -> statement;
/// ```
fn func_definition(
    tokens: &mut TokenStream
) -> Result<Statement, ParseError> {
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => return Err(ParseError::ExpectedIdentifier {
            token: tokens.current().clone(),
        })
    };

    let params = parse_params(tokens)?;

    tokens.require(&[TokenTag::ArrowRight])?;

    let body = statement(tokens)?;

    Ok(Statement::Func {
        name: identifier,
        params: params,
        body: Rc::new(body)
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
            _ => return Err(ParseError::UnexpectedToken {
                token: tokens.current().clone()
            })
        };
    }
}

/// # Rule
/// Return statement matches following grammary:
/// ```
/// return = 'return' expression;
/// ```
fn r#return(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    Ok(Statement::Return {
        expr: expression(tokens)?
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
/// Repeat statement matches following grammary:
/// ```ebnf
/// repeat = 'repeat' expression group;
/// ```
// fn repeat(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
//     let times = expression(tokens);
//     let body = statement(tokens);

//     Ok(Statement::Repeat {
//         times: times?,
//         body: Box::new(body?)
//     })
// }

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
/// define = 'let' identifier '<-' expression ';';
/// ```
fn var_definition(
    tokens: &mut TokenStream
) -> Result<Statement, ParseError> {
    let identifier = match tokens.current().tag {
        TokenTag::Identifier(_) => tokens.accept().clone(),
        _ => return Err(ParseError::ExpectedIdentifier {
            token: tokens.current().clone()
        })
    };

    tokens.require(&[TokenTag::ArrowLeft])?;

    Ok(Statement::Let {
        name: identifier,
        expr: expression(tokens)?
    })
}

/// # Rule
/// Variable assignment matches following grammary:
/// ```
/// assign = identifier ('=' | '+=' | '-=' | '*=' | "/=") expression ';';
/// ```
fn assignment(
    tokens: &mut TokenStream,
) -> Result<Statement, ParseError> {
    let identifier = tokens.prev().clone();

    tokens.require(&[
        TokenTag::Equal,
        TokenTag::PlusEqual,
        TokenTag::MinusEqual,
        TokenTag::StarEqual,
        TokenTag::SlashEqual
    ])?;

    Ok(Statement::Assignment {
        operator: tokens.prev().clone(),
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
