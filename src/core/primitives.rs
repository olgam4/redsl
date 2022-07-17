#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Integer(i32),
    Variable(String, i32),
}
