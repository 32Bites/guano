use build_helper::warning;
use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase};
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use strum::VariantNames;
use ungrammar::{Grammar, Node, Rule};

use crate::{keyword::Keyword, punctuation::Punctuation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Enum {
        name: String,
        variants: Vec<String>,
        traits: Vec<String>,
    },
    Struct {
        name: String,
        fields: Vec<Field>,
        traits: Vec<String>,
    },
}

impl AstNode {
    pub fn handle(grammar: &Grammar) -> Vec<AstNode> {
        grammar
            .iter()
            .map(|node| Self::new(grammar, node))
            .flatten()
            .collect()
    }

    pub fn new(grammar: &Grammar, node: Node) -> Option<AstNode> {
        let data = &grammar[node];
        let name = &*data.name;
        let rule = &data.rule;

        Self::new_enum(grammar, name, rule).or_else(|| Self::new_struct(grammar, name, rule))
    }

    fn new_struct(grammar: &Grammar, name: &str, rule: &Rule) -> Option<AstNode> {
        let mut fields = vec![];

        if let Err(err) = Self::lower_rule(&mut fields, grammar, rule) {
            warning!("Error constructing node: {err}");
            return None;
        }

        let mut field_map = fields
            .into_iter()
            .map(|f| (f.is_token(), f))
            .into_group_map();

        let mut fields = field_map.remove(&true).unwrap_or_else(Vec::new);

        if let Some(remaining) = field_map.remove(&false) {
            let field_map = remaining
                .into_iter()
                .into_grouping_map_by(|f| (f.ty().to_string(), f.card()))
                .fold(0usize, |s, _, _| s + 1);

            let field_map = field_map.into_iter().map(|((ty, co), c)| {
                let card = if c > 1 || co == Some(Cardinality::Many) {
                    Cardinality::Many
                } else {
                    Cardinality::Optional
                };

                (ty, card)
            });

            for (ty, cardinality) in field_map {
                let field = Field::Node { ty, cardinality };

                fields.push(field);
            }
        }

        // let mut fields = field_map[true]

        Some(Self::Struct {
            name: name.to_owned(),
            fields,
            traits: vec![],
        })
    }

    fn lower_rule(fields: &mut Vec<Field>, grammar: &Grammar, rule: &Rule) -> Result<(), String> {
        match rule {
            Rule::Labeled { label: _, rule } => Self::lower_rule(fields, grammar, rule)?,
            Rule::Node(node) => {
                let ty = grammar[*node].name.to_owned();
                let field = Field::Node {
                    ty,
                    cardinality: Cardinality::Optional,
                };

                fields.push(field);
            }
            Rule::Token(token) => {
                let mut name = &*grammar[*token].name;

                if let Some((p, _)) = Punctuation::reprs().find(|(_, s)| *s == name) {
                    name = p.into();
                }
                let field = Field::Token {
                    ty: name.to_pascal_case(),
                };
                fields.push(field);
            }
            Rule::Alt(rules) | Rule::Seq(rules) => {
                for rule in rules {
                    Self::lower_rule(fields, grammar, rule)?;
                }
            }
            Rule::Opt(rule) => Self::lower_rule(fields, grammar, rule)?,
            Rule::Rep(rule) => {
                if let Rule::Node(node) = &**rule {
                    let ty = grammar[*node].name.clone();
                    let field = Field::Node {
                        ty,
                        cardinality: Cardinality::Many,
                    };
                    fields.push(field);
                } else {
                    return Err(format!("unhandled rule: {:?}", rule));
                }
            }
        }

        Ok(())
    }

    fn new_enum(grammar: &Grammar, name: &str, rule: &Rule) -> Option<AstNode> {
        let potential_variants = match rule {
            Rule::Alt(alt) => alt,
            _ => return None,
        };

        let mut variants = Vec::new();

        for potential in potential_variants {
            if let Rule::Node(node) = potential {
                let variant_name = grammar[*node].name.to_owned();
                variants.push(variant_name);
            } else {
                return None;
            }
        }

        let variants = variants.into_iter().unique().collect();

        Some(AstNode::Enum {
            name: name.to_owned(),
            variants,
            traits: vec![],
        })
    }

    pub fn token_stream(&self) -> TokenStream {
        match self {
            AstNode::Enum {
                name,
                variants,
                traits,
            } => Self::enum_token_stream(name, variants, traits),
            AstNode::Struct {
                name,
                fields,
                traits,
            } => Self::struct_token_stream(name, fields, traits),
        }
    }

