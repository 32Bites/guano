use pretty::{Arena, Pretty, RcAllocator};

use super::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Display<T>(T, usize);

impl<'a, T> std::fmt::Display for Display<T>
where
    T: Pretty<'a, RcAllocator, ()> + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let doc = self.0.clone().pretty(&RcAllocator).into_doc();
        doc.render_fmt(self.1, f)?;

        Ok(())
    }
}

pub trait ToDisplay<'a>: Pretty<'a, RcAllocator, ()> + Clone  {
    #[inline]
    fn display_width(self, width: usize) -> Display<Self> {
        Display(self, width)
    }

    #[inline]
    fn display(self) -> Display<Self> {
        self.display_width(usize::MAX)
    }
}

macro_rules! display_pretty {
    ($t:ty) => (

        impl<'a> $crate::ast::display::ToDisplay<'a> for &'a $t {}

        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.display().fmt(f)
            }
        }
    );

    ($t:ty, $($ts:ty),+) => (
        display_pretty!($t);
        display_pretty! ( $($ts),+ );
    );
}

display_pretty!(Iden, Path, Type, Expr, ExprKind);
