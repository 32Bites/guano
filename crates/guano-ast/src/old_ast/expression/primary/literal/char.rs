use guano_syntax::{
    leaf,
    parser::{error::Errors, Input, Res as Result},
    tokens::Char,
    AstToken, SyntaxKind,
};
use nom::{Compare, InputLength};

use crate::ast::prelude::*;

pub trait CharExt {
    fn value(&self) -> Option<char>;
}

impl CharExt for Char {
    fn value(&self) -> Option<char> {
        if let Some(c) = guano_interner::char(self.text()) {
            return Some(c);
        }

        let input = LocatedSpan::new_extra(self.text(), Errors::default());

        let (_, c) = delimited(tag("'"), text_item("'"), tag("'"))(input).ok()?;
        guano_interner::set_char(self.text(), c);

        Some(c)
    }
}

pub fn parse(input: Span) -> Res<Lit> {
    map(
        consumed(delimited(
            tag("'"),
            map(
                expect(parse_text_item('\''), "Empty character literal"),
                |c| c.unwrap_or('\u{FFFD}'),
            ),
            expect(tag("'"), "Expected a '"),
        )),
        |(span, character)| Lit::new_char(character, &span.into_node()),
    )(input)
}

pub fn character_literal<'a>(input: Input<'a>) -> Result<'a> {
    let (input, text) = recognize(delimited(
        tag("'"),
        new_expect(text_item_lazy("'"), "Empty character literal"),
        new_expect(tag("'"), "Expected a '"),
    ))(input)?;

    let literal = leaf(SyntaxKind::LIT_CHAR, text.fragment());

    Ok((input, literal))
}

/// Parses an ascii escape string omitting the preceding \x.
/// Does not determine the unescaped value
pub fn byte_escape_lazy<'a>(input: Input<'a>) -> Result<'a, Option<Input<'a>>> {
    preceded(
        tag("\\x"),
        new_expect(
            recognize(count(one_of("0123456789abcdefABCDEF"), 2)),
            "Expected hexadecimal digit",
        ),
    )(input)
}

pub fn byte_escape<'a>(input: Input<'a>) -> Result<'a, Option<char>> {
    preceded(
        peek(tag("\\x")),
        new_expect(
            map_opt(byte_escape_lazy, |o| {
                o.and_then(|i| {
                    u8::from_str_radix(&i, 16).ok().and_then(|b| {
                        if b <= 0x7F {
                            Some(b as char)
                        } else {
                            None
                        }
                    })
                })
            }),
            "Invalid ascii escape",
        ),
    )(input)
}

/// Parses a unicode escape string omitting the preceding \u or \U
/// Does not determine the unescaped value
pub fn unicode_escape_lazy<'a>(input: Input<'a>) -> Result<'a, Option<Input<'a>>> {
    let (input, amount) = alt((value(4usize, tag("\\u")), value(6usize, tag("\\U"))))(input)?;

    new_expect(
        recognize(count(one_of("0123456789abcdefABCDEF"), amount)),
        "Expected hexadecimal digit",
    )(input)
}

pub fn unicode_escape<'a>(input: Input<'a>) -> Result<'a, Option<char>> {
    preceded(
        peek(tag_no_case("\\u")),
        new_expect(
            map_opt(unicode_escape_lazy, |o| {
                o.and_then(|i| u32::from_str_radix(&i, 16).ok().and_then(char::from_u32))
            }),
            "Invalid unicode escape",
        ),
    )(input)
}

pub fn single_escape_lazy<'a>(input: Input<'a>) -> Result<'a, Option<Input<'a>>> {
    preceded(
        tag("\\"),
        new_expect(
            alt((
                tag("\\"),
                tag("\'"),
                tag("\""),
                tag("n"),
                tag("t"),
                tag("r"),
                tag("0"),
            )),
            "Invalid single escape",
        ),
    )(input)
}

pub fn single_escape<'a>(input: Input<'a>) -> Result<'a, Option<char>> {
    map(single_escape_lazy, |i| {
        i.and_then(|i| {
            Some(match *i.fragment() {
                "\\" => '\\',
                "\'" => '\'',
                "\"" => '\"',
                "n" => '\n',
                "t" => '\t',
                "r" => '\r',
                "0" => '\0',
                _ => return None,
            })
        })
    })(input)
}

