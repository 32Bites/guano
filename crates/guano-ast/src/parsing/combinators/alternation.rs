use impl_trait_for_tuples::impl_for_tuples;

use crate::parsing::{ParseContext, Parser};

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

    fn parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        self.alternation.alt_parse(context)
    }
}

pub trait AlternationTrait<'source> {
    type Output;
    type Error;

    fn alt_parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error>;
}

impl<'source, P> AlternationTrait<'source> for &[P]
where
    P: Parser<'source> + Clone,
{
    type Output = P::Output;
    type Error = P::Error;

    fn alt_parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        assert_ne!(self.len(), 0);
        let mut error = None;

        for parser in self {
            let mut temp_context = context.clone();

            match parser.clone().parse(&mut temp_context) {
                Ok(output) => {
                    *context.position_mut() = temp_context.position();
                    *context.errors_mut() = temp_context.into_errors();
                    return Ok(output);
                }
                Err(err) => error = Some(err),
            }
        }

        Err(error.unwrap())
    }
}

impl<'source, P, const N: usize> AlternationTrait<'source> for [P; N]
where
    P: Parser<'source> + Clone,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn alt_parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        alternation(&self as &[P]).parse(context)
    }
}

#[impl_for_tuples(1, 10)]
#[tuple_types_custom_trait_bound(Parser<'source, Output = O, Error = E>)]
impl<'source, O, E> AlternationTrait<'source> for Tuple {
    type Error = E;
    type Output = O;

    fn alt_parse(self, context: &mut ParseContext<'source>) -> Result<Self::Output, Self::Error> {
        let mut temp_context;
        let mut error;
        for_tuples!(
            #(
                temp_context = context.clone();
                match Tuple::parse(self.Tuple, &mut temp_context) {
                    Err(err) => error = err,
                    Ok(output) => {
                        *context.position_mut() = temp_context.position();
                        *context.errors_mut() = temp_context.into_errors();
                        return Ok(output);
                    },
                }
            )*
        );

        Err(error)
    }
}
