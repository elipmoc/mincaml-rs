// 特定の文字列にマッチしてunitを返すパーサを簡易定義できるマクロ
macro_rules! define_ignore_str_parser {
    ($(#[$attr:meta])*,$f_name:ident,$s:expr) => {

        $(#[$attr])*
        pub fn $f_name(s:&str)->IResult<&str,()>{
            use nom::bytes::complete::tag;
            use nom::combinator::value;
            value((),tag($s))(s)
        }
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


