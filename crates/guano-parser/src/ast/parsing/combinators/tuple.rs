use impl_trait_for_tuples::impl_for_tuples;

use crate::ast::parsing::{Parser, ParseContext};

#[derive(Debug, Clone, Copy)]
pub struct Tuple<T> {
    tuple: T,
}

pub trait TupleTrait<'source> {
    type Output;
    type Error;

    fn tuple_parse(self, parser: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error>;
}

pub fn tuple<'source, T>(tuple: T) -> Tuple<T>
where
    T: TupleTrait<'source>,
{
    Tuple { tuple }
}

impl<'source, T> Parser<'source> for Tuple<T>
where
    T: TupleTrait<'source>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        self.tuple.tuple_parse(context)
    }
}

#[impl_for_tuples(1, 10)]
#[tuple_types_custom_trait_bound(Parser<'source, Error = E>)]
impl<'source, E> TupleTrait<'source> for Tuple {
    type Error = E;
    for_tuples!( type Output = ( #(Tuple::Output ),* ); );

    fn tuple_parse(
        self,
        context: &mut ParseContext<'source>,
    ) -> Result<Self::Output, Self::Error> {
        let output = for_tuples!( ( #( Tuple::parse(self.Tuple, context)? ),* ) );

        Ok(output)
    }
}
