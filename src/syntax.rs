use crate::id::VarId;
use crate::ty::Ty;

type BSyntax = Box<Syntax>;

//構文の構造定義
#[derive(Debug, PartialEq)]
pub enum Syntax {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f32),
    Not(BSyntax),
    Neg(BSyntax),
    Add(BSyntax, BSyntax),
    Sub(BSyntax, BSyntax),
    FNeg(BSyntax),
    FAdd(BSyntax, BSyntax),
    FSub(BSyntax, BSyntax),
    FMul(BSyntax, BSyntax),
    FDiv(BSyntax, BSyntax),
    Eq(BSyntax, BSyntax),
    LE(BSyntax, BSyntax),
    If(BSyntax, BSyntax, BSyntax),
    Let((VarId, Ty), BSyntax, BSyntax),
    Var(VarId),
    LetRec(Box<FunDef>, BSyntax),
    App(BSyntax, Vec<BSyntax>),
    Tuple(Vec<BSyntax>),
    LetTuple((VarId, Ty), Vec<BSyntax>),
    Array(BSyntax, BSyntax),
    Get(BSyntax, BSyntax),
    Put(BSyntax, BSyntax, BSyntax),
}

//関数定義
#[derive(Debug, PartialEq)]
pub struct FunDef {
    name: (VarId, Ty),
    args: Vec<(VarId, Ty)>,
    body: Syntax,
}