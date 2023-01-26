include!(concat!(env!("OUT_DIR"), "/generated_nodes.rs"));

use std::iter::FusedIterator;
use std::iter::Peekable;

use guano_common::rowan::ast::AstChildren;

use crate::SyntaxKind;

impl Literal {
    pub fn is_float(&self) -> bool {
        self.0.kind() == SyntaxKind::LIT_FLOAT
    }

    pub fn is_integer(&self) -> bool {
        self.0.kind() == SyntaxKind::LIT_INTEGER
    }

    pub fn is_string(&self) -> bool {
        self.0.kind() == SyntaxKind::LIT_STRING
    }

    pub fn is_char(&self) -> bool {
        self.0.kind() == SyntaxKind::LIT_CHAR
    }

    pub fn is_true(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_TRUE
    }

    pub fn is_false(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_FALSE
    }

    pub fn is_nil(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_NIL
    }

    pub fn is_nan(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_NAN
    }

    pub fn is_inf(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_INF
    }
}

use crate::AstToken;

use super::tokens;

impl Literal {
    pub fn float(&self) -> Option<tokens::Float> {
        self.float_token().and_then(tokens::Float::cast)
    }

    pub fn integer(&self) -> Option<tokens::Integer> {
        self.integer_token().and_then(tokens::Integer::cast)
    }

    pub fn char(&self) -> Option<tokens::Char> {
        self.char_token().and_then(tokens::Char::cast)
    }

    pub fn string(&self) -> Option<tokens::String> {
        self.string_token().and_then(tokens::String::cast)
    }

    pub fn boolean(&self) -> Option<bool> {
        self.true_token()
            .map(|_| true)
            .or(self.false_token().map(|_| false))
    }
}

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.exprs().next()
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.exprs().nth(1)
    }
}

impl IndexExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.exprs().next()
    }

    pub fn index(&self) -> Option<Expr> {
        self.exprs().nth(1)
    }
}

impl VarKind {
    pub fn is_let(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_LET
    }

    pub fn is_var(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_VAR
    }
}

impl VarQualifiers {
    pub fn has_pub(&self) -> bool {
        self.pub_token().is_some()
    }

    pub fn has_static(&self) -> bool {
        self.static_token().is_some()
    }
}

impl Var {
    pub fn is_pub(&self) -> bool {
        self.var_qualifiers().map_or(false, |q| q.has_pub())
    }

    pub fn is_static(&self) -> bool {
        self.var_qualifiers().map_or(false, |q| q.has_static())
    }

    pub fn is_let(&self) -> bool {
        self.var_kind().map_or(false, |k| k.is_let())
    }

    pub fn is_var(&self) -> bool {
        self.var_kind().map_or(false, |k| k.is_var())
    }

    pub fn ty(&self) -> Option<Type> {
        self.var_type().and_then(|t| t.ty())
    }

    pub fn value(&self) -> Option<Expr> {
        self.var_value().and_then(|v| v.expr())
    }

    pub fn name(&self) -> Option<super::tokens::Iden> {
        self.iden_token().and_then(|t| super::tokens::Iden::cast(t))
    }
}

impl Block {
    pub fn end_expr(&self) -> Option<Expr> {
        self.expr().or_else(|| {
            self.statements().last().and_then(|s| match s {
                Statement::ExprStatement(expr) if expr.semicolon_token().is_none() => expr.expr(),
                _ => None,
            })
        })
    }

    pub fn iter(&self) -> Statements {
        Statements {
            children: Some(self.statements().peekable()),
            has_end: self.expr().is_some(),
        }
    }
}

#[derive(Debug, Clone)]
/// Iterator over a block's statements, excluding the ending expression if the
/// syntax tree considers it a statement.
pub struct Statements {
    children: Option<Peekable<AstChildren<Statement>>>,
    has_end: bool,
}

impl Iterator for Statements {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.children.as_mut()?;

        if self.has_end {
            let res = iter.next();

            if res.is_none() {
                self.children = None;
            }

            res
        } else {
            let res = iter.next()?;

            if iter.peek().is_none() {
                self.children = None;
                let has_semicolon = match &res {
                    Statement::ExprStatement(expr_stmt) => expr_stmt.semicolon_token().is_some(),
                    _ => true,
                };

                if has_semicolon {
                    Some(res)
                } else {
                    None
                }
            } else {
                Some(res)
            }
        }
    }
}

impl FusedIterator for Statements {}
