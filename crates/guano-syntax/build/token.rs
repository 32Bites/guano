use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use strum::{AsRefStr, EnumIter, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Clone, Copy, EnumVariantNames, AsRefStr, EnumIter)]
pub enum Token {
    Comment,
    Iden,
    Whitespace,
    Error,
}

impl Token {
    pub fn syntax_names() -> TokenStream {
        let i = Self::iter().into_iter().map(|s| {
            let kind = format_ident!("{}", s.as_ref().to_shouty_snake_case());
            let doc = s.doc();

            quote! {
                #[doc = #doc]
                #kind,
            }
        });

        quote! {
            #(#i)*
        }
    }

    pub fn impls() -> impl Iterator<Item = &'static str> {
        Self::VARIANTS.into_iter().map(|s| *s)
    }

    pub fn doc(&self) -> &'static str {
        match self {
            Token::Comment => "Comment Token",
            Token::Iden => "Identifier Token",
            Token::Whitespace => "Whitespace Token",
            Token::Error => "Error Token",
        }
    }

    pub fn type_names() -> impl Iterator<Item = &'static str> {
        Self::VARIANTS.into_iter().copied()
    }
}
