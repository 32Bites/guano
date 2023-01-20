mod str;
mod string;

use guano_common::num::traits::ToPrimitive;
use guano_common::rowan::TextRange;
use std::ops::Range;
use std::ops::RangeBounds;
use std::ops::RangeFrom;
use std::ops::RangeFull;
use std::ops::RangeInclusive;
use std::ops::RangeTo;
use std::ops::RangeToInclusive;
use std::slice::SliceIndex;

pub use self::str::*;
pub use self::string::*;

pub const MAXIMUM: usize = u32::MAX as usize;

pub trait StrRangeBounds {
    type BoundedRange: RangeBounds<u32>;
    type UnboundedRange: RangeBounds<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange>;

    #[inline]
    fn to_bounded(&self) -> Self::BoundedRange {
        self.try_to_bounded().unwrap()
    }

    #[inline]
    fn into_bounded(self) -> Self::BoundedRange
    where
        Self: Sized,
    {
        self.to_bounded()
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange>;

    #[inline]
    fn to_unbounded(&self) -> Self::UnboundedRange {
        self.try_to_unbounded().unwrap()
    }

    #[inline]
    fn into_unbounded(self) -> Self::UnboundedRange
    where
        Self: Sized,
    {
        self.to_unbounded()
    }
}

impl<I: ToPrimitive> StrRangeBounds for Range<I> {
    type BoundedRange = Range<u32>;
    type UnboundedRange = Range<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let start = self.start.to_u32()?;
        let end = self.end.to_u32()?;

        Some(start..end)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let bounded = self.try_to_bounded()?;
        let start = bounded.start.to_usize()?;
        let end = bounded.end.to_usize()?;

        Some(start..end)
    }
}

impl<I: ToPrimitive> StrRangeBounds for RangeFrom<I> {
    type BoundedRange = RangeFrom<u32>;
    type UnboundedRange = RangeFrom<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let start = self.start.to_u32()?;
        Some(start..)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let start = self.try_to_bounded()?.start.to_usize()?;

        Some(start..)
    }
}

impl<I: ToPrimitive> StrRangeBounds for RangeTo<I> {
    type BoundedRange = RangeTo<u32>;
    type UnboundedRange = RangeTo<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let end = self.end.to_u32()?;
        Some(..end)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let end = self.try_to_bounded()?.end.to_usize()?;

        Some(..end)
    }
}

impl<I: ToPrimitive> StrRangeBounds for RangeToInclusive<I> {
    type BoundedRange = RangeToInclusive<u32>;
    type UnboundedRange = RangeToInclusive<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let end = self.end.to_u32()?;
        Some(..=end)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let end = self.try_to_bounded()?.end.to_usize()?;

        Some(..=end)
    }
}

impl<I: ToPrimitive> StrRangeBounds for RangeInclusive<I> {
    type BoundedRange = RangeInclusive<u32>;
    type UnboundedRange = RangeInclusive<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let start = self.start().to_u32()?;
        let end = self.end().to_u32()?;
        Some(start..=end)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let bounded = self.try_to_bounded()?;
        let start = bounded.start().to_usize()?;
        let end = bounded.end().to_usize()?;

        Some(start..=end)
    }
}

impl StrRangeBounds for RangeFull {
    type BoundedRange = RangeFull;
    type UnboundedRange = RangeFull;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        Some(..)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        Some(..)
    }
}

impl StrRangeBounds for TextRange {
    type BoundedRange = Range<u32>;
    type UnboundedRange = Range<usize>;

    fn try_to_bounded(&self) -> Option<Self::BoundedRange> {
        let start = self.start().into();
        let end = self.end().into();

        Some(start..end)
    }

    fn try_to_unbounded(&self) -> Option<Self::UnboundedRange> {
        let bounded = self.try_to_bounded()?;
        let start = bounded.start.to_usize()?;
        let end = bounded.end.to_usize()?;

        Some(start..end)
    }
}

pub unsafe trait StrIndex: Sized {
    fn get(self, string: &Str) -> Option<&Str>;
    fn get_mut(self, string: &mut Str) -> Option<&mut Str>;

    unsafe fn get_unchecked(self, string: *const Str) -> *const Str;
    unsafe fn get_unchecked_mut(self, string: *mut Str) -> *mut Str;

    #[inline]
    fn index(self, string: &Str) -> &Str {
        self.get(string).unwrap()
    }

    #[inline]
    fn index_mut(self, string: &mut Str) -> &mut Str {
        self.get_mut(string).unwrap()
    }
}

unsafe impl<R: StrRangeBounds + Sized> StrIndex for R
where
    R::UnboundedRange: SliceIndex<str, Output = str>,
{
    fn get(self, string: &Str) -> Option<&Str> {
        let range = self.try_to_unbounded()?;
        let string = string.as_str().get(range)?;
        let string = unsafe { Str::new_unchecked(string) };

        Some(string)
    }

    fn get_mut(self, string: &mut Str) -> Option<&mut Str> {
        let range = self.try_to_unbounded()?;
        let string = string.as_str_mut().get_mut(range)?;
        let string = unsafe { Str::new_unchecked_mut(string) };

        Some(string)
    }

    unsafe fn get_unchecked(self, string: *const Str) -> *const Str {
        let range = self.into_unbounded();
        let string = string.as_ref().unwrap().as_str();
        let string = string.get_unchecked(range) as *const str;

        string as *const Str
    }

    unsafe fn get_unchecked_mut(self, string: *mut Str) -> *mut Str {
        let range = self.into_unbounded();
        let string = string.as_mut().unwrap().as_str_mut();
        let string = string.get_unchecked_mut(range) as *mut str;

        string as *mut Str
    }
}
