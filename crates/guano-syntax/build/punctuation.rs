use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use strum::{AsRefStr, EnumIter, EnumVariantNames, IntoEnumIterator, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, EnumVariantNames, AsRefStr, IntoStaticStr, EnumIter)]
// #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Punctuation {
    Semicolon,
    Colon,
    Comma,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBrack,
    RightBrack,
    LeftAngle,
    RightAngle,
    At,
    Amp,
    Pipe,
    Plus,
    Star,
    Minus,
    Slash,
    Caret,
    Percent,
    Dot,
    Eq,
    Bang,
    ThinArrow,
    Amp2,
    Pipe2,
    Colon2,
    Eq2,
    LtEq,
    GtEq,
    BangEq,
    PlusEq,
    MinusEq,
    PipeEq,
    AmpEq,
    CaretEq,
    Amp2Eq,
    Pipe2Eq,
    SlashEq,
    StarEq,
    PercentEq,
    Shl,
    Shr,
    ShlEq,
    ShrEq,
    Ques,
}

impl Punctuation {
    pub fn syntax_names() -> TokenStream {
        let i = Self::iter().map(|p| {
            let kind = format_ident!("{}", p.as_ref().to_shouty_snake_case());
            let doc = format!("{:?}", p.repr());
            quote! {
                #[doc = #doc]
                #kind,
            }
        });

        quote! {
            #(#i)*
        }
    }

    pub fn type_names() -> impl Iterator<Item = &'static str> {
        Self::VARIANTS.into_iter().copied()
    }

    pub fn impl_kind() -> TokenStream {
        let from_arms = Self::iter().map(|s| {
            let string = s.repr();
            let iden = format_ident!("{}", s.as_ref().to_shouty_snake_case());

            quote! {
                #string => Self::#iden
            }
        });

        let is_arms = Self::VARIANTS.into_iter().map(|s| {
            let iden = format_ident!("{}", s.to_shouty_snake_case());
            quote! {
                Self::#iden => true
            }
        });

        quote! {
            impl crate::SyntaxKind {
                pub fn from_punctuation(input: &str) -> Option<Self> {
                    Some(match input {
                        #(#from_arms,)*
                        _ => return None
                    })
                }

                pub fn is_punctuation(&self) -> bool {
                    match self {
                        #(#is_arms,)*
                        _ => false
                    }
                }
            }
        }
    }

    pub fn reprs() -> impl Iterator<Item = (Self, &'static str)> {
        Self::iter().map(|p| (p, p.repr()))
    }

    pub fn repr(&self) -> &'static str {
        match self {
            Punctuation::Semicolon => ";",
            Punctuation::Colon => ":",
            Punctuation::Comma => ",",
            Punctuation::LeftParen => "(",
            Punctuation::RightParen => ")",
            Punctuation::LeftCurly => "{",
            Punctuation::RightCurly => "}",
            Punctuation::LeftBrack => "[",
            Punctuation::RightBrack => "]",
            Punctuation::LeftAngle => "<",
            Punctuation::RightAngle => ">",
            Punctuation::At => "@",
            Punctuation::Amp => "&",
            Punctuation::Pipe => "|",
            Punctuation::Plus => "+",
            Punctuation::Star => "*",
            Punctuation::Minus => "-",
            Punctuation::Slash => "/",
            Punctuation::Caret => "^",
            Punctuation::Percent => "%",
            Punctuation::Dot => ".",
            Punctuation::Eq => "=",
            Punctuation::Bang => "!",
            Punctuation::ThinArrow => "->",
            Punctuation::Amp2 => "&&",
            Punctuation::Pipe2 => "||",
            Punctuation::Colon2 => "::",
            Punctuation::Eq2 => "==",
            Punctuation::LtEq => "<=",
            Punctuation::GtEq => ">=",
            Punctuation::BangEq => "!=",
            Punctuation::PlusEq => "+=",
            Punctuation::MinusEq => "-=",
            Punctuation::PipeEq => "|=",
            Punctuation::AmpEq => "&=",
            Punctuation::CaretEq => "^=",
            Punctuation::Amp2Eq => "&&=",
            Punctuation::Pipe2Eq => "||=",
            Punctuation::SlashEq => "/=",
            Punctuation::StarEq => "*=",
            Punctuation::PercentEq => "%=",
            Punctuation::Shl => "<<",
            Punctuation::Shr => ">>",
            Punctuation::ShlEq => "<<=",
            Punctuation::ShrEq => ">>=",
            Punctuation::Ques => "?",
        }
    }
}
