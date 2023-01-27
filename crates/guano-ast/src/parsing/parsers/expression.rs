use guano_syntax::{nodes::Expr, Child};

use crate::parsing::{combinators::alternation, error::Res, ParseContext, Parser};

use self::block::{for_expr, if_expr, loop_expr, while_expr};

pub mod block;
pub mod operator;
pub mod pratt;
pub mod primary;

pub fn expr<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    alternation((pratt::pratt, loop_expr, for_expr, while_expr, if_expr)).parse(context)
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

    use crate::parsing::{parsers::ignorable::IgnorableParser, ParseContext, Parser};

    #[test]
    fn test_binary() {
        let source = r#" "My name is " + "Noah" "#;
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
