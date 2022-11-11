use owning_ref::RcRef;
use pest::{iterators::Pair};

use super::{statement::{Statement, StatementError}, parser::Rule, span::{Span, IntoSpan}};

#[derive(Debug, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub span: Span
}

impl Block {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Result<Self, StatementError> {
        let span = pair.as_span().into_span(input.clone());

        let mut items = vec![];

        for item in pair.into_inner() {
            let item = match item.as_rule() {
                Rule::block => {
                    let block = Self::parse(item, input.clone())?;
                    BlockItem::Block(block)
                },
                Rule::statement => {
                    let statement = Statement::parse(item, input.clone())?;
                    BlockItem::Statement(statement)
                }
                _ => unreachable!()
            };

            items.push(item);
        }

        Ok(
            Block {
                items,
                span
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Block(Block),
    Statement(Statement)
}