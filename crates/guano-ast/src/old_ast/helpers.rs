use guano_common::rowan::TextRange;
use guano_span::input::Input;
use guano_syntax::{
    error, leaf,
    parser::error::{AccumulateErrors, ErrorAccumulator},
    SyntaxKind,
};
use nom::IResult;

use super::{comment, prelude::*};

/// Evaluate `parser` and wrap the result in a `Some(_)`. Otherwise,
/// emit the  provided `error` and return a `None` while allowing
/// parsing to continue.
/// Stolen (mostly) from https://eyalkalderon.com/blog/nom-error-recovery/
pub fn expect<'a, F, E, T>(mut parser: F, error: E) -> impl FnMut(Span) -> Res<Option<T>>
where
    F: Parser<Span, T, NomError<Span>>,
    E: Into<ErrorKind> + Clone,
{
    move |input| {
        match parser.parse(input) {
            Ok((remaining, out)) => Ok((remaining, Some(out))),
            Err(nom::Err::Error(i)) | Err(nom::Err::Failure(i)) => {
                let err = Error(i.input.to_node(), error.clone().into());
                i.input.extra.report_error(err); // Push error onto stack.
                Ok((i.input.clone(), None)) // Parsing failed, but keep going.
            }
            Err(err) => Err(err),
        }
    }
}

/// Evaluate `parser` and wrap the result in a `Some(_)`. Otherwise,
/// emit the  provided `error` and return a `None` while allowing
/// parsing to continue.
/// Stolen (mostly) from https://eyalkalderon.com/blog/nom-error-recovery/
pub fn new_expect<'a, P, E, T>(
    mut parser: P,
    error: E,
) -> impl FnMut(guano_syntax::parser::Input<'a>) -> guano_syntax::parser::Res<'a, Option<T>>
where
    P: Parser<guano_syntax::parser::Input<'a>, T, guano_syntax::parser::NomError<'a>>,
    E: Into<guano_syntax::parser::error::ErrorKind> + Clone,
{
    move |input| match parser.parse(input) {
        Ok((remaining, out)) => Ok((remaining, Some(out))),
        Err(nom::Err::Error(i)) | Err(nom::Err::Failure(i)) => {
            let start = i.input.location_offset() as u32;
            let end = start + i.input.len() as u32;
            let error = (TextRange::new(start.into(), end.into()), error.clone());
            i.input.error_accumulator().report_error(error.into());

            Ok((i.input.clone(), None))
        }
        Err(err) => Err(err),
    }
}

pub fn expect_node<'a, P, E>(
    mut parser: P,
    err: E,
) -> impl FnMut(guano_syntax::parser::Input<'a>) -> guano_syntax::parser::Res<'a>
where
    P: Parser<
        guano_syntax::parser::Input<'a>,
        guano_syntax::Node,
        guano_syntax::parser::NomError<'a>,
    >,
    E: Into<guano_syntax::parser::error::ErrorKind> + Clone,
{
    move |input| {
        map(
            consumed(new_expect(|input| parser.parse(input), err.clone())),
            |(text, node)| node.unwrap_or(error(text.fragment())),
        )(input)
    }
}

/// Same as above but the error is created at through a function call.
#[allow(dead_code)]
pub fn expect_fn<'a, F, E, I, T>(
    mut parser: F,
    mut error_func: E,
) -> impl FnMut(Span) -> Res<Option<T>>
where
    F: FnMut(Span) -> Res<T>,
    E: FnMut(&NomError<Span>) -> Option<I>,
    I: Into<ErrorKind>,
{
    move |input| {
        match parser(input) {
            Ok((remaining, out)) => Ok((remaining, Some(out))),
            e @ (Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_))) => {
                let is_failure = match &e {
                    Ok(_) => unreachable!(),
                    Err(e) => match e {
                        nom::Err::Failure(_) => true,
                        _ => false,
                    },
                };

                match e {
                    Err(nom::Err::Error(i)) | Err(nom::Err::Failure(i)) => {
                        match error_func(&i) {
                            Some(err) => {
                                let err = Error(i.input.to_node(), err.into());
                                i.input.extra.report_error(err); // Push error onto stack.
                                Ok((i.input.clone(), None))
                            }
                            None => Err(if is_failure {
                                nom::Err::Failure(i)
                            } else {
                                nom::Err::Error(i)
                            }),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Err(err) => Err(err),
        }
    }
}

pub fn padded<'a, T, P>(parser: P) -> impl FnMut(Span) -> Res<T>
where
    P: Parser<Span, T, NomError<Span>>,
{
    delimited(ignorable, parser, ignorable)
}

/// returns the output of parser and any comments or whitespace found around the parser
pub fn pad<'a, T, P>(
    mut parser: P,
) -> impl FnMut(
    guano_syntax::parser::Input<'a>,
) -> guano_syntax::parser::Res<'a, (Vec<guano_syntax::Node>, T, Vec<guano_syntax::Node>)>
where
    P: Parser<guano_syntax::parser::Input<'a>, T, guano_syntax::parser::NomError<'a>>,
{
    move |input| {
        tuple((
            comments_whitespace,
            |input| parser.parse(input),
            comments_whitespace,
        ))(input)
    }
}

pub fn pad_l<'a, T, P>(
    mut parser: P,
) -> impl FnMut(
    guano_syntax::parser::Input<'a>,
) -> guano_syntax::parser::Res<'a, (Vec<guano_syntax::Node>, T)>
where
    P: Parser<guano_syntax::parser::Input<'a>, T, guano_syntax::parser::NomError<'a>>,
{
    move |input| pair(comments_whitespace, |input| parser.parse(input))(input)
}

pub fn pad_r<'a, T, P>(
    mut parser: P,
) -> impl FnMut(
    guano_syntax::parser::Input<'a>,
) -> guano_syntax::parser::Res<'a, (T, Vec<guano_syntax::Node>)>
where
    P: Parser<guano_syntax::parser::Input<'a>, T, guano_syntax::parser::NomError<'a>>,
{
    move |input| pair(|input| parser.parse(input), comments_whitespace)(input)
}

pub fn ignorable(input: Span) -> Res<()> {
    value(
        (),
        many0_count(alt((comment::parse, value((), multispace1)))),
    )(input)
}

pub fn comments_whitespace<'a>(
    input: guano_syntax::parser::Input<'a>,
) -> guano_syntax::parser::Res<'a, Vec<guano_syntax::Node>> {
    /*     value(
        (),
        many0_count(alt((comment::parse, value((), multispace1)))),
    )(input) */

    let either = alt((whitespace, comment::comment));

    fold_many0(either, Vec::new, |mut nodes, ignored| {
        nodes.push(ignored);

        nodes
    })(input)
}

pub fn whitespace<'a>(input: guano_syntax::parser::Input<'a>) -> guano_syntax::parser::Res<'a> {
    map(multispace1, |s: guano_syntax::parser::Input<'a>| {
        leaf(SyntaxKind::WHITESPACE, s.fragment())
    })(input)
}

pub fn spanned<'a, T: 'a, P>(parser: P) -> impl FnMut(Span) -> Res<Spanned<T>>
where
    P: Parser<Span, T, NomError<Span>>,
{
    map(consumed(parser), |(span, value)| {
        Spanned::new(value, span.into_node())
    })
}
