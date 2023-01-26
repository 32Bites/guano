use crate::ast::prelude::*;
use guano_interner::InternedInteger;
use guano_syntax::{
    leaf,
    parser::{Input, Res as Result},
    tokens::Integer,
    AstToken, SyntaxKind,
};

pub trait IntegerExt {
    fn value(&self) -> Option<InternedInteger>;
    fn radix(&self) -> Radix;
}

impl IntegerExt for Integer {
    #[inline]
    fn value(&self) -> Option<InternedInteger> {
        if let Some(integer) = guano_interner::integer(self.text()) {
            return Some(integer);
        }

        let radix = self.radix();
        let text = &self.text()[radix.prefix_len()..];

        rug::Integer::from_str_radix(text, radix as i32)
            .ok()
            .map(|i| guano_interner::set_integer(self.text(), i))
    }

    #[inline]
    fn radix(&self) -> Radix {
        let is_hexadecimal = self.text().starts_with("0x") || self.text().starts_with("0X");
        let is_binary = self.text().starts_with("0b") || self.text().starts_with("0B");

        if is_hexadecimal {
            Radix::Hexadecimal
        } else if is_binary {
            Radix::Binary
        } else {
            Radix::Decimal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Radix {
    Binary = 2,
    Decimal = 10,
    Hexadecimal = 16,
}

impl Radix {
    pub const fn prefix_len(&self) -> usize {
        match self {
            Radix::Binary | Radix::Hexadecimal => 2,
            Radix::Decimal => 0,
        }
    }
}

pub fn decimal<'a>(input: Input<'a>) -> Result<'a, Input<'a>> {
    recognize(many1_count(terminated(
        one_of("1234567890"),
        many0_count(char('_')),
    )))(input)
}

pub fn decimal_integer_literal<'a>(input: Input<'a>) -> Result<'a> {
    let (input, text) = decimal(input)?;

    let literal = leaf(SyntaxKind::LIT_INTEGER, text.fragment());

    Ok((input, literal))
}

pub fn binary_integer_literal<'a>(input: Input<'a>) -> Result<'a> {
    let binary_digit = map(new_expect(one_of("10"), "Expected a binary digit"), |o| {
        o.unwrap_or('0')
    });
    let (input, text) = recognize(preceded(
        tag_no_case("0b"),
        many1_count(terminated(binary_digit, many0_count(char('_')))),
    ))(input)?;

    let literal = leaf(SyntaxKind::LIT_INTEGER, text.fragment());

    Ok((input, literal))
}

pub fn hexadecimal_integer_literal<'a>(input: Input<'a>) -> Result<'a> {
    let hex_digit = map(
        new_expect(
            one_of("0123456789abcdefABCDEF"),
            "Expected a hexadecimal digit",
        ),
        |o| o.unwrap_or('0'),
    );
    let (input, text) = recognize(preceded(
        tag_no_case("0x"),
        many1_count(terminated(hex_digit, many0_count(char('_')))),
    ))(input)?;

    let literal = leaf(SyntaxKind::LIT_INTEGER, text.fragment());

    Ok((input, literal))
}

pub fn integer_literal<'a>(input: Input<'a>) -> Result<'a> {
    alt((
        decimal_integer_literal,
        binary_integer_literal,
        hexadecimal_integer_literal,
    ))(input)
}

pub fn parse_binary_integer(input: Span) -> Res<Lit> {
    let prefix = tag_no_case("0b");
    let binary_digit = map(expect(one_of("10"), "Expected a '1' or '0'"), |o| {
        o.unwrap_or('0')
    });
    let binary_string = preceded(prefix, recognize(many1(binary_digit)));
    let binary_int = map_res(binary_string, |s: Span| u128::from_str_radix(s.as_ref(), 2));
    let int = map(expect(binary_int, "Invalid binary integer"), |o| {
        o.unwrap_or_default()
    });
    let mut spanned_int = map(consumed(int), |(span, i)| (span.to_node(), i));

    let (input, (span, integer)) = spanned_int(input)?;

    Ok((input, Lit::new_integer(integer, &span)))
}

pub fn parse_hexadecimal_integer(input: Span) -> Res<Lit> {
    let prefix = tag_no_case("0x");
    let hex_digit = map(
        expect(
            one_of("0123456789abcdefABCDEF"),
            "Expected a hexadecimal digit",
        ),
        |o| o.unwrap_or('0'),
    );
    let hex_string = preceded(prefix, recognize(many1(hex_digit)));
    let hex_int = map_res(hex_string, |s: Span| u128::from_str_radix(s.as_ref(), 16));

    let int = map(expect(hex_int, "Invalid hexadecimal integer"), |o| {
        o.unwrap_or_default()
    });

    let mut spanned_int = map(consumed(int), |(span, i)| (span.to_node(), i));

    let (input, (span, integer)) = spanned_int(input)?;

    Ok((input, Lit::new_integer(integer, &span)))
}

pub fn parse_decimal_integer(input: Span) -> Res<Lit> {
    let decimal_string = recognize(many1_count(digit1));
    let decimal_int = map_res(decimal_string, |s: Span| {
        u128::from_str_radix(s.as_ref(), 10)
    });

    let int = map(expect(decimal_int, "Invalid integer"), |o| {
        o.unwrap_or_default()
    });

    let mut spanned_int = map(consumed(int), |(span, i)| (span.to_node(), i));

    let (input, (span, integer)) = spanned_int(input)?;

    Ok((input, Lit::new_integer(integer, &span)))
}

pub fn parse(input: Span) -> Res<Lit> {
    let mut int = alt((
        preceded(peek(tag_no_case("0b")), parse_binary_integer),
        preceded(peek(tag_no_case("0x")), parse_hexadecimal_integer),
        preceded(
            peek(alt((
                value((), preceded(tag("0"), not(digit1))),
                value((), one_of("123456789")),
            ))),
            parse_decimal_integer,
        ),
    ));

    int(input)
}