pub fn unescaped<'a, D>(delim: D) -> impl FnMut(Input<'a>) -> Result<'a, Option<char>>
where
    D: InputTake + InputLength + Clone,
    Input<'a>: Compare<D>,
{
    move |input| {
        new_expect(
            preceded(
                not(alt((
                    value((), one_of("\r\n\\")),
                    value((), tag(delim.clone())),
                ))),
                anychar,
            ),
            "Unexpected character",
        )(input)
    }
}

pub fn text_item_lazy<'a, D>(delim: D) -> impl FnMut(Input<'a>) -> Result<'a, ()>
where
    D: InputTake + InputLength + Clone,
    Input<'a>: Compare<D>,
{
    move |input| {
        let escape = alt((
            preceded(peek(tag("\\x")), value((), byte_escape_lazy)),
            preceded(peek(tag_no_case("\\u")), value((), unicode_escape_lazy)),
            preceded(peek(tag("\\")), value((), single_escape_lazy)),
        ));

        let unescaped = value((), unescaped(delim.clone()));

        alt((escape, unescaped))(input)
    }
}

pub fn text_item<'a, D>(delim: D) -> impl FnMut(Input<'a>) -> Result<'a, char>
where
    D: InputTake + InputLength + Clone,
    Input<'a>: Compare<D>,
{
    move |input| {
        let escape = alt((
            preceded(peek(tag("\\x")), byte_escape),
            preceded(peek(tag_no_case("\\u")), unicode_escape),
            preceded(peek(tag("\\")), single_escape),
        ));

        map(
            alt((escape, unescaped(delim.clone()))),
            |o: Option<char>| o.unwrap_or(char::REPLACEMENT_CHARACTER),
        )(input)
    }
}

pub fn parse_text_item(delim: char) -> impl FnMut(Span) -> Res<char> {
    let byte_escape = preceded(
        tag("x"),
        expect(
            map_opt(
                recognize(count(one_of("0123456789abcdefABCDEF"), 2)),
                |s: Span| {
                    u8::from_str_radix(&s, 16).ok().and_then(|b| {
                        if b <= 0x7F {
                            Some(b as char)
                        } else {
                            None
                        }
                    })
                },
            ),
            "Invalid ascii escape",
        ),
    );
    let little_unicode_escape = preceded(
        tag("u"),
        expect(
            map_opt(
                recognize(count(one_of("0123456789abcdefABCDEF"), 4)),
                |s: Span| {
                    u32::from_str_radix(&s, 16)
                        .ok()
                        .and_then(|u| char::from_u32(u))
                },
            ),
            "Invalid little unicode escape",
        ),
    );
    let big_unicode_escape = preceded(
        tag("U"),
        expect(
            map_opt(
                recognize(count(one_of("0123456789abcdefABCDEF"), 8)),
                |s: Span| {
                    u32::from_str_radix(&s, 16)
                        .ok()
                        .and_then(|u| char::from_u32(u))
                },
            ),
            "Invalid big unicode escape",
        ),
    );

    let escape = preceded(
        tag("\\"),
        alt((
            byte_escape,
            little_unicode_escape,
            big_unicode_escape,
            expect(
                alt((
                    value('\\', tag("\\")),
                    value('\'', tag("\'")),
                    value('\"', tag("\"")),
                    value('\n', tag("n")),
                    value('\t', tag("t")),
                    value('\r', tag("r")),
                    value('\0', tag("0")),
                )),
                "Invalid escape code",
            ),
        )),
    );
    let unescaped = map(
        preceded(not(alt((one_of("\r\n\\"), char(delim)))), anychar),
        |c: char| Some(c),
    );
    let mut text_char = map(alt((escape, unescaped)), |c: Option<char>| {
        c.unwrap_or('\u{FFFD}')
    });

    move |input| text_char(input)
}

pub(super) fn display(c: char, f: &mut impl std::fmt::Write) -> std::fmt::Result {
    match c {
        '\n' => f.write_str("\\n"),
        '\t' => f.write_str("\\t"),
        '\r' => f.write_str("\\r"),
        '\0' => f.write_str("\\0"),
        '\"' => f.write_str("\\\""),
        '\'' => f.write_str("\\\'"),
        '\\' => f.write_str("\\\\"),
        '\x20'..='\x7e' => write!(f, "{c}"), // Printable ascii
        c if c.is_ascii() => {
            write!(f, "\\x{:X}", c as u32)
        } // Escaped byte
        c => {
            if u16::try_from(c as u32).is_ok() {
                write!(f, "\\u{:X}", c as u32)
            } else {
                write!(f, "\\U{:X}", c as u32)
            }
        }
    }
}
