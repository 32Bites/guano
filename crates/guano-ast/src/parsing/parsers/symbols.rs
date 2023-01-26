use guano_syntax::Child;

use crate::parsing::{error::Res, ParseContext};

use self::{
    identifier::{iden, raw_iden},
    keyword::keyword,
    path::{name, path, path_segment},
};

pub mod identifier;
pub mod keyword;
pub mod path;
pub mod ty;

pub trait Symbols<'source> {
    fn name(&mut self) -> Res<'source, Child>;
    fn path_segment(&mut self) -> Res<'source, Child>;
    fn path(&mut self) -> Res<'source, Child>;
    fn keyword(&mut self) -> Res<'source, Child>;
    fn iden(&mut self) -> Res<'source, Child>;
    fn raw_iden(&mut self) -> Res<'source>;
}

impl<'source> Symbols<'source> for ParseContext<'source> {
    #[inline]
    fn name(&mut self) -> Res<'source, Child> {
        name(self)
    }

    #[inline]
    fn path_segment(&mut self) -> Res<'source, Child> {
        path_segment(self)
    }

    #[inline]
    fn path(&mut self) -> Res<'source, Child> {
        path(self)
    }

    #[inline]
    fn keyword(&mut self) -> Res<'source, Child> {
        keyword(self)
    }

    #[inline]
    fn iden(&mut self) -> Res<'source, Child> {
        iden(self)
    }

    #[inline]
    fn raw_iden(&mut self) -> Res<'source> {
        raw_iden(self)
    }
}
