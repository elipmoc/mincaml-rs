use crate::token_parser::*;
use super::syntax_creator::SyntaxCreator;

use nom::IResult;
use nom::bytes::complete::take;
use nom::combinator::{map, not, value};
use nom::sequence::{tuple, preceded, pair};
use nom::branch::alt;

fn curry<T1: Copy, T2, R>(t1: T1, f: impl Fn(T1, T2) -> R) -> impl Fn(T2) -> R {
    move |t2: T2| f(t1, t2)
}

///括弧で囲まなくても関数の引数になれる式の一部分
fn simple_exp_part_parser<SC: SyntaxCreator>(syntax_creator: SC, s: &str) -> IResult<&str, SC::S> {
    alt((
        value(syntax_creator.create_unit(), preceded(lparen_parser, rparen_parser)),
        map(tuple((
            lparen_parser,
            curry(syntax_creator, exp_parser),
            rparen_parser,
        )), |(_, exp, _)| exp),
        map(bool_parser, |v| syntax_creator.create_bool(v)),
        map(int_parser, |v| syntax_creator.create_int(v)),
        map(float_parser, |v| syntax_creator.create_float(v)),
        value(syntax_creator.create_ignore_var(), ident_ignore_parser)
    ))(s)
}

///括弧で囲まなくても関数の引数になれる式
pub fn simple_exp_parser<SC: SyntaxCreator>(syntax_creator: SC, s: &str) -> IResult<&str, SC::S> {
    let (s, e1) = simple_exp_part_parser(syntax_creator, s)?;
    let e1_value = value(e1.clone(), take(0usize));
    alt((
        map(tuple((
            not(eof_parser),
            dot_parser,
            lparen_parser,
            curry(syntax_creator, exp_parser),
            rparen_parser,
        )), move |(_, _, _, e2, _)| syntax_creator.create_get(e1.clone(), e2)),
        e1_value
    ))(s)
}


#[test]
fn simple_exp_test() {
    use crate::syntax_creator_impl::SyntaxCreatorImpl;

    let syntax_creator = SyntaxCreatorImpl;

    let result = simple_exp_parser(syntax_creator, "((false))");
    let expect = syntax_creator.create_bool(false);
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "()");
    let expect = syntax_creator.create_unit();
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "true");
    let expect = syntax_creator.create_bool(true);
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "false");
    let expect = syntax_creator.create_bool(false);
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "448877");
    let expect = syntax_creator.create_int(448877);
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "1.7e-2");
    let expect = syntax_creator.create_float(1.7e-2);
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "_");
    let expect = syntax_creator.create_ignore_var();
    assert_full_match_ok!(result,expect);

    let result = simple_exp_parser(syntax_creator, "5.(777)");
    let expect = syntax_creator.create_get(
        syntax_creator.create_int(5),
        syntax_creator.create_int(777),
    );
    assert_full_match_ok!(result,expect);
}

fn unary_op_exp_parser<SC: SyntaxCreator>(syntax_creator: SC, s: &str) -> IResult<&str, SC::S> {
    alt((
        curry(syntax_creator, simple_exp_parser),
        map(preceded(not_parser, curry(syntax_creator, exp_parser)), |e| syntax_creator.create_not(e)),
        map(preceded(minus_parser, curry(syntax_creator, exp_parser)), |e| syntax_creator.create_minus(e))
    ))(s)
}

fn binary_op_exp_parser<SC: SyntaxCreator>(syntax_creator: SC, s: &str) -> IResult<&str, SC::S> {
    alt((
        map(
            pair(
                curry(syntax_creator, unary_op_exp_parser),
                preceded(plus_parser, curry(syntax_creator, exp_parser)),
            ),
            |(e1, e2)| syntax_creator.create_add(e1, e2),
        ),
        curry(syntax_creator, unary_op_exp_parser)
    ))(s)
}


pub fn exp_parser<SC: SyntaxCreator>(syntax_creator: SC, s: &str) -> IResult<&str, SC::S> {
    ws!(s,curry(syntax_creator,binary_op_exp_parser))
}


#[test]
fn exp_test() {
    use crate::syntax_creator_impl::SyntaxCreatorImpl;

    let syntax_creator = SyntaxCreatorImpl;

    let result = exp_parser(syntax_creator, "not 44");
    let expect = syntax_creator.create_not(syntax_creator.create_int(44));
    assert_full_match_ok!(result, expect);

    let result = exp_parser(syntax_creator, "- 118");
    let expect = syntax_creator.create_minus(syntax_creator.create_int(118));
    assert_full_match_ok!(result,expect);

    let result = exp_parser(syntax_creator, "- 118.5e4");
    let expect = syntax_creator.create_float(-118.5e4);
    assert_full_match_ok!(result,expect);

    let result = exp_parser(syntax_creator, "4+7788");
    let expect = syntax_creator.create_add(
        syntax_creator.create_int(4),
        syntax_creator.create_int(7788),
    );
    assert_full_match_ok!(result,expect);
}
