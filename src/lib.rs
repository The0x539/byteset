#![no_std]

use core::{
    borrow::Borrow,
    iter::{Extend, FromIterator},
    ops::{Index, RangeInclusive},
};
use ethnum::u256;

pub mod iter;
mod ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ByteSet(u256);

impl ByteSet {
    fn mask<T: Borrow<u8>>(val: T) -> u256 {
        u256::ONE << val.borrow()
    }

    fn range(&self) -> RangeInclusive<u8> {
        if self.is_empty() {
            // whatever, just an empty range
            return 1..=0;
        }
        // max possible value for these functions is 256, which we just checked for
        let leading = self.0.leading_zeros() as u8;
        let trailing = self.0.trailing_zeros() as u8;
        RangeInclusive::new(trailing, 255 - leading)
    }

    pub const fn new() -> Self {
        Self(u256::ZERO)
    }

    pub fn iter(&self) -> iter::Iter<'_> {
        iter::Iter(iter::IterImpl::new(self))
    }

    pub fn pairs(&self) -> iter::Pairs<'_> {
        iter::Pairs(iter::PairsImpl::new(self))
    }

    pub fn into_pairs(self) -> iter::IntoPairs {
        iter::IntoPairs(iter::PairsImpl::new(self))
    }

    pub const fn len(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn clear(&mut self) {
        self.0 = u256::ZERO;
    }

    pub fn difference(&self, other: &Self) -> Self {
        self - other
    }

    pub fn symmetric_difference(&self, other: &Self) -> Self {
        self ^ other
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self & other
    }

    pub fn union(&self, other: &Self) -> Self {
        self | other
    }

    pub fn contains<T: Borrow<u8>>(&self, val: T) -> bool {
        (self.0 & Self::mask(val)) != 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        (self.0 & other.0) == 0
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        (self.0 & other.0) == self.0
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn insert(&mut self, val: u8) -> bool {
        let prev = self.0;
        self.0 |= Self::mask(val);
        prev != self.0
    }

    pub fn remove<T: Borrow<u8>>(&mut self, val: T) -> bool {
        let prev = self.0;
        self.0 &= !Self::mask(val);
        prev != self.0
    }

    pub fn toggle<T: Borrow<u8>>(&mut self, val: T) -> bool {
        let mask = Self::mask(val);
        self.0 ^= mask;
        (mask & self.0) == 0
    }

    pub fn take<T: Borrow<u8>>(&mut self, val: T) -> Option<T> {
        if self.remove(val.borrow()) {
            Some(val)
        } else {
            None
        }
    }

    pub fn retain<F: FnMut(u8) -> bool>(&mut self, mut f: F) {
        for val in self.clone() {
            if !f(val) {
                self.remove(val);
            }
        }
    }
}

impl Default for ByteSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Extend<u8> for ByteSet {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        for val in iter {
            self.insert(val);
        }
    }
}

impl FromIterator<u8> for ByteSet {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<T: Borrow<u8>> Index<T> for ByteSet {
    type Output = bool;
    fn index(&self, index: T) -> &Self::Output {
        if self.contains(index) {
            &true
        } else {
            &false
        }
    }
}
