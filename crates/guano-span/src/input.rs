use std::{
    borrow::Cow,
    mem,
    ops::{Deref, Range, RangeFrom, RangeFull, RangeTo},
    str::{CharIndices, Chars, FromStr},
    sync::Arc,
};

use guano_files::file::File;
use nom::{
    AsBytes, Compare, ExtendInto, FindSubstring, FindToken, InputIter, InputLength, InputTake,
    InputTakeAtPosition, Offset, ParseTo, Slice,
};
use owning_ref::OwningRef;

use crate::Display;

#[derive(Debug, Clone)]
pub struct Input(InputInner);

impl Input {
    pub fn display(&self) -> Display<'_, Self> {
        Display(self)
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Input {}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Input {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl std::hash::Hash for Input {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

#[derive(Debug, Clone)]
enum InputInner {
    String(OwningRef<Arc<str>, str>),
    File(OwningRef<File, str>),
}

impl Input {
    #[inline]
    pub fn file(file: File) -> Self {
        Input(InputInner::File(OwningRef::new(file)))
    }

    #[inline]
    pub fn string(string: impl Into<String>) -> Self {
        Input(InputInner::String(OwningRef::new(string.into().into())))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            InputInner::String(s) => &s,
            InputInner::File(f) => &f,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        match &self.0 {
            InputInner::String(s) => s.as_bytes(),
            InputInner::File(f) => f.as_bytes(),
        }
    }

    #[inline]
    pub fn inner_file(&self) -> Option<&File> {
        match &self.0 {
            InputInner::String(_) => None,
            InputInner::File(f) => Some(f.as_owner()),
        }
    }

    #[inline]
    pub fn inner_str(&self) -> Option<&str> {
        match &self.0 {
            InputInner::String(s) => Some(s.as_owner()),
            InputInner::File(_) => None,
        }
    }

    pub fn chars(&self) -> InputChars {
        let inner: Chars<'static> = unsafe { mem::transmute(self.as_str().chars()) };

        InputChars(self.clone(), inner)
    }

    pub fn char_indices(&self) -> InputCharIndices {
        let inner: CharIndices<'static> = unsafe { mem::transmute(self.as_str().char_indices()) };

        InputCharIndices(self.clone(), inner)
    }
}

impl Default for Input {
    fn default() -> Self {
        "".into()
    }
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        Self::string(s)
    }
}

impl From<String> for Input {
    fn from(s: String) -> Self {
        Self::string(s)
    }
}

impl From<&String> for Input {
    fn from(s: &String) -> Self {
        Self::string(s)
    }
}

impl From<Box<str>> for Input {
    fn from(s: Box<str>) -> Self {
        Input(InputInner::String(OwningRef::new(s.into())))
    }
}

impl From<Arc<str>> for Input {
    fn from(s: Arc<str>) -> Self {
        Input(InputInner::String(OwningRef::new(s)))
    }
}

impl From<&Arc<str>> for Input {
    fn from(s: &Arc<str>) -> Self {
        Input(InputInner::String(OwningRef::new(s.clone())))
    }
}

impl From<Cow<'_, str>> for Input {
    fn from(s: Cow<'_, str>) -> Self {
        Input(InputInner::String(OwningRef::new(s.into())))
    }
}

impl From<char> for Input {
    fn from(s: char) -> Self {
        Self::string(s)
    }
}

impl From<File> for Input {
    fn from(f: File) -> Self {
        Self::file(f)
    }
}

impl From<&File> for Input {
    fn from(f: &File) -> Self {
        Self::file(f.clone())
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Input {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for Input {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsBytes for Input {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl Compare<&[u8]> for Input {
    fn compare(&self, t: &[u8]) -> nom::CompareResult {
        self.as_str().compare(t)
    }

    fn compare_no_case(&self, t: &[u8]) -> nom::CompareResult {
        self.as_str().compare_no_case(t)
    }
}

impl Compare<&str> for Input {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.as_str().compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.as_str().compare_no_case(t)
    }
}

impl Slice<Range<usize>> for Input {
    fn slice(&self, range: Range<usize>) -> Self {
        Input(match &self.0 {
            InputInner::String(s) => InputInner::String(s.clone().map(|s| &s[range])),
            InputInner::File(f) => InputInner::File(f.clone().map(|f| &f[range])),
        })
    }
}

impl Slice<RangeTo<usize>> for Input {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Input(match &self.0 {
            InputInner::String(s) => InputInner::String(s.clone().map(|s| &s[range])),
            InputInner::File(f) => InputInner::File(f.clone().map(|f| &f[range])),
        })
    }
}

impl Slice<RangeFrom<usize>> for Input {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Input(match &self.0 {
            InputInner::String(s) => InputInner::String(s.clone().map(|s| &s[range])),
            InputInner::File(f) => InputInner::File(f.clone().map(|f| &f[range])),
        })
    }
}

impl Slice<RangeFull> for Input {
    fn slice(&self, range: RangeFull) -> Self {
        Input(match &self.0 {
            InputInner::String(s) => InputInner::String(s.clone().map(|s| &s[range])),
            InputInner::File(f) => InputInner::File(f.clone().map(|f| &f[range])),
        })
    }
}

impl ExtendInto for Input {
    type Item = char;

    type Extender = String;

    fn new_builder(&self) -> Self::Extender {
        String::new()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        acc.push_str(&self);
    }
}

impl FindSubstring<&str> for Input {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.as_str().find_substring(substr)
    }
}

impl FindToken<char> for Input {
    fn find_token(&self, token: char) -> bool {
        self.as_str().find_token(token)
    }
}

impl FindToken<u8> for Input {
    fn find_token(&self, token: u8) -> bool {
        self.as_str().find_token(token)
    }
}

impl FindToken<&u8> for Input {
    fn find_token(&self, token: &u8) -> bool {
        self.as_str().find_token(token)
    }
}

impl InputLength for Input {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl InputTake for Input {
    fn take(&self, count: usize) -> Self {
        self.slice(0..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(..count), self.slice(count..))
    }
}

impl InputTakeAtPosition for Input {
    type Item = char;

    fn split_at_position<P, E: nom::error::ParseError<Self>>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
    {
        match self.find(predicate) {
            Some(i) => Ok(self.take_split(i)),
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
        match self.find(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(i) => Ok(self.take_split(i)),
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
        match self.find(predicate) {
            Some(i) => Ok(self.take_split(i)),
            None => Ok(self.take_split(self.input_len())),
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
        match self.find(predicate) {
            Some(0) => Err(nom::Err::Error(E::from_error_kind(self.clone(), e))),
            Some(i) => Ok(self.take_split(i)),
            None => {
                if self.is_empty() {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                } else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl InputIter for Input {
    type Item = char;

    type Iter = InputCharIndices;

    type IterElem = InputChars;

    fn iter_indices(&self) -> Self::Iter {
        self.char_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.as_str().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.as_str().slice_index(count)
    }
}

impl Offset for Input {
    fn offset(&self, second: &Self) -> usize {
        self.as_str().offset(second.as_str())
    }
}

impl<R: FromStr> ParseTo<R> for Input {
    fn parse_to(&self) -> Option<R> {
        self.as_str().parse().ok()
    }
}

#[derive(Debug, Clone)]
pub struct InputCharIndices(Input, CharIndices<'static>);

impl Iterator for InputCharIndices {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next()
    }
}

#[derive(Debug, Clone)]
pub struct InputChars(Input, Chars<'static>);

impl Iterator for InputChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next()
    }
}
