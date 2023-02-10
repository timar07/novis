use crate::{
    lexer::token::TokenTag::{*, self},
};
use super::{
    ast::expression::{
        Expression,
        PrimaryNode,
        UnaryNode,
        BinaryNode,
        ExpressionNode,
    },
    parse_error::{
        ParseErrorTag::*,
        ParseError
    },
    token_stream::TokenStream
};


/// This function implements expression parsing
/// # Arguments
/// * `tokens` - Stream of the tokens
pub fn expression(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    equality(tokens)
}

/// # Rule
/// ```ebnf
/// equality = comparison (('!=' | '==') comparison)*;
/// ```
fn equality(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = comparison(tokens);

    while tokens.match_next(&[BangEqual, EqualEqual]) {
        let node = ExpressionNode::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: comparison(tokens)?,
            }
        );

        expr = Ok(Expression::create(node))
    };

    expr
}

/// # Rule
/// ```ebnf
/// comparison = term (('<' | '>' | '<=' | '>=') term)*;
/// ```
fn comparison(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = term(tokens);

    while tokens.match_next(
        &[Less, Greater, LessEqual, GreaterEqual]
    ) {

        let node = ExpressionNode::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: term(tokens)?,
            }
        );

        expr = Ok(Expression::create(node));
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
        let node = ExpressionNode::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: factor(tokens)?,
            }
        );

        expr = Ok(Expression::create(node));
    }

    return expr;
}


/// # Rule
/// ```ebnf
/// factor = unary (('*' | '/') unary)*;
/// ```
fn factor(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = exponent(tokens);

    while tokens.match_next(&[Star, Slash]) {
        let node = ExpressionNode::Binary (
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: exponent(tokens)?,
            }
        );

        expr = Ok(Expression::create(node));
    }

    return expr;
}

/// # Rule
/// ```ebnf
/// exponent = unary (('^') unary)*;
/// ```
fn exponent(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let mut expr = unary(tokens);

    if tokens.match_next(&[Circ]) {
        let node = ExpressionNode::Binary(
            BinaryNode {
                op: tokens.prev().clone(),
                left: expr?,
                right: exponent(tokens)?, // TODO: Avoid recursion
            }
        );
        expr = Ok(Expression::create(node))
    };

    expr
}

/// # Rule
/// ```ebnf
/// unary = '-' primary;
/// ```
fn unary(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    if tokens.match_next(&[Minus]) {
        let node = ExpressionNode::Unary(
            UnaryNode {
                op: tokens.prev().clone(),
                left: primary(tokens)?,
            }
        );

        return Ok(Expression::create(node));
    }

    primary(tokens)
}

/// # Rule
/// ```ebnf
/// primary = literal | identifier | '(' expression ')';
/// ```
fn primary(tokens: &mut TokenStream) -> Result<Box<Expression>, ParseError> {
    let node = ExpressionNode::Primary(match &tokens.accept().tag {
        Number(_) | String(_) => PrimaryNode::Literal(tokens.prev().clone()),
        Identifier(_) => {
            match tokens.current().tag {
                TokenTag::LeftParen => call(tokens)?,
                _ => PrimaryNode::Identifier(tokens.prev().clone())
            }
        },
        LeftParen => {
            let node = PrimaryNode::Paren(expression(tokens)?);
            tokens.require(&[RightParen])?;
            node
        },
        _ => return Err(ParseError {
            token: tokens.prev().clone(),
            tag: ExpectedExpression
        })
    });

    Ok(Expression::create(node))
}

/// # Rule
/// Function call matches following grammary:
/// ```ebnf
/// call = identifier '(' args ')';
/// ```
fn call(
    tokens: &mut TokenStream,
) -> Result<PrimaryNode, ParseError> {
    let identifier = tokens.prev().clone();
    Ok(PrimaryNode::Call {
        name: identifier,
        args: parse_args(tokens)?
    })
}

/// # Rule
/// Arguments match following grammary:
/// ```ebnf
/// args = expression (',' expression)*;
/// ```
fn parse_args(
    tokens: &mut TokenStream,
) -> Result<Vec<Box<Expression>>, ParseError> {
    let mut params = vec![];

    tokens.require(&[TokenTag::LeftParen])?;

    loop {
        match tokens.current().tag {
            TokenTag::RightParen => {
                tokens.accept();
                break Ok(params);
            },
            _ => {
                params.push(expression(tokens)?);

                if tokens.current().tag != TokenTag::RightParen {
                    tokens.require(&[TokenTag::Comma])?;
                }
            }
        };
    }
}
