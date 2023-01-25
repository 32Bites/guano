include!(concat!(env!("OUT_DIR"), "/generated_nodes.rs"));

impl Literal {
    pub fn is_float(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::LIT_FLOAT
    }

    pub fn is_integer(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::LIT_INTEGER
    }

    pub fn is_string(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::LIT_STRING
    }

    pub fn is_char(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::LIT_CHAR
    }

    pub fn is_true(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::KW_TRUE
    }

    pub fn is_false(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::KW_FALSE
    }

    pub fn is_nil(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::KW_NIL
    }

    pub fn is_nan(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::KW_NAN
    }

    pub fn is_inf(&self) -> bool {
        self.0.kind() == crate::SyntaxKind::KW_INF
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
