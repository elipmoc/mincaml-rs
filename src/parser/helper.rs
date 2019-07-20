use crate::syntax::Syntax;

pub fn create_not_syntax(e: Syntax) -> Syntax {
    Syntax::Not(Box::new(e))
}


pub fn create_minus_syntax(e: Syntax) -> Syntax {
    match e {
        Syntax::Float(f) => Syntax::Float(-f),
        e => Syntax::Neg(Box::new(e))
    }
}

pub fn create_plus_syntax((e1, e2): (Syntax, Syntax)) -> Syntax {
    Syntax::Add(Box::new(e1), Box::new(e2))
}