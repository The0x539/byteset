use crate::ByteSet;
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
};

impl Not for ByteSet {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Not for &ByteSet {
    type Output = ByteSet;
    fn not(self) -> Self::Output {
        ByteSet(!self.0)
    }
}

macro_rules! bin_impl {
    ($trait:ident, $func:ident, $lhs:ty, $rhs:ty) => {
        impl $trait<$rhs> for $lhs {
            type Output = ByteSet;
            fn $func(self, other: $rhs) -> Self::Output {
                ByteSet($trait::$func(self.0, other.0))
            }
        }
    };
}

macro_rules! bin_op {
    (impl $trait:ident for ByteSet: $func:ident) => {
        bin_impl!($trait, $func, ByteSet, ByteSet);
        bin_impl!($trait, $func, ByteSet, &ByteSet);
        bin_impl!($trait, $func, &ByteSet, ByteSet);
        bin_impl!($trait, $func, &ByteSet, &ByteSet);
    };
}

bin_op!(impl BitAnd for ByteSet: bitand);
bin_op!(impl BitOr for ByteSet: bitor);
bin_op!(impl BitXor for ByteSet: bitxor);
bin_op!(impl Sub for ByteSet: sub);

macro_rules! assign_op {
    (impl $trait:ident for $t:ty: $func:ident) => {
        impl $trait for $t {
            fn $func(&mut self, other: Self) {
                $trait::$func(&mut self.0, other.0);
            }
        }
        impl $trait<&Self> for $t {
            fn $func(&mut self, other: &Self) {
                $trait::$func(&mut self.0, other.0);
            }
        }
    };
}

assign_op!(impl BitAndAssign for ByteSet: bitand_assign);
assign_op!(impl BitOrAssign for ByteSet: bitor_assign);
assign_op!(impl BitXorAssign for ByteSet: bitxor_assign);
assign_op!(impl SubAssign for ByteSet: sub_assign);
