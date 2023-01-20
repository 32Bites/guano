use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use strum::{AsRefStr, EnumIter, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Clone, Copy, EnumVariantNames, AsRefStr, EnumIter)]
pub enum Literal {
    Integer,
    Float,
    String,
    Char,
}

impl Literal {
    pub fn syntax_names() -> TokenStream {
        let i = Self::iter().into_iter().map(|s| {
            let kind = format_ident!("LIT_{}", s.as_ref().to_shouty_snake_case());
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

    pub fn doc(&self) -> &'static str {
        match self {
            Literal::Integer => "Integer Literal",
            Literal::Float => "Float Literal",
            Literal::String => "String Literal",
            Literal::Char => "Character Literal",
        }
    }

    pub fn impl_kind() -> TokenStream {
        let arms = Self::VARIANTS.into_iter().map(|s| {
            let iden = format_ident!("{}", s.to_shouty_snake_case());
            quote! {
                Self::#iden => true
            }
        });

        quote! {
            impl crate::SyntaxKind {
                pub fn is_literal(&self) -> bool {
                    match self {
                        #(#arms,)*
                        _ => false
                    }
                }
            }
        }
    }

    pub fn type_names() -> impl Iterator<Item = &'static str> {
        Self::VARIANTS.into_iter().copied()
    }
}
