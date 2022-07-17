use crate::core::{expression, interpretation::interpret, primitives::Primitive};

#[macro_use]
pub mod core;

pub fn main() {
    let expr = expr!(Let {
        variable: expr_from!(Primitive::Variable("x".to_string(), 32)),
        scope: expr!(Sum {
            left: expr!(Use {
                variable: "x".to_string()
            }),
            right: expr_from!(Primitive::Integer(20)),
        }),
    });
    println!("{:?}", expr);

    let result = interpret(expr);

    println!("{:?}", result);
}
