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

pub fn ignorable(input: Span) -> Res<()> {
    value(
        (),
        many0_count(alt((comment::parse, value((), multispace1)))),
    )(input)
}

pub fn spanned<'a, T: 'a, P>(parser: P) -> impl FnMut(Span) -> Res<Spanned<T>>
where
    P: Parser<Span, T, NomError<Span>>,
{
    map(consumed(parser), |(span, value)| {
        Spanned::new(value, span.into_node())
    })
}
