use crate::{
    parser::ast::expression::{Expression},
    lexer::token::Token, errors::Span
};

pub enum Statement {
    Group(Group),
    Expr(ExprStatment),
    Retrun(Return),
    Let(Let),
    Func(Func),
}

/// Group statement representation.
/// ```text
/// {   <stmts>   }
/// ^ - lcurly    ^ - rcurly
/// ```
pub struct Group {
    stmts: Vec<Statement>,
    lcurly: Token,
    rcurly: Token
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
pub struct ExprStatment {
    expr: Box<Expression>,
}

impl From<ExprStatment> for Span {
    fn from(stmt: ExprStatment) -> Self {
        Span::from(*stmt.expr.as_ref())
    }
}

/// Return statement representation.
/// ```text
/// return <expr>
/// ^^^^^^ - keyword
/// ```
pub struct Return {
    keyword: Token,
    expr: Box<Expression>,
}

impl From<Return> for Span {
    fn from(stmt: Return) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(*stmt.expr.as_ref()).end
        }
    }
}

/// Variable declaration statement representation.
/// ```text
/// let <name>     <- <expr>
/// ^^^ - keyword  ^^ - operator
/// ```
pub struct Let {
    keyword: Token,
    name: Token,
    operator: Token,
    expr: Box<Expression>
}

impl From<Let> for Span {
    fn from(stmt: Let) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(*stmt.expr.as_ref()).end
        }
    }
}

/// Assignment statement representation.
/// ```text
/// <name> <- <expr>
///        ^^ - operator
/// ```
pub struct Assignment {
    operator: Token,
    name: Token,
    expr: Box<Expression>
}

impl From<Assignment> for Span {
    fn from(stmt: Assignment) -> Self {
        Self {
            start: stmt.name,
            end: Span::from(*stmt.expr.as_ref()).end
        }
    }
}

/// Function definition representation
/// ```text
/// func <name><params> -> <body>
/// ^^^^ - keyword
/// ```
pub struct Func {
    keyword: Token,
    name: Token,
    params: Vec<Token>,
    body: Box<Group>
}

impl From<Func> for Span {
    fn from(stmt: Func) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(*stmt.body.as_ref()).end
        }
    }
}

/// Loop statement representation
/// ```text
/// loop <condition> <body>
/// ^^^^ - keyword
/// ```
pub struct Loop {
    keyword: Token,
    condition: Box<Expression>,
    body: Box<Group>
}

impl From<Loop> for Span {
    fn from(stmt: Loop) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(*stmt.body.as_ref()).end
        }
    }
}

/// If statement representation
/// ```text
/// if <condition> <if_block> <else_block>?
/// ^^ - keyword
/// ```
pub struct Cond {
    keyword: Token,
    condition: Box<Expression>,
    if_block: Box<Group>,
    else_block: Option<Box<Group>>
}

impl From<Cond> for Span {
    fn from(stmt: Cond) -> Self {
        Self {
            start: Span::from(stmt.keyword).start,
            end: if let Some(block) = stmt.else_block {
                Span::from(*block.as_ref()).end
            } else {
                Span::from(*stmt.if_block.as_ref()).end
            }
        }
    }
}

/// Print statement representation.
/// This is very simple statement that allows you
/// to output values in ouput stream.
/// ```text
/// print <expr>
/// ^^^^^ - keyword
/// ```
pub struct Print {
    keyword: Token,
    expr: Box<Expression>,
}

impl From<Print> for Span {
    fn from(stmt: Print) -> Self {
        Self {
            start: stmt.keyword,
            end: Span::from(*stmt.expr.as_ref()).end
        }
    }
}
