use crate::token_parser::*;
use crate::syntax::Syntax;


named_attr!(#[doc="括弧をつけなくても関数の引数になれる式"],pub simple_exp_parser<Syntax>,
    alt!(
        map!(preceded!(lparen_parser,rparen_parser),|_|Syntax::Unit) |
        map!(bool_parser,Syntax::Bool) |
        map!(int_parser,Syntax::Int)
    )
);

#[test]
fn simple_exp_test() {
    let result = simple_exp_parser("true".as_bytes());
    assert_full_match_ok!(result,Syntax::Bool(true));

    let result = simple_exp_parser("false".as_bytes());
    assert_full_match_ok!(result,Syntax::Bool(false));

    let result = simple_exp_parser("448877".as_bytes());
    assert_full_match_ok!(result,Syntax::Int(448877));

    let result = simple_exp_parser("()".as_bytes());
    assert_full_match_ok!(result,Syntax::Unit);
}


named!(exp_parser<Syntax>,do_parse!(e:simple_exp_parser>>((e)) ) );

