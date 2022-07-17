macro_rules! expr {
    ($($x:tt)*) => {
        std::rc::Rc::new($crate::expression::Expression::$($x)*)
    };
}


macro_rules! expr_from {
    ($($x:tt)*) => {
        expr!(Primitive(primitives::Primitive::$($x)*))
    };
}

// generate expression for a variable with a value and a name
macro_rules! var {
    ($name:expr, $value:expr) => {
        expr!(Primitive(primitives::Primitive::Variable($name, $value)))
    };
}

