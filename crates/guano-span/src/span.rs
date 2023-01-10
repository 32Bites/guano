use std::{
    hash::Hash,
    ops::{Deref, RangeFrom, RangeTo},
    str::FromStr,
};

use nom::{
    AsBytes, Compare, ExtendInto, FindSubstring, FindToken, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, ParseTo, Slice,
};
use nom_locate::LocatedSpan;

use crate::{input::Input, Display};

#[derive(Debug, Clone)]
pub struct Span<X>(LocatedSpan<Input, X>);

impl<X: Default> Default for Span<X> {
    fn default() -> Self {
        Self(Input::default().into())
    }
}

impl<X> Span<X> {
    pub fn display(&self) -> Display<'_, Self> {
        Display(self)
    }
}

impl<X> From<LocatedSpan<Input, X>> for Span<X> {
    fn from(l: LocatedSpan<Input, X>) -> Self {
        Span(l)
    }
}

impl<X> From<Span<X>> for LocatedSpan<Input, X> {
    fn from(s: Span<X>) -> Self {
        s.0
    }
}

impl<X: Default> From<Input> for Span<X> {
    fn from(i: Input) -> Self {
        Span(i.into())
    }
}

impl<X> Deref for Span<X> {
    type Target = LocatedSpan<Input, X>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<X> AsRef<LocatedSpan<Input, X>> for Span<X> {
    fn as_ref(&self) -> &LocatedSpan<Input, X> {
        &self.0
    }
}

impl<X> AsRef<Input> for Span<X> {
    fn as_ref(&self) -> &Input {
        self.0.fragment()
    }
}

impl<X> AsRef<str> for Span<X> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<X> AsRef<[u8]> for Span<X> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<X> AsBytes for Span<X> {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<B, X> Compare<B> for Span<X>
where
    Input: Compare<B>,
    B: Into<LocatedSpan<B>>,
{
    fn compare(&self, t: B) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: B) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}

impl<X> ExtendInto for Span<X> {
    type Item = <Input as ExtendInto>::Item;

    type Extender = <Input as ExtendInto>::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.0.new_builder()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        self.0.extend_into(acc)
    }
}

impl<U, X> FindSubstring<U> for Span<X>
where
    Input: FindSubstring<U>,
{
    fn find_substring(&self, substr: U) -> Option<usize> {
        self.0.find_substring(substr)
    }
}

impl<U, X> FindToken<U> for Span<X>
where
    Input: FindToken<U>,
{
    fn find_token(&self, token: U) -> bool {
        self.0.find_token(token)
    }
}

impl<X> InputIter for Span<X> {
    type Item = <Input as InputIter>::Item;

    type Iter = <Input as InputIter>::Iter;

    type IterElem = <Input as InputIter>::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.0.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.0.slice_index(count)
    }
}

impl<X> InputLength for Span<X> {
    fn input_len(&self) -> usize {
        self.0.input_len()
    }
}

impl<X: Clone, R> Slice<R> for Span<X>
where
    Input: Slice<R>,
{
    fn slice(&self, range: R) -> Self {
        Span(LocatedSpan::slice(&self.0, range))
    }
}

impl<X: Clone> InputTake for Span<X>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        Span(LocatedSpan::take(&self.0, count))
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (left, right) = LocatedSpan::take_split(&self.0, count);
        (Span(left), Span(right))
    }
}

impl<X: Clone> InputTakeAtPosition for Span<X> {
    type Item = <Input as InputTakeAtPosition>::Item;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.0.fragment().position(predicate) {
            Some(n) => Ok(Span::take_split(self, n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.fragment().position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.split_at_position(predicate) {
            Err(nom::Err::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            res => res,
        }
    }

    fn split_at_position1_complete<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.fragment().position(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.fragment().input_len() == 0 {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl<X> Offset for Span<X> {
    fn offset(&self, second: &Self) -> usize {
        self.0.offset(&second)
    }
}

impl<X, R: FromStr> ParseTo<R> for Span<X> {
    fn parse_to(&self) -> Option<R> {
        self.0.parse_to()
    }
}

impl<X> PartialEq for Span<X> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<X> Eq for Span<X> {}

impl<X> PartialOrd for Span<X> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<X> Ord for Span<X> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<X> Hash for Span<X> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
