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
use super::{expression_parser::expression};

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
        TokenTag::LeftCurly => group(tokens),
        TokenTag::Let => var_definition(tokens),
        _ => return Err(ParseError {
            token: token.clone(),
            msg: "Expected statement".to_string()
        })
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
/// Conditional statement matches following grammary:
/// ```
/// cond = 'if' expression group ('else' group)?;
/// ```
fn cond(tokens: &mut TokenStream) -> Result<Statement, ParseError> {
    let condition = expression(tokens);
    let if_block = statement(tokens);
    Ok(Statement::Cond {
        condition: condition?,
        if_block: Box::new(if_block?)
    })
}

/// # Rule
/// Group statement matches following grammary:
/// ```
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
/// ```
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

    tokens
        .require(&[TokenTag::Equal])
        .expect("Expected assignment");

    Ok(Statement::Let {
        name: identifier,
        expr: expression(tokens)?
    })
}
