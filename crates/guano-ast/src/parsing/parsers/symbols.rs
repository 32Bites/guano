use guano_syntax::Node;

use crate::parsing::{error::Res, ParseContext};

use self::{
    iden::{iden, raw_iden},
    keyword::keyword,
    path::{name, path, path_segment},
};

pub mod iden;
pub mod keyword;
pub mod path;
pub mod ty;

pub trait Symbols<'source> {
    fn name(&mut self) -> Res<'source, Node>;
    fn path_segment(&mut self) -> Res<'source, Node>;
    fn path(&mut self) -> Res<'source, Node>;
    fn keyword(&mut self) -> Res<'source, Node>;
    fn iden(&mut self) -> Res<'source, Node>;
    fn raw_iden(&mut self) -> Res<'source>;
}

impl<'source> Symbols<'source> for ParseContext<'source> {
    #[inline]
    fn name(&mut self) -> Res<'source, Node> {
        name(self)
    }

    #[inline]
    fn path_segment(&mut self) -> Res<'source, Node> {
        path_segment(self)
    }

    #[inline]
    fn path(&mut self) -> Res<'source, Node> {
        path(self)
    }

    #[inline]
    fn keyword(&mut self) -> Res<'source, Node> {
        keyword(self)
    }

    #[inline]
    fn iden(&mut self) -> Res<'source, Node> {
        iden(self)
    }

    #[inline]
    fn raw_iden(&mut self) -> Res<'source> {
        raw_iden(self)
    }
}
