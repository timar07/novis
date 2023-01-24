use std::string::String;
use crate::{lexer::token::{Token, TokenTag::*}, errors::DebugInfo};
use super::{ast::expression::{
    Expression, PrimaryNode, UnaryNode, BinaryNode,
}, parse_error::ParseError, token_stream::TokenStream};

pub struct ExpressionParser {
    tokens: TokenStream,
}

impl ExpressionParser {
    pub fn new(tokens: TokenStream) -> ExpressionParser {
        ExpressionParser {
            tokens: tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ParseError> {
        let result = self.expression();
        match result {
            Ok(ast) => {
                println!("{:#}", ast.as_ref());
                Ok(ast)
            },
            Err(error) => Err(error),
        }
    }

    // * Grammary Rules

    fn expression(&mut self) -> Result<Box<Expression>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expression>, ParseError> {
        // equality = comparison (('!=' | '==') comparison)*;
        let mut expr = self.comparison();

        while self.tokens.match_next(&[BangEqual, EqualEqual]) {

            let node = Expression::Binary(
                BinaryNode {
                    op: self.tokens.prev().expect("Expected equality operator"),
                    left: expr?,
                    right: self.comparison()?,
                }
            );
            expr = Ok(Box::new(node))
        };

        expr
    }

    fn comparison(&mut self) -> Result<Box<Expression>, ParseError> {
        // comparison = term (('<' | '>' | '<=' | '>=') term)*;
        let mut expr = self.term();

        while self.tokens.match_next(
            &[Less, Greater, LessEqual, GreaterEqual]
        ) {

            let node = Expression::Binary(
                BinaryNode {
                    op: self.tokens.prev().unwrap(),
                    left: expr?,
                    right: self.term()?,
                }
            );

            expr = Ok(Box::new(node));
        }

        expr
    }

    fn term(&mut self) -> Result<Box<Expression>, ParseError> {
        // term = factor (('+' | '-') factor)*;
        let mut expr = self.factor();

        while self.tokens.match_next(&[Plus, Minus]) {

            let node = Expression::Binary(
                BinaryNode {
                    op: self.tokens.prev().expect("Expected binary operator"),
                    left: expr?,
                    right: self.factor()?,
                }
            );

            expr = Ok(Box::new(node));
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Box<Expression>, ParseError> {
        // factor = unary (('*' | '/') unary)*;
        let mut expr = self.exponent();

        while self.tokens.match_next(&[Star, Slash]) {
            let node = Expression::Binary (
                BinaryNode {
                    op: self.tokens.prev().expect("Expected binary operator"),
                    left: self.exponent()?,
                    right: expr?,
                }
            );

            expr = Ok(Box::new(node));
        }

        return expr;
    }

    // TODO: Avoid recursion
    fn exponent(&mut self) -> Result<Box<Expression>, ParseError> {
        // exponent = unary (('^') unary)*;
        let mut expr = self.unary();

        if self.tokens.match_next(&[Circ]) {
            let node = Expression::Binary(
                BinaryNode {
                    op: self.tokens.prev().unwrap(),
                    left: expr?,
                    right: self.exponent()?,
                }
            );
            expr = Ok(Box::new(node))
        };

        expr
    }

    fn unary(&mut self) -> Result<Box<Expression>, ParseError> {
        // unary = '-' primary;
        if self.tokens.match_next(&[Minus]) {
            let node = Expression::Unary(
                UnaryNode {
                    op: self.tokens.prev().expect("Expected unary operator"),
                    left: self.primary()?,
                }
            );

            return Ok(Box::new(node));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expression>, ParseError> {
        if self.tokens.current().is_none() {
            panic!("Unexpected EOF");
        }

        let node = match self.tokens.accept().unwrap().tag {
            Number(n) => PrimaryNode::Literal(n),
            LeftParen => {
                let node = PrimaryNode::Paren(self.expression()?);
                self.tokens
                    .require(&[RightParen])
                    .expect("Expected ')'");

                node
            },
            _ => return Err(ParseError {
                msg: String::from("Expected primary value"),
                token: self.tokens.prev().unwrap()
            })
        };

        Ok(Box::new(Expression::Primary(node)))
    }
}
