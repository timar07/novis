use std::fmt::{
    Display, self, Binary
};

use super::expression::{
    Expression,
    BinaryNode, UnaryNode
};

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Binary(expr) => {
                write!(f, "Binary Node {:#?}", expr)
            }
            _ => write!(f, "Unknown expression")
        }
    }
}

impl fmt::Debug for BinaryNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Foo")
           .field("left", &self.left)
           .field("op", &format!("{}", self.op))
           .field("right", &self.right)
           .finish()
    }
}

impl fmt::Debug for UnaryNode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Foo")
           .field("op", &format!("{}", self.op))
           .field("left", &self.left)
           .finish()
    }
}
