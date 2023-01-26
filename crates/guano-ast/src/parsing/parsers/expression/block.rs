mod r#for;
mod r#if;
mod r#loop;
mod statement;
mod r#while;

pub use r#for::*;
pub use r#if::*;
pub use r#loop::*;
pub use r#while::*;
pub use statement::*;

use guano_syntax::{consts::Punctuation, node, Child, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::{expression::expr, ignorable::IgnorableParser},
    ParseContext, Parser,
};

use statement::statement;

pub fn block<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (l_curly, statements, end_expr, r_curly) = tuple((
        Punctuation::LEFT_CURLY,
        statement.padded().repeated(),
        expr.optional().padded(),
        Punctuation::RIGHT_CURLY.expected(),
    ))
    .parse(context)?;

    let mut children = vec![l_curly];
    for (l_ws, statement, r_ws) in statements {
        children.extend(l_ws);
        children.push(statement);
        children.extend(r_ws);
    }

    let (l_ws, end_expr, r_ws) = end_expr;
    children.extend(l_ws);
    if let Some(expr) = end_expr {
        children.push(expr);
    }
    children.extend(r_ws);
    children.push(r_curly);

    let block = node(SyntaxKind::BLOCK, children);

    Ok(block)
}

#[cfg(test)]
mod test {
    use guano_common::rowan::ast::AstNode;
    use guano_syntax::{
        nodes::{Expr, Statement},
        SyntaxNode,
    };

    use crate::parsing::{parsers::ignorable::IgnorableParser, ParseContext, Parser};

    #[test]
    fn test_block() {
        let source = r#"
        {   
            let variable = "";
            let variable;
            let variable: string = "" - 100;
            let variable: uint;
            flskemlkmes; 
            fs;
            feslkf;
            les;
            reee; 
            100;
            { this is; another;
                block }
        } "#;
        let mut context = ParseContext::new(source);

        match super::block.padded().parse(&mut context) {
            Ok((_, node, _)) => {
                let syntax_node = SyntaxNode::new_root(node.into_node().unwrap());
                // println!("{syntax_node:#?}");
                let expr = Expr::cast(syntax_node).expect("Somehow not an expression");

                if let Expr::Block(block) = expr {
                    for (i, statement) in block.iter().enumerate() {
                        println!("Statements[{i}]: {}", statement.syntax().to_string());

                        if let Statement::Var(var) = &statement {
                            println!(
                                "\tVariable name: {}",
                                var.name().as_ref().map(|n| n.as_str()).unwrap_or("None")
                            );

                            println!(
                                "\tVariable type: {}",
                                var.ty()
                                    .as_ref()
                                    .map(|t| t.syntax().to_string())
                                    .unwrap_or("None".into())
                            );

                            println!(
                                "\tVariable value: {}",
                                var.value()
                                    .as_ref()
                                    .map(|v| v.syntax().to_string())
                                    .unwrap_or("None".into())
                            );
                        }
                    }

                    if let Some(end_expr) = block.end_expr() {
                        println!("End Expression: {:?}", end_expr.syntax().to_string());
                    }
                } else {
                    panic!("Somehow not a block");
                }
            }
            Err(err) => {
                let span = err.span.unwrap();
                println!("39..40 {:?}", &source[39..40]);
                println!("Error for {:?}: {err}", &source[span]);
            }
        }
    }
}
