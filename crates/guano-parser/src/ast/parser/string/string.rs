use std::{
    borrow::{Borrow, BorrowMut, Cow},
    mem,
    ops::{Deref, DerefMut},
    str::{from_utf8_unchecked, from_utf8_unchecked_mut},
    string::String as StdString,
};

use super::Str;

#[derive(Debug, Clone, Default)]
pub struct String {
    inner: Vec<u8>,
}

impl String {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    /// Will panic if the mutations make the length exceed [u32::MAX]
    pub fn as_string<F: FnOnce(&mut StdString) -> T, T>(&mut self, f: F) -> T {
        let mut s = mem::take(self).into_string();

        let ret = f(&mut s);
        *self = s.try_into().unwrap();

        ret
    }

    #[inline]
    pub fn into_string(self) -> StdString {
        unsafe { StdString::from_utf8_unchecked(self.inner) }
    }

    #[inline]
    pub fn into_boxed_str(self) -> Box<str> {
        self.into_string().into_boxed_str()
    }

    #[inline]
    pub fn into_boxed_reference(self) -> Box<Str> {
        let raw = Box::into_raw(self.into_boxed_str()) as *mut Str;

        unsafe { Box::from_raw(raw) }
    }

    #[inline]
    pub fn into_boxed_bytes(self) -> Box<[u8]> {
        self.into_boxed_str().into()
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.inner
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    #[inline]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline]
    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    #[inline]
    pub fn as_reference(&self) -> &Str {
        unsafe { Str::new_unchecked(self.as_str()) }
    }

    #[inline]
    pub fn as_reference_mut(&mut self) -> &mut Str {
        unsafe { Str::new_unchecked_mut(self.as_str_mut()) }
    }

    /// Will panic if the length exceeds [u32::MAX]
    #[inline]
    pub unsafe fn from_vec(vec: Vec<u8>) -> Self {
        String { inner: vec }
    }

    #[inline]
    pub unsafe fn try_from_vec(vec: Vec<u8>) -> Result<Self, Vec<u8>> {
        if vec.len() > super::MAXIMUM {
            Err(vec)
        } else {
            Ok(Self::from_vec(vec))
        }
    }

    /*     /// Will panic if length exceeds [u32::MAX]
    pub fn from_utf8 */

    #[inline]
    /// Will panic if the length exceeds [u32::MAX]
    pub fn from_string(string: StdString) -> Self {
        unsafe { Self::from_vec(string.into_bytes()) }
    }

    #[inline]
    pub fn try_from_string(string: StdString) -> Result<Self, StdString> {
        if string.len() > super::MAXIMUM {
            Err(string)
        } else {
            Ok(Self::from_string(string))
        }
    }

    #[inline]
    /// Will panic if the length exceeds [u32::MAX]
    pub fn from_str(string: &str) -> Self {
        Self::from_string(string.to_owned())
    }

    #[inline]
    pub fn try_from_str(string: &str) -> Result<Self, &str> {
        if string.len() > super::MAXIMUM {
            Err(string)
        } else {
            Ok(Self::from_str(string))
        }
    }

    #[inline]
    pub fn from_reference(string: &Str) -> Self {
        unsafe { Self::from_vec(string.as_bytes().to_owned()) }
    }
}

impl String {
    pub fn push(&mut self, c: char) {
        self.as_string(|st| st.push(c))
    }

    pub fn push_str(&mut self, s: impl AsRef<str>) {
        self.as_string(|st| st.push_str(s.as_ref()))
    }

    pub fn insert(&mut self, index: u32, c: char) {
        self.as_string(|st| st.insert(index as usize, c))
    }
}

impl Deref for String {
    type Target = Str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_reference()
    }
}

impl DerefMut for String {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_reference_mut()
    }
}

impl AsRef<Str> for String {
    fn as_ref(&self) -> &Str {
        self.as_reference()
    }
}

impl AsMut<Str> for String {
    fn as_mut(&mut self) -> &mut Str {
        self.as_reference_mut()
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsMut<str> for String {
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl AsRef<[u8]> for String {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<String> for StdString {
    fn from(s: String) -> Self {
        s.into_string()
    }
}

impl TryFrom<StdString> for String {
    type Error = StdString;

    fn try_from(value: StdString) -> Result<Self, Self::Error> {
        Self::try_from_string(value)
    }
}

impl<'a> TryFrom<&'a str> for String {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from_str(value)
    }
}

impl<'a> From<&'a Str> for String {
    fn from(value: &'a Str) -> Self {
        Self::from_reference(value)
    }
}

impl<'a> From<String> for Cow<'a, Str> {
    fn from(s: String) -> Self {
        Cow::Owned(s)
    }
}

impl<'a> From<String> for Cow<'a, str> {
    fn from(s: String) -> Self {
        Cow::Owned(s.into())
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<S> PartialEq<S> for String
where
    Str: PartialEq<S>,
{
    fn eq(&self, other: &S) -> bool {
        self.as_reference().eq(other)
    }
}

impl Eq for String {}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<S> PartialOrd<S> for String
where
    Str: PartialOrd<S>,
{
    fn partial_cmp(&self, other: &S) -> Option<std::cmp::Ordering> {
        self.as_reference().partial_cmp(other)
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Borrow<str> for String {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl BorrowMut<str> for String {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl Borrow<Str> for String {
    #[inline]
    fn borrow(&self) -> &Str {
        self.as_reference()
    }
}

impl BorrowMut<Str> for String {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Str {
        self.as_reference_mut()
    }
}

impl std::hash::Hash for String {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