    fn struct_token_stream(name: &str, fields: &[Field], _traits: &[String]) -> TokenStream {
        let kind = format_ident!("{}", name.to_shouty_snake_case());
        let doc = format!("{}", name.to_title_case());
        let name = format_ident!("{name}");
        let methods = fields.into_iter().map(|f| f.method());

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[doc = #doc]
            pub struct #name(crate::SyntaxNode);

            impl #name {
                #(#methods)*
            }

            impl ::guano_common::rowan::ast::AstNode for #name {
                type Language = crate::Lang;
                #[inline]
                fn can_cast(kind: crate::SyntaxKind) -> bool {
                    kind == crate::SyntaxKind::#kind
                }

                #[inline]
                fn cast(syntax: crate::SyntaxNode) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self(syntax))
                    } else {
                        None
                    }
                }

                #[inline]
                fn syntax(&self) -> &crate::SyntaxNode {
                    &self.0
                }
            }

            impl ::std::fmt::Display for #name {
                #[inline]
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    ::std::fmt::Display::fmt(&self.0, f)
                }
            }
        }
    }

    fn enum_token_stream(name: &str, variants: &[String], _traits: &[String]) -> TokenStream {
        let kinds = variants
            .into_iter()
            .map(|v| format_ident!("{}", v.to_shouty_snake_case()))
            .collect_vec();
        let variants = variants
            .into_iter()
            .map(|v| format_ident!("{v}"))
            .collect_vec();
        let doc = format!("{}", name.to_title_case());
        let enum_variant = format_ident!("{}", name.to_shouty_snake_case());
        let name = format_ident!("{name}");

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[doc = #doc]
            pub enum #name {
                #(#variants(#variants),)*
            }

            impl ::guano_common::rowan::ast::AstNode for #name {
                type Language = crate::Lang;
                fn can_cast(kind: crate::SyntaxKind) -> bool {
                    match kind {
                        crate::SyntaxKind::#enum_variant => true,
                        #(crate::SyntaxKind::#kinds => true,)*
                        _ => false,
                    }
                }

                fn cast(syntax: crate::SyntaxNode) -> Option<Self> {
                    Some(match syntax.kind() {
                        crate::SyntaxKind::#enum_variant => return Self::cast(syntax.first_child()?),
                        #(crate::SyntaxKind::#kinds => #name::#variants(#variants::cast(syntax).unwrap()),)*
                        _ => return None
                    })
                }

                fn syntax(&self) -> &crate::SyntaxNode {
                    match self {
                        #(#name::#variants(v) => v.syntax(),)*
                    }
                }
            }

            #(
                impl From<#variants> for #name {
                    fn from(node: #variants) -> Self {
                        Self::#variants(node)
                    }
                }
            )*
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    Token {
        ty: String,
    },
    Node {
        ty: String,
        cardinality: Cardinality,
    },
}

impl Field {
    pub fn method(&self) -> TokenStream {
        let method_name = self.method_name();
        let ty = self.ty();
        match self {
            Field::Token { ty: _ } => {
                let token_kind = self.token_kind().unwrap();

                quote! {
                    #[inline]
                    pub fn #method_name(&self) -> Option<#ty> {
                        ::guano_common::rowan::ast::support::token(&self.0, #token_kind)
                    }
                }
            }
            Field::Node {
                ty: _,
                cardinality: Cardinality::Many,
            } => {
                quote! {
                    #[inline]
                    pub fn #method_name(&self) -> ::guano_common::rowan::ast::AstChildren<#ty> {
                        ::guano_common::rowan::ast::support::children(&self.0)
                    }
                }
            }
            Field::Node {
                ty: _,
                cardinality: Cardinality::Optional,
            } => {
                quote! {
                    #[inline]
                    pub fn #method_name(&self) -> Option<#ty> {
                        ::guano_common::rowan::ast::support::child(&self.0)
                    }
                }
            }
        }
    }

    pub fn card(&self) -> Option<Cardinality> {
        match self {
            Field::Node { ty: _, cardinality } => Some(*cardinality),
            _ => None,
        }
    }

    pub fn ty(&self) -> TokenStream {
        match self {
            Field::Token { ty: _ } => quote! { crate::SyntaxToken },
            Field::Node { ty, cardinality: _ } => {
                let iden = format_ident!("{ty}");

                quote! { #iden }
            }
        }
    }

    pub fn token_kind(&self) -> Option<TokenStream> {
        match &self {
            Field::Token { ty } => {
                let is_keyword = Keyword::VARIANTS
                    .into_iter()
                    .find(|s| **s == &**ty)
                    .is_some();
                let shouty = ty.to_shouty_snake_case();
                let iden = match is_keyword {
                    true => format_ident!("KW_{shouty}"),
                    false => format_ident!("{shouty}"),
                };

                Some(quote! {
                    crate::SyntaxKind::#iden
                })
            }
            Field::Node {
                ty: _,
                cardinality: _,
            } => None,
        }
    }

    pub fn is_token(&self) -> bool {
        matches!(self, Field::Token { ty: _ })
    }

    pub fn method_name(&self) -> Ident {
        match &self {
            Field::Token { ty } => {
                format_ident!("{}_token", ty.trim_start_matches("Lit").to_snake_case())
            }
            Field::Node { ty, cardinality } => {
                let name = format!(
                    "{}{}",
                    ty.to_snake_case(),
                    if cardinality == &Cardinality::Many {
                        "s"
                    } else {
                        ""
                    }
                );
                if &*name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{name}")
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cardinality {
    Optional,
    Many,
}
