include!(concat!(env!("OUT_DIR"), "/generated_nodes.rs"));

use crate::tokens::Iden;
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
    #[inline]
    pub fn float(&self) -> Option<tokens::Float> {
        self.float_token().and_then(tokens::Float::cast)
    }

    #[inline]
    pub fn integer(&self) -> Option<tokens::Integer> {
        self.integer_token().and_then(tokens::Integer::cast)
    }

    #[inline]
    pub fn char(&self) -> Option<tokens::Char> {
        self.char_token().and_then(tokens::Char::cast)
    }

    #[inline]
    pub fn string(&self) -> Option<tokens::String> {
        self.string_token().and_then(tokens::String::cast)
    }

    #[inline]
    pub fn boolean(&self) -> Option<bool> {
        self.true_token()
            .map(|_| true)
            .or(self.false_token().map(|_| false))
    }
}

impl BinaryExpr {
    #[inline]
    pub fn lhs(&self) -> Option<Expr> {
        self.exprs().next()
    }

    #[inline]
    pub fn rhs(&self) -> Option<Expr> {
        self.exprs().nth(1)
    }
}

impl IndexExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> {
        self.exprs().next()
    }

    #[inline]
    pub fn index(&self) -> Option<Expr> {
        self.exprs().nth(1)
    }
}

impl Func {
    #[inline]
    pub fn is_pub(&self) -> bool {
        self.pub_token().is_some()
    }

    #[inline]
    pub fn is_veto(&self) -> bool {
        self.veto_token().is_some()
    }
    #[inline]
    pub fn is_static(&self) -> bool {
        self.static_token().is_some()
    }

    #[inline]
    pub fn name(&self) -> Option<Iden> {
        self.iden_token().and_then(|i| Iden::cast(i))
    }

    // Return an iterator over all valid parameters
    pub fn params(&self) -> Option<impl Iterator<Item = (Iden, Type)>> {
        self.func_params().map(|p| {
            p.func_params().filter_map(|p| {
                let name = p.iden_token().and_then(|i| Iden::cast(i));

                if let (Some(name), Some(ty)) = (name, p.ty()) {
                    Some((name, ty))
                } else {
                    None
                }
            })
        })
    }

    #[inline]
    pub fn ty(&self) -> Option<Type> {
        self.func_type().and_then(|t| t.ty())
    }

    #[inline]
    pub fn block(&self) -> Option<Block> {
        self.func_body().and_then(|b| b.block())
    }
}

impl VarKind {
    #[inline]
    pub fn is_let(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_LET
    }

    #[inline]
    pub fn is_var(&self) -> bool {
        self.0.kind() == SyntaxKind::KW_VAR
    }
}

impl Var {
    #[inline]
    pub fn is_pub(&self) -> bool {
        self.pub_token().is_some()
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.pub_token().is_some()
    }

    #[inline]
    pub fn is_let(&self) -> bool {
        self.var_kind().map_or(false, |k| k.is_let())
    }

    #[inline]
    pub fn is_var(&self) -> bool {
        self.var_kind().map_or(false, |k| k.is_var())
    }

    #[inline]
    pub fn ty(&self) -> Option<Type> {
        self.var_type().and_then(|t| t.ty())
    }

    #[inline]
    pub fn value(&self) -> Option<Expr> {
        self.var_value().and_then(|v| v.expr())
    }

    #[inline]
    pub fn name(&self) -> Option<Iden> {
        self.iden_token().and_then(|t| Iden::cast(t))
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

    #[inline]
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
