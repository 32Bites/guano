use guano_syntax::{leaf, Node, SyntaxKind};

use crate::parsing::{
    combinators::{regex, Combinators},
    error::Res,
    ParseContext, Parser,
};

pub const REPLACEMENT_STRING: &'static str = "\u{FFFD}";

pub mod regex {
    use guano_common::konst::format::formatcp;

    pub const BYTE_ESCAPE: &'static str = r"(?:\\x[[:xdigit:]]{2})";
    pub const LITTLE_UNICODE_ESCAPE: &'static str = r"(?:\\u[[:xdigit:]]{4})";
    pub const BIG_UNICODE_ESCAPE: &'static str = r"(?:\\U[[:xdigit:]]{6})";
    pub const SINGLE_ESCAPE: &'static str = r#"(?:\\(\\|'|"|n|t|r|0))"#;
    pub const ESCAPED: &'static str = formatcp!(
        "(?:{}|{}|{}|{})",
        BYTE_ESCAPE,
        LITTLE_UNICODE_ESCAPE,
        BIG_UNICODE_ESCAPE,
        SINGLE_ESCAPE
    );
    pub const CHAR_UNESCAPED: &'static str = r"(?:[^'\n\r\\])";
    pub const STRING_UNESCAPED: &'static str = r#"(?:[^"\n\r\\])"#;

    pub const CHAR_ITEM: &'static str = formatcp!("^(?:{}|{})", CHAR_UNESCAPED, ESCAPED);
    pub const STRING_ITEM: &'static str = formatcp!("^(?:{}|{})", STRING_UNESCAPED, ESCAPED);

    pub const CHAR_LAZY: &'static str = r"^'(?s:\\.|[^'\\])*'";
    pub const STRING_LAZY: &'static str = r#"^"(?s:\\.|[^"\\])*""#;
}

pub fn char_lazy<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    regex(self::regex::CHAR_LAZY)
        .map(|text| leaf(SyntaxKind::LIT_CHAR, text))
        .parse(context)
}
