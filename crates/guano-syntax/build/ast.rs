use build_helper::warning;
use heck::{ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase};
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

        if let Err(err) = Self::lower_rule(&mut fields, grammar, None, rule) {
            warning!("Error constructing node: {err}");
            return None;
        }

        let fields = fields.into_iter().unique().collect();

        Some(Self::Struct {
            name: name.to_owned(),
            fields,
            traits: vec![],
        })
    }

    fn lower_rule(
        fields: &mut Vec<Field>,
        grammar: &Grammar,
        label: Option<&String>,
        rule: &Rule,
    ) -> Result<(), String> {
        match rule {
            Rule::Labeled { label, rule } => Self::lower_rule(fields, grammar, Some(label), rule)?,
            Rule::Node(node) => {
                let ty = grammar[*node].name.to_owned();
                let name = label.cloned().unwrap_or_else(|| ty.to_lower_camel_case());
                let field = Field::Node {
                    name,
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
                    name: label.cloned(),
                    ty: name.to_pascal_case(),
                };
                fields.push(field);
            }
            Rule::Alt(rules) | Rule::Seq(rules) => {
                for rule in rules {
                    Self::lower_rule(fields, grammar, label, rule)?;
                }
            }
            Rule::Opt(rule) => Self::lower_rule(fields, grammar, label, rule)?,
            Rule::Rep(rule) => {
                if let Rule::Node(node) = &**rule {
                    let ty = grammar[*node].name.clone();
                    let name = label
                        .cloned()
                        .unwrap_or_else(|| format!("{}s", ty.to_lower_camel_case()));
                    let field = Field::Node {
                        name,
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
                        #(crate::SyntaxKind::#kinds => true,)*
                        _ => false,
                    }
                }

                fn cast(syntax: crate::SyntaxNode) -> Option<Self> {
                    Some(match syntax.kind() {
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
        name: Option<String>,
        ty: String,
    },
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

impl Field {
    pub fn method(&self) -> TokenStream {
        let method_name = self.method_name();
        let ty = self.ty();
        match self {
            Field::Token { name: _, ty: _ } => {
                let token_kind = self.token_kind().unwrap();

                quote! {
                    #[inline]
                    pub fn #method_name(&self) -> Option<#ty> {
                        ::guano_common::rowan::ast::support::token(&self.0, #token_kind)
                    }
                }
            }
            Field::Node {
                name: _,
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
                name: _,
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

    pub fn ty(&self) -> TokenStream {
        match self {
            Field::Token { name: _, ty: _ } => quote! { crate::SyntaxToken },
            Field::Node {
                name: _,
                ty,
                cardinality: _,
            } => {
                let iden = format_ident!("{ty}");

                quote! { #iden }
            }
        }
    }

    pub fn token_kind(&self) -> Option<TokenStream> {
        match &self {
            Field::Token { name: _, ty } => {
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
                name: _,
                ty: _,
                cardinality: _,
            } => None,
        }
    }

    pub fn method_name(&self) -> Ident {
        match &self {
            Field::Token { name: _, ty } => {
                format_ident!("{}_token", ty.trim_start_matches("Lit").to_snake_case())
            }
            Field::Node {
                name,
                ty: _,
                cardinality: _,
            } => {
                let name = name.to_snake_case();
                if &*name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{name}")
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cardinality {
    Optional,
    Many,
}
