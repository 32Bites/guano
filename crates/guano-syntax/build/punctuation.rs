use heck::ToShoutySnakeCase;
use itertools::Itertools;
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
    Lt,
    Gt,
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
    Lt2,
    LtEq,
    GtEq,
    Gt2,
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
    Lt2Eq,
    Gt2Eq,
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

    pub fn consts() -> TokenStream {
        let mut names = vec![];
        let mut arms = vec![];
        let mut repr_match_arms = vec![];

        let reprs = Punctuation::reprs()
            .into_group_map_by(|v| v.1.chars().next().unwrap())
            .into_iter()
            .sorted_by_key(|k| k.0)
            .map(|(_, v)| v)
            .into_group_map_by(|v| v.len() == 1)
            .into_iter()
            .sorted_by_key(|k| k.0)
            .rev()
            .map(|(_, v)| {
                v.into_iter()
                    .map(|v| v.into_iter().sorted_by_key(|p| p.1).rev())
                    .flatten()
            })
            .flatten()
            .collect_vec();

        // let groups = format!("{:#?}", reprs);

        for (name, repr) in reprs {
            let name = format_ident!("{}", name.as_ref().to_shouty_snake_case());
            let doc = format!("{repr:?}");

            names.push(name.clone());

            let arm = quote! {
                #[doc = #doc]
                #name,
            };

            arms.push(arm);

            let repr_match_arm = quote! {
                Punctuation::#name => #repr,
            };

            repr_match_arms.push(repr_match_arm);
        }

        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            #[doc = "Punctuation marks"]
            pub enum Punctuation {
                #(#arms)*
            }

            impl Punctuation {
                pub const ALL: &'static [Punctuation] = &[#(Punctuation::#names,)*];
                // pub const STRINGS: &'static [&'static str] = &[#(Punctuation::#names.as_str(),)*];

                pub const fn as_str(&self) -> &'static str {
                    match self {
                        #(#repr_match_arms)*
                    }
                }

                pub const fn syntax_kind(&self) -> crate::SyntaxKind {
                    match self {
                        #(Punctuation::#names => crate::SyntaxKind::#names,)*
                    }
                }
            }

            impl From<Punctuation> for crate::SyntaxKind {
                #[inline]
                fn from(p: Punctuation) -> Self {
                    p.syntax_kind()
                }
            }
        }
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
            Punctuation::Lt => "<",
            Punctuation::Gt => ">",
            Punctuation::Lt2 => "<<",
            Punctuation::Gt2 => ">>",
            Punctuation::Lt2Eq => "<<=",
            Punctuation::Gt2Eq => ">>=",
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
            Punctuation::Ques => "?",
        }
    }
}
