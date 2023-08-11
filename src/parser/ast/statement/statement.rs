use std::rc::Rc;

use crate::{
    parser::ast::expression::Expression,
    lexer::token::Token, errors::Span
};

/// Statement representation.
#[derive(Debug)]
pub enum Statement {
    Group(Group),
    Expr(ExprStatment),
    Retrun(Return),
    Print(Print),
    Let(Let),
    Func(Func),
    Cond(Cond),
    Loop(Loop),
    Assign(Assignment),
}

/// Group statement representation.
/// ```text
/// {    <stmts>   }
/// ^ - lcurly     ^ - rcurly
/// ```
#[derive(Debug)]
pub struct Group {
    pub stmts: Vec<Statement>,
    pub lcurly: Token,
    pub rcurly: Token
}

impl From<Group> for Span {
    fn from(stmt: Group) -> Self {
        Self {
            start: stmt.lcurly,
            end: stmt.rcurly
        }
    }
}

/// Expression statement representation.
/// ```text
/// <expr>
/// ```
#[derive(Debug)]
pub struct ExprStatment {
    pub expr: Box<Expression>,
}

impl From<ExprStatment> for Span {
    fn from(stmt: ExprStatment) -> Self {
        Span::from(stmt.expr.as_ref().clone())
    }
}

/// Return statement representation.
/// ```text
/// return <expr>
/// ^^^^^^ - keyword
/// ```
#[derive(Debug)]
pub struct Return {
    pub keyword: Token,
    pub expr: Box<Expression>,
}

impl From<Return> for Span {
    fn from(stmt: Return) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(stmt.expr.as_ref().clone()).end
        }
    }
}

/// Variable declaration statement representation.
/// ```text
/// let <name>     <- <expr>
/// ^^^ - keyword  ^^ - operator
/// ```
#[derive(Debug)]
pub struct Let {
    pub keyword: Token,
    pub name: Token,
    pub operator: Token,
    pub expr: Box<Expression>
}

impl From<Let> for Span {
    fn from(stmt: Let) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(stmt.expr.as_ref().clone()).end
        }
    }
}

/// Assignment statement representation.
/// ```text
/// <name> <- <expr>
///        ^^ - operator
/// ```
#[derive(Debug)]
pub struct Assignment {
    pub operator: Token,
    pub name: Token,
    pub expr: Box<Expression>
}

impl From<Assignment> for Span {
    fn from(stmt: Assignment) -> Self {
        Self {
            start: stmt.name,
            end: Span::from(stmt.expr.as_ref().clone()).end
        }
    }
}

/// Function definition representation
/// ```text
/// func <name><params> -> <body>
/// ^^^^ - keyword
/// ```
#[derive(Debug)]
pub struct Func {
    pub keyword: Token,
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Rc<Group>
}

impl From<Func> for Span {
    fn from(stmt: Func) -> Self {
        Span::from(stmt.keyword)
    }
}

/// Loop statement representation
/// ```text
/// loop <condition> <body>
/// ^^^^ - keyword
/// ```
#[derive(Debug)]
pub struct Loop {
    pub keyword: Token,
    pub condition: Box<Expression>,
    pub body: Box<Group>
}

impl From<Loop> for Span {
    fn from(stmt: Loop) -> Self {
        Span::from(stmt.keyword)
    }
}

/// If statement representation
/// ```text
/// if <condition> <if_block> <else_block>?
/// ^^ - keyword
/// ```
#[derive(Debug)]
pub struct Cond {
    pub keyword: Token,
    pub condition: Box<Expression>,
    pub if_block: Box<Group>,
    pub else_block: Option<Box<Group>>
}

impl From<Cond> for Span {
    fn from(stmt: Cond) -> Self {
        Span::from(stmt.keyword)
    }
}

/// Print statement representation.
/// This is very simple statement that allows you
/// to output values in ouput stream.
/// ```text
/// print <expr>
/// ^^^^^ - keyword
/// ```
#[derive(Debug)]
pub struct Print {
    pub keyword: Token,
    pub expr: Box<Expression>,
}

impl From<Print> for Span {
    fn from(stmt: Print) -> Self {
        Span::from(stmt.keyword)
    }
}
