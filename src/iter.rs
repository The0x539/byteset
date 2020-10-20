use crate::ByteSet;
use core::{borrow::Borrow, ops::RangeInclusive};

macro_rules! wrapped {
    (impl Iterator<Item = $item:ty> for $type:ty) => {
        impl Iterator for $type {
            type Item = $item;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub(crate) struct IterImpl<T> {
    range: RangeInclusive<u8>,
    set: T,
}

#[derive(Debug, Clone)]
pub struct Iter<'a>(pub(crate) IterImpl<&'a ByteSet>);
wrapped!(impl Iterator<Item = u8> for Iter<'_>);

#[derive(Debug, Clone)]
pub struct IntoIter(pub(crate) IterImpl<ByteSet>);
wrapped!(impl Iterator<Item = u8> for IntoIter);

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
        for val in &mut self.range {
            if set.contains(val) {
                return Some(val);
            }
        }
        None
    }
}

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
wrapped!(impl Iterator<Item = (u8, bool)> for Pairs<'_>);

#[derive(Debug, Clone)]
pub struct IntoPairs(pub(crate) PairsImpl<ByteSet>);
wrapped!(impl Iterator<Item = (u8, bool)> for IntoPairs);

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
