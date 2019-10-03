use crate::parser::syntax_creator::SyntaxCreator;
use crate::syntax_impl::SyntaxImpl;

#[derive(Copy, Clone)]
pub struct SyntaxCreatorImpl;

impl SyntaxCreator for SyntaxCreatorImpl {
    type S = SyntaxImpl;

    fn create_unit(&self) -> SyntaxImpl {
        SyntaxImpl::Unit
    }

    fn create_bool(&self, v: bool) -> SyntaxImpl {
        SyntaxImpl::Bool(v)
    }

    fn create_int(&self, v: i32) -> SyntaxImpl {
        SyntaxImpl::Int(v)
    }

    fn create_float(&self, v: f32) -> SyntaxImpl {
        SyntaxImpl::Float(v)
    }

    fn create_ignore_var(&self) -> SyntaxImpl {
        SyntaxImpl::IgnoreVar
    }

    fn create_get(&self, s1: SyntaxImpl, s2: SyntaxImpl) -> SyntaxImpl {
        SyntaxImpl::Get(Box::new(s1), Box::new(s2))
    }

    fn create_add(&self, s1: SyntaxImpl, s2: SyntaxImpl) -> SyntaxImpl {
        SyntaxImpl::Add(Box::new(s1), Box::new(s2))
    }

    fn create_minus(&self, s: Self::S) -> Self::S {
        match s {
            SyntaxImpl::Float(f) => SyntaxImpl::Float(-f),
            e => SyntaxImpl::Neg(Box::new(e))
        }
    }

    fn create_not(&self, s: Self::S) -> Self::S {
        SyntaxImpl::Not(Box::new(s))
    }
}