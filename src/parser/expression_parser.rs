use crate::{
    lexer::token::TokenTag::*,
};
use super::{
    ast::expression::{
        Expression,
        PrimaryNode,
        UnaryNode,
        BinaryNode,
    },
    parse_error::ParseError,
    token_stream::TokenStream
};


/// This function implements expression parsing
/// # Arguments
/// * `tokens` - Stream of the tokens
pub fn expression(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    equality(tokens)
}

/// # Rule
/// ```
/// equality = comparison (('!=' | '==') comparison)*;
/// ```
fn equality(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = comparison(tokens);

    while tokens.match_next(&[BangEqual, EqualEqual]) {

        let node = Expression::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: comparison(tokens)?,
            }
        );
        expr = Ok(Box::new(node))
    };

    expr
}

/// # Rule
/// ```
/// comparison = term (('<' | '>' | '<=' | '>=') term)*;
/// ```
fn comparison(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = term(tokens);

    while tokens.match_next(
        &[Less, Greater, LessEqual, GreaterEqual]
    ) {

        let node = Expression::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: term(tokens)?,
            }
        );

        expr = Ok(Box::new(node));
    }

    expr
}

/// # Rule
/// ```
/// term = factor (('+' | '-') factor)*;
/// ```
fn term(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = factor(tokens);

    while tokens.match_next(&[Plus, Minus]) {

        let node = Expression::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: factor(tokens)?,
            }
        );

        expr = Ok(Box::new(node));
    }

    return expr;
}

fn factor(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    // factor = unary (('*' | '/') unary)*;
    let mut expr = exponent(tokens);

    while tokens.match_next(&[Star, Slash]) {
        let node = Expression::Binary (
            BinaryNode {
                op: tokens.prev().clone(),
                left: exponent(tokens)?,
                right: expr?,
            }
        );

        expr = Ok(Box::new(node));
    }

    return expr;
}

/// # Rule
/// ```
/// exponent = unary (('^') unary)*;
/// ```
fn exponent(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = unary(tokens);

    if tokens.match_next(&[Circ]) {
        let node = Expression::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: exponent(tokens)?, // TODO: Avoid recursion
            }
        );
        expr = Ok(Box::new(node))
    };

    expr
}

/// # Rule
/// ```
/// unary = '-' primary;
/// ```
fn unary(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    if tokens.match_next(&[Minus]) {
        let node = Expression::Unary(
            UnaryNode {
                op: tokens.prev().clone(),
                left: primary(tokens)?,
            }
        );

        return Ok(Box::new(node));
    }

    primary(tokens)
}

/// # Rule
/// ```
/// primary = literal;
/// ```
fn primary(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let node = match tokens.accept().tag {
        Number(n) => PrimaryNode::Literal(n),
        Identifier(_) => PrimaryNode::Identifier(tokens.prev().clone()),
        LeftParen => {
            let node = PrimaryNode::Paren(expression(tokens)?);
            tokens
                .require(&[RightParen])
                .expect("Expected ')'");

            node
        },
        _ => return Err(ParseError {
            msg: "Expected expression".into(),
            token: tokens.prev().clone()
        })
    };

    Ok(Box::new(Expression::Primary(node)))
}
