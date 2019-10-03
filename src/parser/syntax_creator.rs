use crate::parser::syntax::Syntax;

pub trait SyntaxCreator: Clone + Copy {
    type S: Syntax;
    fn create_unit(&self) -> Self::S;
    fn create_bool(&self, v: bool) -> Self::S;
    fn create_int(&self, v: i32) -> Self::S;
    fn create_float(&self, v: f32) -> Self::S;
    fn create_ignore_var(&self) -> Self::S;
    fn create_get(&self, s1: Self::S, s2: Self::S) -> Self::S;
    fn create_add(&self, s1: Self::S, s2: Self::S) -> Self::S;
    fn create_minus(&self, s: Self::S) -> Self::S;
    fn create_not(&self, s: Self::S) -> Self::S;
}
