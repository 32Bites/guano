use impl_trait_for_tuples::impl_for_tuples;

use crate::ast::parser::{Parser, ParserContext};

#[derive(Debug, Clone, Copy)]
pub struct Alternation<A> {
    alternation: A,
}

pub fn alternation<'source, A>(alternation: A) -> Alternation<A>
where
    A: AlternationTrait<'source>,
{
    Alternation { alternation }
}

impl<'source, A> Parser<'source> for Alternation<A>
where
    A: AlternationTrait<'source>,
{
    type Output = A::Output;

    type Error = A::Error;

    fn parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        self.alternation.alt_parse(context)
    }
}

pub trait AlternationTrait<'source> {
    type Output;
    type Error;

    fn alt_parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error>;
}

#[impl_for_tuples(1, 10)]
#[tuple_types_custom_trait_bound(Parser<'source, Output = O, Error = E>)]
impl<'source, O, E> AlternationTrait<'source> for Tuple {
    type Error = E;
    type Output = O;

    fn alt_parse(self, context: &mut ParserContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut temp_context;
        let mut error;
        for_tuples!(
            #(
                temp_context = context.clone();
                match Tuple::parse(self.Tuple, &mut temp_context) {
                    Err(err) => error = err,
                    Ok(output) => {
                        context.input = temp_context.input;
                        context.errors = temp_context.errors;
                        return Ok(output);
                    },
                }
            )*
        );

        Err(error)
    }
}
