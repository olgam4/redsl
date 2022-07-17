use std::rc::Rc;

use super::primitives::Primitive;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Equals,
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Primitive(Primitive),
    Sum {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    Product {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    If {
        condition: ExpressionRef,
        then_branch: ExpressionRef,
        else_branch: ExpressionRef,
    },
    Compare {
        left: ExpressionRef,
        operand: Operand,
        right: ExpressionRef,
    },
    Let {
        variable: ExpressionRef,
        scope: ExpressionRef,
    },
    Use {
        variable: String,
    },
}
pub type ExpressionRef = Rc<Expression>;
