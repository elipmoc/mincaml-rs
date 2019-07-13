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
mod bool;
mod op;
mod paren;
mod etc;


