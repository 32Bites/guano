use crate::ast::prelude::*;

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

pub fn parse_hexidecimal_integer(input: Span) -> Res<Lit> {
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
        preceded(peek(tag_no_case("0x")), parse_hexidecimal_integer),
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
