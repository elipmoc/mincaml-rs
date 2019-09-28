use nom::IResult;

define_ignore_str_parser!(#[doc="not演算子のパーサ"],not_parser,"not");

define_ignore_str_parser!(#[doc="-演算子のパーサ"],minus_parser,"-");

define_ignore_str_parser!(#[doc="+演算子のパーサ"],plus_parser,"+");

define_ignore_str_parser!(#[doc="浮動小数点数用の加算演算子"],plus_dot_parser,"+.");

define_ignore_str_parser!(#[doc="浮動小数点数用の掛け算演算子"],ast_dot_parser,"*.");

define_ignore_str_parser!(#[doc="浮動小数点数用の割り算演算子"],slash_dot_parser,"/.");

define_ignore_str_parser!(#[doc="代入演算子"],equal_parser,"=");

define_ignore_str_parser!(#[doc="大小比較演算子"],less_greater_parser,"<>");
define_ignore_str_parser!(#[doc="大小比較演算子"],less_equal_parser,"<=");
define_ignore_str_parser!(#[doc="大小比較演算子"],greater_equal_parser,">=");
define_ignore_str_parser!(#[doc="大小比較演算子"],less_parser,"<");
define_ignore_str_parser!(#[doc="大小比較演算子"],greater_parser,">");

#[test]
fn not_token_parser_test() {
    let result = not_parser("not");
    assert_full_match_ok!(result,());

    let result = not_parser("not rtert");
    assert_ok!(result,());
}
