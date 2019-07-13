// 特定の文字列にマッチしてunitを返すパーサを簡易定義できるマクロ
macro_rules! define_ignore_str_parser {
    ($f_name:ident,$s:expr) => {
        named!(pub $f_name<()>,do_parse!(
            tag!($s)>>(())
        ));
    };
}

mod comment;
mod number;
mod boool;
mod op;
mod paren;
mod etc;

pub use comment::*;
pub use number::*;
pub use boool::*;
pub use op::*;
pub use paren::*;
pub use etc::*;


