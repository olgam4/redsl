use std::collections::HashMap;

use super::{
    expression::{Expression, Operand},
    primitives::Primitive,
    values::{Error, Value},
};

pub fn evaluate(
    expr: &Expression,
    state: &Option<HashMap<String, Primitive>>,
) -> Result<Value, Error> {
    match expr {
        Expression::Sum { left, right } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left + right)),
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::Product { left, right } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left * right)),
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_value = evaluate(&condition, state)?;
            match condition_value {
                Value::Boolean(true) => evaluate(&then_branch, state),
                Value::Boolean(false) => evaluate(&else_branch, state),
                _ => Err(Error::InvalidValues(vec![condition_value])),
            }
        }
        Expression::Compare {
            left,
            operand,
            right,
        } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => match operand {
                    Operand::Equals => Ok(Value::Boolean(left == right)),
                    Operand::LessThan => Ok(Value::Boolean(left < right)),
                    Operand::GreaterThan => Ok(Value::Boolean(left > right)),
                },
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::Primitive(primitive) => match primitive {
            Primitive::Integer(value) => Ok(Value::Integer(*value)),
            Primitive::Variable(_, value) => {
                Ok(Value::Integer(*value))
            },
        },
        Expression::Let { variable, scope } => {
            match state {
                Some(state) => match &*variable.clone() {
                    Expression::Primitive(value) => match value {
                        Primitive::Variable(name, _) => {
                            let mut state = state.clone();
                            state.insert(name.to_string(), value.clone());
                            evaluate(&scope, &Some(state))
                        }
                        _ => return Err(Error::InvalidExpression(expr.clone())),
                    },
                    _ => return Err(Error::InvalidExpression(expr.clone())),
                },
                None => {
                    let mut state = HashMap::new();
                    let variable = match &**variable {
                        Expression::Primitive(value) => value,
                        _ => return Err(Error::InvalidExpression(expr.clone())),
                    };
                    match variable {
                        Primitive::Variable(name, _) => {
                            state.insert(name.clone(), variable.clone());
                            evaluate(&scope, &Some(state))
                        }
                        _ => return Err(Error::InvalidExpression(expr.clone())),
                    }
                }
            }
        }
        Expression::Use { variable } => match state {
            Some(state) => match state.get(&variable.to_string()) {
                Some(value) => {
                    evaluate(&Expression::Primitive(value.clone()), &Some(state.clone()))
                }
                None => Err(Error::UndefinedVariable(variable.clone())),
            },
            None => Err(Error::InvalidExpression(expr.clone())),
        },
    }
}
