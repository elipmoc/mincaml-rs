use crate::token_parser::*;
use crate::syntax::Syntax;
use super::helper::*;

use nom::IResult;
use nom::bytes::complete::take;
use nom::combinator::{map, not, value};
use nom::sequence::{tuple, preceded, pair};
use nom::branch::alt;

///括弧で囲まなくても関数の引数になれる式の一部分
fn simple_exp_part_parser(s: &str) -> IResult<&str, Syntax> {
    alt((
        value(Syntax::Unit, preceded(lparen_parser, rparen_parser)),
        map(tuple((
            lparen_parser,
            exp_parser,
            rparen_parser,
        )), |(_, exp, _)| exp),
        map(bool_parser, Syntax::Bool),
        map(int_parser, Syntax::Int),
        map(float_parser, Syntax::Float),
        value(Syntax::IgnoreVar, ident_ignore_parser)
    ))(s)
}

///括弧で囲まなくても関数の引数になれる式
pub fn simple_exp_parser(s: &str) -> IResult<&str, Syntax> {
    let (s, e1) = simple_exp_part_parser(s)?;
    let e1_value = value(e1.clone(), take(0usize));
    alt((
        map(tuple((
            not(eof_parser),
            dot_parser,
            lparen_parser,
            exp_parser,
            rparen_parser,
        )), move |(_, _, _, e2, _)| Syntax::Get(Box::new(e1.clone()), Box::new(e2))),
        e1_value
    ))(s)
}


#[test]
fn simple_exp_test() {
    let result = simple_exp_parser("((false))");
    assert_full_match_ok!(result,Syntax::Bool(false));

    let result = simple_exp_parser("()");
    assert_full_match_ok!(result,Syntax::Unit);

    let result = simple_exp_parser("true");
    assert_full_match_ok!(result,Syntax::Bool(true));

    let result = simple_exp_parser("false");
    assert_full_match_ok!(result,Syntax::Bool(false));

    let result = simple_exp_parser("448877");
    assert_full_match_ok!(result,Syntax::Int(448877));

    let result = simple_exp_parser("1.7e-2");
    assert_full_match_ok!(result,Syntax::Float(1.7e-2));

    let result = simple_exp_parser("_");
    assert_full_match_ok!(result,Syntax::IgnoreVar);

    let result = simple_exp_parser("5.(777)");
    let expect = Syntax::Get(
        Box::new(Syntax::Int(5)),
        Box::new(Syntax::Int(777)),
    );
    assert_full_match_ok!(result,expect);
}

fn unary_op_exp_parser(s: &str) -> IResult<&str, Syntax> {
    alt((
        simple_exp_parser,
        map(preceded(not_parser, exp_parser), create_not_syntax),
        map(preceded(minus_parser, exp_parser), create_minus_syntax)
    ))(s)
}

fn binary_op_exp_parser(s: &str) -> IResult<&str, Syntax> {
    alt((
        map(
            pair(
                unary_op_exp_parser,
                preceded(plus_parser, exp_parser),
            ),
            create_plus_syntax,
        ),
        unary_op_exp_parser
    ))(s)
}


named!(pub exp_parser<&str,Syntax>,
        ws!(binary_op_exp_parser)
);

#[test]
fn exp_test() {
    let result = exp_parser("not 44");
    assert_full_match_ok!(result,Syntax::Not(Box::new(Syntax::Int(44))));

    let result = exp_parser("- 118");
    assert_full_match_ok!(result,Syntax::Neg(Box::new(Syntax::Int(118))));

    let result = exp_parser("- 118.5e4");
    assert_full_match_ok!(result,Syntax::Float(-118.5e4));

    let result = exp_parser("4+7788");
    let expect = Syntax::Add(
        Box::new(Syntax::Int(4)),
        Box::new(Syntax::Int(7788)),
    );
    assert_full_match_ok!(result,expect);
}
