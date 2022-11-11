use super::{statement::Statement, Desugar};
use crate::parser::{
    block::{Block as ParseBlock, BlockItem as ParseBlockItem},
    statement::{Statement as ParseStatement},
    span::Span,
};

#[derive(Debug, Default, Clone)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub span: Option<Span>,
}

impl Desugar for ParseBlock {
    type Unsweetened = Block;

    fn desugar(self) -> Self::Unsweetened {
        Block {
            span: Some(self.span),
            items: self.items.into_iter().map(|i| i.desugar()).collect(),
        }
    }
}

impl<Item: Into<BlockItem>, I: IntoIterator<Item = Item>> From<I> for Block {
    fn from(iter: I) -> Self {
        Self {
            span: None,
            items: iter.into_iter().map(|i| i.into()).collect()
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockItem {
    Block(Block),
    Statement(Statement),
}

impl From<Block> for BlockItem {
    fn from(b: Block) -> Self {
        BlockItem::Block(b)
    }
}

impl From<Statement> for BlockItem {
    fn from(s: Statement) -> Self {
        BlockItem::Statement(s)
    }
}

impl From<ParseBlock> for BlockItem {
    fn from(b: ParseBlock) -> Self {
        BlockItem::Block(b.desugar())
    }
}

impl From<ParseStatement> for BlockItem {
    fn from(s: ParseStatement) -> Self {
        BlockItem::Statement(s.desugar())
    }
}

impl Desugar for ParseBlockItem {
    type Unsweetened = BlockItem;

    fn desugar(self) -> Self::Unsweetened {
        match self {
            ParseBlockItem::Block(b) => b.into(),
            ParseBlockItem::Statement(s) => s.into(),
        }
    }
}
