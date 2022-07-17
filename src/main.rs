use crate::core::{expression, interpretation::interpret, primitives};

#[macro_use]
pub mod core;

pub fn main() {
    let expression = expr!(Let {
        variable: var!("x".to_string(), 1),
        scope: expr!(Sum {
            left: expr!(Use { variable: "x".to_string() }),
            right: expr!(Product {
                left: expr_from!(Integer(2)),
                right: expr_from!(Integer(3)),
            }),
        }),
    });

    let _ = interpret(expression);
}
