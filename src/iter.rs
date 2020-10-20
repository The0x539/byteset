use crate::ByteSet;
use core::{borrow::Borrow, iter::FusedIterator, ops::RangeInclusive};

macro_rules! wrapped {
    ($(impl $trait:ident $(<Item = $item:ty>)? for $type:ty;)*) => {
        $(wrapped!($trait $(<Item = $item>)? for $type);)*
    };

    (Iterator<Item = $item:ty> for $type:ty) => {
        impl Iterator for $type {
            type Item = $item;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
            // All iterators in this module only give the bytes in order,
            // so min and max are first and last.
            fn min(mut self) -> Option<Self::Item> {
                self.next()
            }
            fn max(mut self) -> Option<Self::Item> {
                self.next_back()
            }
        }
    };

    (DoubleEndedIterator for $type:ty) => {
        impl DoubleEndedIterator for $type {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back()
            }
        }
    };

    (FusedIterator for $type:ty) => {
        impl FusedIterator for $type {}
    };

    (ExactSizeIterator for $type:ty) => {
        impl ExactSizeIterator for $type {
            fn len(&self) -> usize {
                self.0.len()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IterImpl<T> {
    range: RangeInclusive<u8>,
    set: T,
}

#[derive(Debug, Clone)]
pub struct Iter<'a>(pub(crate) IterImpl<&'a ByteSet>);

#[derive(Debug, Clone)]
pub struct IntoIter(pub(crate) IterImpl<ByteSet>);

wrapped! {
    impl Iterator<Item = u8> for Iter<'_>;
    impl Iterator<Item = u8> for IntoIter;
    impl DoubleEndedIterator for Iter<'_>;
    impl DoubleEndedIterator for IntoIter;
    impl FusedIterator for Iter<'_>;
    impl FusedIterator for IntoIter;
}

impl<T: Borrow<ByteSet>> IterImpl<T> {
    pub(crate) fn new(set: T) -> Self {
        Self {
            range: set.borrow().range(),
            set,
        }
    }
}

impl<T: Borrow<ByteSet>> Iterator for IterImpl<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let set = self.set.borrow();
        while let Some(val) = self.range.next() {
            if set.contains(val) {
                return Some(val);
            }
        }
        None
    }
}

impl<T: Borrow<ByteSet>> DoubleEndedIterator for IterImpl<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let set = self.set.borrow();
        while let Some(val) = self.range.next_back() {
            if set.contains(val) {
                return Some(val);
            }
        }
        None
    }
}

impl<T> FusedIterator for IterImpl<T> where Self: Iterator {}

impl<'a> IntoIterator for &'a ByteSet {
    type IntoIter = Iter<'a>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        Iter(IterImpl::new(self))
    }
}

impl IntoIterator for ByteSet {
    type IntoIter = IntoIter;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(IterImpl::new(self))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PairsImpl<T> {
    range: RangeInclusive<u8>,
    set: T,
}

#[derive(Debug, Clone)]
pub struct Pairs<'a>(pub(crate) PairsImpl<&'a ByteSet>);

#[derive(Debug, Clone)]
pub struct IntoPairs(pub(crate) PairsImpl<ByteSet>);

wrapped! {
    impl Iterator<Item = (u8, bool)> for Pairs<'_>;
    impl Iterator<Item = (u8, bool)> for IntoPairs;
    impl DoubleEndedIterator for Pairs<'_>;
    impl DoubleEndedIterator for IntoPairs;
    impl FusedIterator for Pairs<'_>;
    impl FusedIterator for IntoPairs;
    impl ExactSizeIterator for Pairs<'_>;
    impl ExactSizeIterator for IntoPairs;
}

impl<T: Borrow<ByteSet>> PairsImpl<T> {
    pub(crate) fn new(set: T) -> Self {
        Self {
            range: u8::MIN..=u8::MAX,
            set,
        }
    }
}

impl<T: Borrow<ByteSet>> Iterator for PairsImpl<T> {
    type Item = (u8, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.range.next()?;
        Some((val, self.set.borrow().contains(val)))
    }
}

impl<T: Borrow<ByteSet>> DoubleEndedIterator for PairsImpl<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let val = self.range.next_back()?;
        Some((val, self.set.borrow().contains(val)))
    }
}

impl<T> FusedIterator for PairsImpl<T> where Self: Iterator {}

impl<T> ExactSizeIterator for PairsImpl<T>
where
    Self: Iterator,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}
