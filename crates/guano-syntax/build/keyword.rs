use heck::ToShoutySnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use strum::{AsRefStr, EnumIter, EnumVariantNames, IntoEnumIterator, VariantNames};

#[derive(Debug, Clone, Copy, EnumVariantNames, AsRefStr, EnumIter, PartialEq, Eq)]
pub enum Keyword {
    Return,
    While,
    For,
    In,
    Loop,
    Break,
    Continue,
    Impl,
    Fun,
    Let,
    Var,
    Veto,
    Pub,
    Proto,
    Class,
    On,
    Module,
    Try,
    Catch,
    Throw,
    As,
    Import,
    Is,
    From,
    If,
    Else,
    Static,
    This,
    Nil,
    True,
    False,
    Nan,
    Inf,
    // Float,
    // Uint,
    // Int,
    // Char,
    // String,
    // Boolean,
}

impl Keyword {
    pub fn syntax_names() -> TokenStream {
        let i = Self::iter().map(|s| {
            let kind = format_ident!("KW_{}", s.as_ref().to_shouty_snake_case());
            let doc = s.as_ref().to_lowercase();
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

    /*     pub const fn primitives() -> &'static [Self] {
        &[Self::Boolean,  Self::Int, Self::Uint,  Self::Float,  Self::Char,  Self::String]
    } */

    pub fn impl_kind() -> TokenStream {
        let from_arms = Self::VARIANTS.into_iter().map(|s| {
            let string = s.to_lowercase();
            let iden = format_ident!("KW_{}", s.to_shouty_snake_case());

            quote! {
                #string => Self::#iden
            }
        });

        let is_key_arms = Self::VARIANTS.into_iter().map(|s| {
            let iden = format_ident!("KW_{}", s.to_shouty_snake_case());
            quote! {
                Self::#iden => true
            }
        });

        /*         let is_prim_arms = Self::primitives().into_iter().map(|s| {
            let iden = format_ident!("KW_{}", s.as_ref().to_shouty_snake_case());

            quote! {
                Self::#iden => true
            }
        }); */

        quote! {
                    impl crate::SyntaxKind {
                        pub fn from_keyword(input: &str) -> Option<Self> {
                            Some(match input {
                                #(#from_arms,)*
                                _ => return None
                            })
                        }

                        pub fn is_keyword(&self) -> bool {
                            match self {
                                #(#is_key_arms,)*
                                _ => false
                            }
                        }

        /*                 #[doc = "Check if this syntax type is a keyword that corresponds to a primitive"]
                        pub fn is_primitive(&self) -> bool {
                            match self {
                                #(#is_prim_arms,)*
                                _ => false
                            }
                        } */
                    }
                }
    }
}
