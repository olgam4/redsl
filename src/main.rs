use crate::core::{expression, interpretation::interpret, primitives};

#[macro_use]
pub mod core;

pub fn main() {
    interpret(expr!(Let {
        variables: vec![var!("x", expr_from!(Integer(1)))],
        scope: expr!(For {
            variable: "i".to_string(),
            from: expr_from!(Integer(0)),
            to: expr_from!(Integer(2)),
            body: expr!(Chain {
                left: expr!(Print {
                    expression: expr!(Use {
                        variable: "x".to_string(),
                    })
                }),
                right: expr!(Assign {
                    variable: "x".to_string(),
                    value: expr!(Sum {
                        left: expr!(Use {
                            variable: "x".to_string(),
                        }),
                        right: expr_from!(Integer(1)),
                    })
                }),
            }),
        }),
    }));

    interpret(expr!(For {
        variable: "i".to_string(),
        from: expr_from!(Integer(0)),
        to: expr_from!(Integer(2)),
        body: expr!(Print {
            expression: expr!(Use {
                variable: "i".to_string(),
            })
        }),
    }));

    interpret(expr!(Let {
        variables: vec!(
            var!("x", expr_from!(Integer(3))),
            var!("y", expr_from!(Integer(6)))
        ),
        scope: expr!(Product {
            left: expr!(Use {
                variable: "x".to_string(),
            }),
            right: expr!(Use {
                variable: "y".to_string(),
            }),
        }),
    }));
}
