//変数名
#[derive(Debug, PartialEq,Clone)]
pub struct VarId(String);

//トップレベル関数やグローバル配列のラベル
pub struct Label(String);

//文字列の要素を結合する
pub fn pp_list(xs: &Vec<String>) -> String {
    xs.connect(" ")
}

#[test]
fn pp_list_test() {
    assert_eq!(pp_list(&vec![]), "".to_string());
    assert_eq!(pp_list(&vec!["a".to_string()]), "a".to_string());
    assert_eq!(pp_list(&vec!["a".to_string(), "b".to_string()]), "a b".to_string());
}

//識別子を生成する構造体
struct IdGenerator {
    counter: u32
}

use crate::ty::Ty;

impl IdGenerator {
    //IdGenerator生成
    pub fn new() -> IdGenerator { IdGenerator { counter: 0 } }

    //ベースとなる識別子を受け取り、一意な識別子を生成する
    pub fn gen_id(&mut self, s: String) -> String {
        format!("{}.{}", s, self.count_up())
    }

    //型を持った一意な識別子を生成する
    pub fn gen_tmp(&mut self, ty: &Ty) -> String {
        format!("T{}.{}", id_of_type(ty), self.count_up())
    }

    //内部カウンターをカウントアップ
    fn count_up(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}


#[test]
fn gen_id_test() {
    let mut id_generator = IdGenerator::new();

    let result = id_generator.gen_id("aab".to_string());
    assert_eq!(result, "aab.1".to_string());

    let result = id_generator.gen_id("cca".to_string());
    assert_eq!(result, "cca.2".to_string());
}

#[test]
fn gen_tmp_test() {
    let mut id_generator = IdGenerator::new();

    let result = id_generator.gen_tmp(&Ty::Unit);
    assert_eq!(result, "Tu.1".to_string());

    let result = id_generator.gen_tmp(&Ty::Bool);
    assert_eq!(result, "Tb.2".to_string());
}


//型から識別文字列を得る
fn id_of_type(ty: &Ty) -> &'static str {
    match ty {
        Ty::Unit => "u",
        Ty::Bool => "b",
        Ty::Int => "i",
        Ty::Float => "d",
        Ty::Fun(_, _) => "f",
        Ty::Tuple(_) => "t",
        Ty::Array(_) => "a",
        Ty::Var(_) => panic!("error!")
    }
}

#[test]
fn id_of_type_test() {
    let result = id_of_type(&Ty::Int);
    assert_eq!(result, "i");

    let result = id_of_type(&Ty::Fun(Box::new(Ty::Int), vec![Box::new(Ty::Float)]));
    assert_eq!(result, "f");
}