use std::{
    borrow::Cow,
    cmp::Ordering,
    mem,
    ops::{Index, IndexMut},
    rc::Rc,
    str::{
        Bytes, CharIndices, Chars, EscapeDebug, EscapeDefault, EscapeUnicode, Lines,
        SplitAsciiWhitespace, SplitWhitespace,
    },
    sync::Arc,
};

use super::StrIndex;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
/// Wrapper for [str] that is indexable by [u32] rather than [usize].
pub struct Str(str);

impl Str {
    #[inline]
    pub unsafe fn new_unchecked<'a>(value: &'a str) -> &'a Str {
        mem::transmute(value)
    }

    #[inline]
    pub unsafe fn new_unchecked_mut<'a>(value: &'a mut str) -> &'a mut Str {
        mem::transmute(value)
    }

    #[inline]
    pub fn new<'a>(value: &'a str) -> Result<&'a Str, &'static str> {
        if value.len() > u32::MAX as usize {
            Err("The provided string cannot be indexed by a u32")
        } else {
            Ok(unsafe { Str::new_unchecked(value) })
        }
    }

    #[inline]
    pub fn new_mut<'a>(value: &'a mut str) -> Result<&'a mut Str, &'static str> {
        if value.len() > u32::MAX as usize {
            Err("The provided string cannot be indexed by a u32")
        } else {
            Ok(unsafe { Str::new_unchecked_mut(value) })
        }
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    #[inline]
    pub fn as_str_mut(&mut self) -> &mut str {
        &mut self.0
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[inline]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_ptr_mut(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    #[inline]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        let raw = Box::into_raw(self) as *mut str;

        unsafe { Box::from_raw(raw) }
    }

    #[inline]
    pub fn into_boxed_bytes(self: Box<Self>) -> Box<[u8]> {
        let raw = Box::into_raw(self) as *mut [u8];

        unsafe { Box::from_raw(raw) }
    }
}

impl Str {
    #[inline]
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        self.0.bytes()
    }

    #[inline]
    pub fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }

    #[inline]
    pub fn char_indices(&self) -> CharIndices<'_> {
        self.0.char_indices()
    }

    #[inline]
    pub fn lines(&self) -> Lines<'_> {
        self.0.lines()
    }

    #[inline]
    pub fn escape_debug(&self) -> EscapeDebug<'_> {
        self.0.escape_debug()
    }

    #[inline]
    pub fn escape_default(&self) -> EscapeDefault<'_> {
        self.0.escape_default()
    }

    #[inline]
    pub fn escape_unicode(&self) -> EscapeUnicode<'_> {
        self.0.escape_unicode()
    }

    #[inline]
    pub fn is_char_boundary(&self, index: u32) -> bool {
        self.0.is_char_boundary(index as usize)
    }

    #[inline]
    pub fn make_ascii_lowercase(&mut self) {
        self.0.make_ascii_lowercase()
    }

    #[inline]
    pub fn make_ascii_uppercase(&mut self) {
        self.0.make_ascii_uppercase()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn split_at(&self, mid: u32) -> (&Str, &Str) {
        let (l, r) = self.0.split_at(mid as usize);
        unsafe { (Self::new_unchecked(l), Self::new_unchecked(r)) }
    }

    #[inline]
    pub fn split_at_mut(&mut self, mid: u32) -> (&mut Str, &mut Str) {
        let (l, r) = self.0.split_at_mut(mid as usize);
        unsafe { (Self::new_unchecked_mut(l), Self::new_unchecked_mut(r)) }
    }

    #[inline]
    pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_> {
        self.0.split_ascii_whitespace()
    }

    #[inline]
    pub fn split_whitespace(&self) -> SplitWhitespace<'_> {
        self.0.split_whitespace()
    }

    #[inline]
    pub fn trim(&self) -> &Self {
        unsafe { Self::new_unchecked(self.0.trim()) }
    }

    #[inline]
    pub fn trim_start(&self) -> &Self {
        unsafe { Self::new_unchecked(self.0.trim_start()) }
    }

    #[inline]
    pub fn trim_end(&self) -> &Self {
        unsafe { Self::new_unchecked(self.0.trim_end()) }
    }

    #[inline]
    pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.as_str())
    }

    #[inline]
    pub fn to_lowercase(&self) -> super::String {
        super::String::from_string(self.0.to_lowercase())
    }

    #[inline]
    pub fn to_uppercase(&self) -> super::String {
        super::String::from_string(self.0.to_uppercase())
    }

    #[inline]
    pub fn to_ascii_lowercase(&self) -> super::String {
        super::String::from_string(self.0.to_ascii_lowercase())
    }

    #[inline]
    pub fn to_ascii_uppercase(&self) -> super::String {
        super::String::from_string(self.0.to_ascii_uppercase())
    }

    #[inline]
    pub fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }
}

