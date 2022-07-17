macro_rules! expr {
    ($($x:tt)*) => {
        std::rc::Rc::new($crate::expression::Expression::$($x)*)
    };
}

macro_rules! expr_from {
    ($($x:tt)*) => {
        expr!(Primitive($($x)*))
    };
}
