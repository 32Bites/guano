use crate::ast::prelude::*;

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

pub(super) fn display(c: char, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