impl AsRef<str> for &Str {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Str> for &Str {
    #[inline]
    fn as_ref(&self) -> &Str {
        self
    }
}

impl AsRef<[u8]> for &Str {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<Str> for &mut Str {
    #[inline]
    fn as_mut(&mut self) -> &mut Str {
        self
    }
}

impl AsMut<str> for &mut Str {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl PartialEq<str> for Str {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl<'a> PartialEq<Cow<'a, str>> for Str {
    #[inline]
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<Cow<'a, str>> for &Str {
    #[inline]
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.as_str() == other
    }
}

impl<'a> PartialEq<Cow<'a, Str>> for Str {
    #[inline]
    fn eq(&self, other: &Cow<'a, Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> PartialEq<Cow<'a, Str>> for &Str {
    #[inline]
    fn eq(&self, other: &Cow<'a, Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Box<Str>> for Str {
    #[inline]
    fn eq(&self, other: &Box<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Box<Str>> for &Str {
    #[inline]
    fn eq(&self, other: &Box<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Box<str>> for Str {
    #[inline]
    fn eq(&self, other: &Box<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<Box<str>> for &Str {
    #[inline]
    fn eq(&self, other: &Box<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<Rc<Str>> for Str {
    #[inline]
    fn eq(&self, other: &Rc<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Rc<Str>> for &Str {
    #[inline]
    fn eq(&self, other: &Rc<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Rc<str>> for Str {
    #[inline]
    fn eq(&self, other: &Rc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<Rc<str>> for &Str {
    #[inline]
    fn eq(&self, other: &Rc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<Arc<Str>> for Str {
    #[inline]
    fn eq(&self, other: &Arc<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Arc<Str>> for &Str {
    #[inline]
    fn eq(&self, other: &Arc<Str>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<Arc<str>> for Str {
    #[inline]
    fn eq(&self, other: &Arc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<Arc<str>> for &Str {
    #[inline]
    fn eq(&self, other: &Arc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<'a> PartialOrd<Cow<'a, str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl<'a> PartialOrd<Cow<'a, str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl<'a> PartialOrd<Cow<'a, Str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<'a> PartialOrd<Cow<'a, Str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Cow<'a, Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Box<Str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Box<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Box<Str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Box<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Box<str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Box<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<Box<str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Box<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<Rc<Str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Rc<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Rc<Str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Rc<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Rc<str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Rc<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<Rc<str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Rc<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<Arc<Str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Arc<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Arc<Str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Arc<Str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<Arc<str>> for Str {
    #[inline]
    fn partial_cmp(&self, other: &Arc<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl PartialOrd<Arc<str>> for &Str {
    #[inline]
    fn partial_cmp(&self, other: &Arc<str>) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_ref())
    }
}

impl<'a> TryFrom<&'a str> for &'a Str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Str::new(value)
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut Str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &'a mut str) -> Result<Self, Self::Error> {
        Str::new_mut(value)
    }
}

impl From<Box<Str>> for Box<str> {
    #[inline]
    fn from(s: Box<Str>) -> Self {
        s.into_boxed_str()
    }
}

impl From<Box<Str>> for Box<[u8]> {
    #[inline]
    fn from(s: Box<Str>) -> Self {
        s.into_boxed_bytes()
    }
}

impl From<Box<Str>> for super::String {
    #[inline]
    fn from(s: Box<Str>) -> Self {
        super::String::from_reference(&s)
    }
}

impl<'a> From<&'a Str> for &'a str {
    fn from(s: &'a Str) -> Self {
        s.as_str()
    }
}

impl<'a> From<&'a Str> for String {
    fn from(s: &'a Str) -> Self {
        s.to_string()
    }
}

impl<'a> From<&'a Str> for Cow<'a, Str> {
    fn from(s: &'a Str) -> Self {
        Cow::Borrowed(s)
    }
}

impl<'a> From<&'a Str> for Cow<'a, str> {
    fn from(s: &'a Str) -> Self {
        Cow::Borrowed(s.into())
    }
}

impl Default for &Str {
    fn default() -> Self {
        Str::new("").unwrap()
    }
}

impl ToOwned for Str {
    type Owned = super::String;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        super::String::from_reference(self)
    }
}

impl<I: StrIndex> Index<I> for Str {
    type Output = Str;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<I: StrIndex> IndexMut<I> for Str {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

impl std::fmt::Display for Str {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for Str {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::hash::Hash for Str {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
