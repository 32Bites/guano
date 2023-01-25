use guano_syntax::{nodes::Expr, Node};

use crate::ast::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

pub mod block;
pub mod operator;
pub mod pratt;
pub mod primary;

pub fn expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    alternation((pratt::pratt_expr,)).parse(context)
}

pub trait ExprExt {
    fn is_block(&self) -> bool;
}

impl ExprExt for Expr {
    fn is_block(&self) -> bool {
        match self {
            Expr::Block(_)
            | Expr::IfExpr(_)
            | Expr::LoopExpr(_)
            | Expr::WhileExpr(_)
            | Expr::ForExpr(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use guano_syntax::SyntaxNode;

    use crate::ast::parsing::{parsers::ignorable::IgnorableParser, ParseContext, Parser};

    #[test]
    fn test_binary() {
        let source = r"
        1.5 + 90 * 400 - 2  [path / 100 /* THis is a comment*** */ // Comment
        ]
        ";

        let mut context = ParseContext::new(source);

        match super::expr.padded().parse(&mut context) {
            Ok((_, node, _)) => {
                let syntax_node = SyntaxNode::new_root(node.into_node().unwrap());

                println!("{syntax_node:#?}");
            }
            Err(err) => println!("Error: {err}"),
        }
    }
}
