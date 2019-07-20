use crate::token_parser::*;
use crate::syntax::Syntax;
use super::helper::*;


named_attr!(#[doc="括弧で囲まなくても関数の引数になれる式の一部分"],simple_exp_part_parser<Syntax>,
    alt!(
        do_parse!(
            lparen_parser >>
            exp: exp_parser >>
            rparen_parser >>
            (exp)
        )|
        map!(preceded!(lparen_parser,rparen_parser),|_|Syntax::Unit) |
        map!(bool_parser,Syntax::Bool) |
        map!(int_parser,Syntax::Int) |
        map!(float_parser,Syntax::Float) |
        map!(ident_ignore_parser,|_|Syntax::IgnoreVar)
    )
);

named_attr!(#[doc="括弧で囲まなくても関数の引数になれる式"],pub simple_exp_parser<Syntax>,
    do_parse!(
        e1: simple_exp_part_parser >>
        e: alt!(
                do_parse!(
                    not!(eof!()) >>
                    dot_parser >>
                    lparen_parser >>
                    e2: exp_parser >>
                    rparen_parser >>
                    ((Syntax::Get(Box::new(e1.clone()),Box::new(e2)) ))
                 ) |
                 value!(e1)
           ) >>
        (e)
    )
);


#[test]
fn simple_exp_test() {
    let result = simple_exp_parser("((false))".as_bytes());
    assert_full_match_ok!(result,Syntax::Bool(false));

    let result = simple_exp_parser("()".as_bytes());
    assert_full_match_ok!(result,Syntax::Unit);

    let result = simple_exp_parser("true".as_bytes());
    assert_full_match_ok!(result,Syntax::Bool(true));

    let result = simple_exp_parser("false".as_bytes());
    assert_full_match_ok!(result,Syntax::Bool(false));

    let result = simple_exp_parser("448877".as_bytes());
    assert_full_match_ok!(result,Syntax::Int(448877));

    let result = simple_exp_parser("1.7e-2".as_bytes());
    assert_full_match_ok!(result,Syntax::Float(1.7e-2));

    let result = simple_exp_parser("_".as_bytes());
    assert_full_match_ok!(result,Syntax::IgnoreVar);

    let result = simple_exp_parser("5.(777)".as_bytes());
    let expect = Syntax::Get(
        Box::new(Syntax::Int(5)),
        Box::new(Syntax::Int(777)),
    );
    assert_full_match_ok!(result,expect);
}


named!(pub exp_parser<Syntax>,
        alt!(
            simple_exp_parser |
            map!(ws!(preceded!(not_parser,exp_parser)),create_not_syntax) |
            map!(ws!(preceded!(minus_parser,exp_parser)),create_minus_syntax)
        )
);

#[test]
fn exp_test() {
    let result = exp_parser("not 44".as_bytes());
    assert_full_match_ok!(result,Syntax::Not(Box::new(Syntax::Int(44))));

    let result = exp_parser("- 118".as_bytes());
    assert_full_match_ok!(result,Syntax::Neg(Box::new(Syntax::Int(118))));

    let result = exp_parser("- 118.5e4".as_bytes());
    assert_full_match_ok!(result,Syntax::Float(-118.5e4));
}
