// mincamlの型を表す
#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Unit,
    Bool,
    Int,
    Float,
    Fun(Box<Ty>, Vec<Box<Ty>>),
    Tuple(Vec<Box<Ty>>),
    Array(Box<Ty>),
    Var(Option<Box<Ty>>),
}

impl Ty {
    // 型未定義の型変数を生成する
    pub fn gen_var_ty() -> Ty {
        Ty::Var(Option::None)
    }
}

#[test]
fn gen_var_ty_test() {
    assert_eq!(Ty::gen_var_ty(), Ty::Var(Option::None));
}
