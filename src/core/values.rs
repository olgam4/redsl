use crate::expression::{Expression, Operand};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i32),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidOperand(Operand),
    InvalidExpression(Expression),
    InvalidValues(Vec<Value>),
    UndefinedVariable(String),
}
