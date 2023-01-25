#![allow(dead_code)]

use ast::AstNode;
use build_helper::out_dir;
use heck::{ToShoutySnakeCase, ToTitleCase};
use keyword::Keyword;
use literal::Literal;
use proc_macro2::TokenStream;
use punctuation::Punctuation;
use quote::{format_ident, quote};
use rust_format::Formatter;
use std::{borrow::Cow, fs::File, io::Write};
use strum::VariantNames;
use token::Token;
use ungrammar::Grammar;

mod ast;
mod keyword;
mod literal;
mod punctuation;
mod token;

fn main() {
    // rerun_if_changed(concat!(env!("CARGO_MANIFEST_DIR"), "/guano.ungram"));
    let grammar: Grammar = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/guano.ungram"))
        .parse()
        .unwrap();
    let mut lib_file = open_file("lib");

    write_syntax(&mut lib_file, &grammar);
    let mut tokens_file = open_file("tokens");
    let tokens = Token::impls()
        .into_iter()
        .map(|s| (s.into(), s.into()))
        .chain(
            Literal::VARIANTS
                .into_iter()
                .map(|s| (format!("Lit{s}").into(), (*s).into())),
        );
    write_tokens(&mut tokens_file, tokens);

    let mut nodes_file = open_file("nodes");
    write_nodes(&mut nodes_file, &grammar);

    let mut parser_file = open_file("consts");
    write_consts(&mut parser_file);
}

fn open_file(name: &str) -> File {
    let output_path = out_dir().join(format!("generated_{name}.rs"));

    File::create(&output_path).expect(&format!("Failed to open {}", output_path.display()))
}

fn write_source(file: &mut File, source: TokenStream) {
    let source = rust_format::RustFmt::new()
        .format_tokens(source.clone())
        .unwrap_or_else(|_| source.to_string());

    write!(file, "{}", source).unwrap();
}

fn write_syntax(file: &mut File, grammar: &Grammar) {
    let token_variants = Token::syntax_names();
    let punctuation_variants = Punctuation::syntax_names();
    let punctuation_impl = Punctuation::impl_kind();
    let keyword_variants = Keyword::syntax_names();
    let keyword_impl = Keyword::impl_kind();
    let literal_variants = Literal::syntax_names();

    let node_variants = grammar.iter().map(|node| {
        let node_name = &grammar[node].name;
        let kind = format_ident!("{}", node_name.to_shouty_snake_case());

        quote! {
            #kind,
        }
    });

    let source = quote! {
        use ::guano_common::num::traits;
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ::guano_common::num::derive::FromPrimitive, ::guano_common::num::derive::ToPrimitive)]
        #[num_traits = "traits"]
        #[allow(non_camel_case_types)]
        #[repr(u16)]
        #[doc = "Syntax kind type for use with [rowan]"]
        pub enum SyntaxKind {
            #token_variants
            #punctuation_variants
            #keyword_variants
            #literal_variants
            #(#node_variants)*
        }

        impl From<SyntaxKind> for ::guano_common::rowan::SyntaxKind {
            fn from(kind: SyntaxKind) -> Self {
                use ::guano_common::num::traits::ToPrimitive;
                ::guano_common::rowan::SyntaxKind(kind.to_u16().unwrap())
            }
        }
        #punctuation_impl
        #keyword_impl
    };

    write_source(file, source);
}

fn write_tokens<'a>(
    file: &mut File,
    names: impl IntoIterator<Item = (Cow<'a, str>, Cow<'a, str>)>,
) {
    let types = names.into_iter().map(|(kind, ty)| {
        let doc = format!("{}", ty.to_title_case());
        let ty = format_ident!("{}", ty);
        let kind = format_ident!("{}", kind.to_shouty_snake_case());

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[doc = #doc]
            pub struct #ty(crate::SyntaxToken);

            impl crate::AstToken for #ty {
                #[inline]
                fn can_cast(kind: crate::SyntaxKind) -> bool { kind == crate::SyntaxKind::#kind }
                #[inline]
                fn cast(syntax: crate::SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self(syntax))
                    } else {
                        None
                    }
                }
                #[inline]
                fn syntax(&self) -> &crate::SyntaxToken { &self.0 }
            }
        }
    });

    let source = quote! {
        #(#types)*
    };

    write_source(file, source);
}

fn write_nodes(file: &mut File, grammar: &Grammar) {
    let nodes = AstNode::handle(grammar)
        .into_iter()
        .map(|n| n.token_stream());

    let source = quote! {
        #(#nodes)*
    };

    write_source(file, source);
}

fn write_consts(file: &mut File) {
    let kw_consts = Keyword::consts();
    let punct_consts = Punctuation::consts();

    let source = quote! {
        #kw_consts

        #punct_consts
    };

    write_source(file, source);
}
