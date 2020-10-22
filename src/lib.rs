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

    /// Creates an empty `ByteSet`.
    pub const fn new() -> Self {
        Self(u256::ZERO)
    }

    /// An iterator visiting all elements in increasing order.
    pub fn iter(&self) -> iter::Iter<'_> {
        iter::Iter(iter::IterImpl::new(self))
    }

    /// An iterator visiting each possible element in increasing order,
    /// accompanied by whether the set contains it.
    /// The iterator element type is `(u8, bool)`.
    ///
    /// Equivalent to `(0..=u8::MAX).map(|x| (x, self.contains(x)))`.
    pub fn pairs(&self) -> iter::Pairs<'_> {
        iter::Pairs(iter::PairsImpl::new(self))
    }

    /// Like [`ByteSet::pairs`], but owns the underlying set.
    /// `ByteSet` implements the `Copy` trait and doesn't offer `Item = &u8` iterators,
    /// so the importance of this distinction is somewhat unclear.
    ///
    /// Equivalent to `(0..=u8::MAX).map(move |x| (x, self.contains(x)))`.
    pub fn into_pairs(self) -> iter::IntoPairs {
        iter::IntoPairs(iter::PairsImpl::new(self))
    }

    /// Returns the number of elements in the set.
    pub const fn len(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.0 = u256::ZERO;
    }

    /// Returns a new `ByteSet` representing the difference,
    /// i.e. the values that are in `self` but not in `other`.
    pub fn difference(&self, other: &Self) -> Self {
        self - other
    }

    /// Returns a new `ByteSet` representing the symmetric difference,
    /// i.e. the values that are in `self` or in `other` but not in both.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        self ^ other
    }

    /// Returns a new `ByteSet` representing the intersection,
    /// i.e. the values that are both in `self` and `other.`
    pub fn intersection(&self, other: &Self) -> Self {
        self & other
    }

    /// Returns a new `ByteSet` representing the union,
    /// i.e. all the values in `self` or `other`.
    pub fn union(&self, other: &Self) -> Self {
        self | other
    }

    /// Returns `true` if the set contains a value.
    /// The value may be passed as a `u8` or as any borrowed form of `u8`.
    pub fn contains<T: Borrow<u8>>(&self, val: T) -> bool {
        (self.0 & Self::mask(val)) != 0
    }

    /// Returns `true` if `self` has no elements in common with `other`.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        (self.0 & other.0) == 0
    }

    /// Returns `true` if `self` is a subset of `other`,
    /// i.e. every value in `self` is also in `other`.
    pub fn is_subset(&self, other: &Self) -> bool {
        (self.0 & other.0) == self.0
    }

    /// Returns `true` if `self` is a superset of `other`,
    /// i.e. every value in `other` is also in `self`.
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Adds a value to the set.
    /// Returns whether the value was already present in the set.
    /// The value may be passed as a `u8` or as any borrowed form of `u8`.
    pub fn insert<T: Borrow<u8>>(&mut self, val: T) -> bool {
        let prev = self.0;
        self.0 |= Self::mask(val);
        prev != self.0
    }

    /// Removes a value from the set.
    /// Returns whether the value was present in the set.
    /// The value may be passed as a `u8` or as any borrowed form of `u8`.
    pub fn remove<T: Borrow<u8>>(&mut self, val: T) -> bool {
        let prev = self.0;
        self.0 &= !Self::mask(val);
        prev != self.0
    }

    /// Toggles the presence of a value in the set.
    /// If the value is not in the set, add it and return `true`.
    /// If the value is in the set, remove it and return `false`.
    /// The value may be passed as a `u8` or as any borrowed form of `u8`.
    pub fn toggle<T: Borrow<u8>>(&mut self, val: T) -> bool {
        let mask = Self::mask(val);
        self.0 ^= mask;
        (mask & self.0) == 0
    }

    /// Removes a value from the set and returns it if present.
    /// The value may be passed as a `u8` or as any borrowed form of `u8`.
    pub fn take<T: Borrow<u8>>(&mut self, val: T) -> Option<T> {
        if self.remove(val.borrow()) {
            Some(val)
        } else {
            None
        }
    }

    /// Retains only the elements specified by the predicate.
    /// In other words, remove all elements `x` such that `f(x)` returns false.
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
