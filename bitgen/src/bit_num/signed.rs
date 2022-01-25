use std::{mem, ops};

use num_traits::{AsPrimitive, Num, One, PrimInt, Unsigned, Zero};

use super::*;
use crate::bit_type::BitType;

use super::max_with_bits;

/// An unsigned integer with N bits.
#[derive(Default, Clone, Copy, Debug)]
pub struct I<const N: usize>(pub <Underlying<N> as Type>::I)
where
    Underlying<N>: Type;
impl<const N: usize> I<N>
where
    Underlying<N>: Type,
{
    fn neg_bit() -> <Underlying<N> as Type>::I {
        <Underlying<N> as Type>::I::one() << (N - 1)
    }

    #[cfg(debug_assertions)]
    fn fits(value: <Underlying<N> as Type>::I) -> bool {
        value
            >= -max_with_bits::<<Underlying<N> as Type>::I>(N - 1)
                - <Underlying<N> as Type>::I::one()
            && value <= max_with_bits(N - 1)
    }

    pub fn new(value: <Underlying<N> as Type>::I) -> Self {
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("The value {:?} does not fit in {} bits", value, N);
            }
        }
        if value < <Underlying<N> as Type>::I::zero() {
            I((value & max_with_bits(N)) | Self::neg_bit())
        } else {
            I(value & max_with_bits(N))
        }
    }

    pub fn is_negative(self) -> bool {
        self.0 & Self::neg_bit() != <Underlying<N> as Type>::I::zero()
    }

    pub fn extract_underlying(self) -> <Underlying<N> as Type>::I {
        if self.is_negative() {
            self.0 & max_with_bits(N)
        } else {
            self.0 & max_with_bits(<Underlying<N> as Type>::BITS)
        }
    }
}

impl<const N: usize> ops::Add<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn add(self, rhs: I<N>) -> Self::Output {
        let value = self.0 + rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to add with overflow");
            }
        }
        I(value & max_with_bits(N))
    }
}
impl<const N: usize> ops::Sub<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn sub(self, rhs: I<N>) -> Self::Output {
        let value = self.0 - rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to subtract with overflow");
            }
        }
        I(value & max_with_bits(N))
    }
}
impl<const N: usize> ops::Mul<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn mul(self, rhs: I<N>) -> Self::Output {
        let value = self.0 * rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to multiply with overflow");
            }
        }
        I(value & max_with_bits(N))
    }
}
impl<const N: usize> ops::Div<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn div(self, rhs: I<N>) -> Self::Output {
        let value = self.0 / rhs.0;
        #[cfg(debug_assertions)]
        {
            // Can happen if self.0 is smalest negative and rhs.0 is -1
            if !Self::fits(value) {
                panic!("Attempted to divide with overflow");
            }
        }
        I(value & max_with_bits(N))
    }
}
impl<const N: usize> ops::Rem<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn rem(self, rhs: I<N>) -> Self::Output {
        // Should never overflow
        I(self.0 % rhs.0)
    }
}
impl<const N: usize> ops::BitAnd<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn bitand(self, rhs: I<N>) -> Self::Output {
        // Should never overflow
        I(self.0 & rhs.0)
    }
}
impl<const N: usize> ops::BitOr<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn bitor(self, rhs: I<N>) -> Self::Output {
        // Should never overflow
        I(self.0 | rhs.0)
    }
}
impl<const N: usize> ops::BitXor<I<N>> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn bitxor(self, rhs: I<N>) -> Self::Output {
        // Should never overflow
        I(self.0 ^ rhs.0)
    }
}
impl<const N: usize> ops::Shl<usize> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        {
            if rhs >= N {
                panic!("Attempted to shift left with overflow");
            }
        }
        I((self.0 << rhs) & max_with_bits(N))
    }
}
impl<const N: usize> ops::Shr<usize> for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        #[cfg(debug_assertions)]
        {
            if rhs >= N {
                panic!("Attempted to shift right with overflow");
            }
        }
        I(self.0 >> rhs)
    }
}

impl<const N: usize> ops::AddAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::AddAssign<<Underlying<N> as Type>::I>,
{
    fn add_assign(&mut self, rhs: I<N>) {
        let value = self.0 + rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to add with overflow");
            }
        }
        self.0 = value & max_with_bits(N);
    }
}
impl<const N: usize> ops::SubAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::SubAssign<<Underlying<N> as Type>::I>,
{
    fn sub_assign(&mut self, rhs: I<N>) {
        let value = self.0 - rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to subtract with overflow");
            }
        }
        self.0 = value & max_with_bits(N);
    }
}
impl<const N: usize> ops::MulAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::MulAssign<<Underlying<N> as Type>::I>,
{
    fn mul_assign(&mut self, rhs: I<N>) {
        let value = self.0 * rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to multiply with overflow");
            }
        }
        self.0 = value & max_with_bits(N);
    }
}
impl<const N: usize> ops::DivAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::DivAssign<<Underlying<N> as Type>::I>,
{
    fn div_assign(&mut self, rhs: I<N>) {
        let value = self.0 / rhs.0;
        #[cfg(debug_assertions)]
        {
            if !Self::fits(value) {
                panic!("Attempted to divide with overflow");
            }
        }
        self.0 = value & max_with_bits(N);
    }
}
impl<const N: usize> ops::RemAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::RemAssign<<Underlying<N> as Type>::I>,
{
    fn rem_assign(&mut self, rhs: I<N>) {
        // Should never overflow
        self.0 %= rhs.0;
    }
}
impl<const N: usize> ops::BitAndAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::BitAndAssign<<Underlying<N> as Type>::I>,
{
    fn bitand_assign(&mut self, rhs: I<N>) {
        // Should never overflow
        self.0 &= rhs.0;
    }
}
impl<const N: usize> ops::BitOrAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::BitOrAssign<<Underlying<N> as Type>::I>,
{
    fn bitor_assign(&mut self, rhs: I<N>) {
        // Should never overflow
        self.0 |= rhs.0;
    }
}
impl<const N: usize> ops::BitXorAssign<I<N>> for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: ops::BitXorAssign<<Underlying<N> as Type>::I>,
{
    fn bitxor_assign(&mut self, rhs: I<N>) {
        // Should never overflow
        self.0 ^= rhs.0;
    }
}
impl<const N: usize> ops::ShlAssign<usize> for I<N>
where
    Underlying<N>: Type,
{
    fn shl_assign(&mut self, rhs: usize) {
        #[cfg(debug_assertions)]
        {
            if rhs >= N {
                panic!("Attempted to shift left with overflow");
            }
        }
        self.0 = (self.0 << rhs) & max_with_bits(N);
    }
}
impl<const N: usize> ops::ShrAssign<usize> for I<N>
where
    Underlying<N>: Type,
{
    fn shr_assign(&mut self, rhs: usize) {
        #[cfg(debug_assertions)]
        {
            if rhs >= N {
                panic!("Attempted to shift right with overflow");
            }
        }
        self.0 = (self.0 << rhs) & max_with_bits(N);
    }
}

impl<const N: usize> ops::Not for I<N>
where
    Underlying<N>: Type,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        I(!self.0 & max_with_bits(N))
    }
}

impl<const N: usize> Zero for I<N>
where
    Underlying<N>: Type,
{
    fn zero() -> Self {
        I(<Underlying<N> as Type>::I::zero())
    }

    fn is_zero(&self) -> bool {
        (self.0 & max_with_bits(N)).is_zero()
    }
}
impl<const N: usize> One for I<N>
where
    Underlying<N>: Type,
{
    fn one() -> Self {
        I(<Underlying<N> as Type>::I::one())
    }
}

impl<const N: usize> PartialEq for I<N>
where
    Underlying<N>: Type,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<const N: usize> Eq for I<N> where Underlying<N>: Type {}

impl<const N: usize> PartialOrd for I<N>
where
    Underlying<N>: Type,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.extract_underlying()
            .partial_cmp(&other.extract_underlying())
    }
}

impl<const N: usize> Ord for I<N>
where
    Underlying<N>: Type,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.extract_underlying().cmp(&other.extract_underlying())
    }
}

impl<const N: usize> Num for I<N>
where
    Underlying<N>: Type,
{
    type FromStrRadixErr = <<Underlying<N> as Type>::I as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <Underlying<N> as Type>::I::from_str_radix(str, radix).map(I::new)
    }
}
impl<const N: usize> Unsigned for I<N> where Underlying<N>: Type {}

impl<const N: usize> fmt::Display for I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::I: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.extract_underlying())
    }
}

impl<const A: usize, const B: usize> AsPrimitive<I<A>> for I<B>
where
    Underlying<A>: Type,
    Underlying<B>: Type,
    <Underlying<B> as Type>::I: AsPrimitive<<Underlying<A> as Type>::I>,
{
    fn as_(self) -> I<A> {
        I::new(self.extract_underlying().as_())
    }
}

impl<T: PrimInt + 'static, const B: usize> AsPrimitive<T> for I<B>
where
    Underlying<B>: Type,
    <Underlying<B> as Type>::I: AsPrimitive<T>,
{
    fn as_(self) -> T {
        self.extract_underlying().as_()
    }
}

macro_rules! impl_bit_type {
    ($n:literal) => {
        impl BitType for I<$n> {
            const BITS: usize = $n;

            fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
                if slice.len() == (Self::BITS + 7) / 8 {
                    let mut num: <Underlying<{ Self::BITS }> as Type>::I =
                        unsafe { mem::zeroed() };
                    let mut bits = !num;
                    let num_slice = unsafe {
                        mem::transmute::<
                            &mut <Underlying<{ Self::BITS }> as Type>::I,
                            &mut [u8; mem::size_of::<Self>()],
                        >(&mut num)
                    };
                    num_slice[0..mem::size_of::<Self>()].copy_from_slice(unsafe {
                        mem::transmute::<&Self, &[u8; mem::size_of::<Self>()]>(aligned)
                    });
                    num <<= mem::size_of::<Self>() * 8 - Self::BITS;
                    bits <<= mem::size_of::<Self>() * 8 - Self::BITS;

                    num >>= mem::size_of::<Self>() * 8 - Self::BITS - offset;
                    bits >>= mem::size_of::<Self>() * 8 - Self::BITS - offset;

                    let target_num = unsafe {
                        mem::transmute::<
                            &mut [u8],
                            (&mut <Underlying<{ Self::BITS }> as Type>::I, usize),
                        >(slice)
                        .0
                    };

                    *target_num &= !bits;
                    *target_num |= num;
                } else {
                    let mut num: <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I =
                        unsafe { mem::zeroed() };
                    let mut bits = !num;
                    let num_slice = unsafe {
                        mem::transmute::<
                            &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I,
                            &mut [u8; mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>()],
                        >(&mut num)
                    };

                    num_slice[0..mem::size_of::<Self>()].copy_from_slice(unsafe {
                        mem::transmute::<&Self, &[u8; mem::size_of::<Self>()]>(aligned)
                    });

                    num <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - Self::BITS;
                    bits <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - Self::BITS;

                    num >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - Self::BITS - offset;
                    bits >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - Self::BITS - offset;

                    let target_num = unsafe {
                        mem::transmute::<
                            &mut [u8],
                            (
                                &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I,
                                usize,
                            ),
                        >(slice)
                        .0
                    };
                    *target_num &= !bits;
                    *target_num |= num;
                }
            }

            fn to_aligned(slice: &[u8], offset: usize) -> Self {
                if slice.len() == (Self::BITS + 7) / 8 {
                    let mut num: <Underlying<{ Self::BITS }> as Type>::I =
                        unsafe { mem::zeroed() };
                    let num_slice = unsafe {
                        mem::transmute::<
                            &mut <Underlying<{ Self::BITS }> as Type>::I,
                            &mut [u8; mem::size_of::<<Underlying<{ Self::BITS }> as Type>::I>(
                            )],
                        >(&mut num)
                    };
                    num_slice[0..slice.len()].copy_from_slice(slice);
                    num <<= mem::size_of::<Self>() * 8 - <Self>::BITS - offset;
                    num >>= mem::size_of::<Self>() * 8 - <Self>::BITS;
                    unsafe { mem::transmute_copy(&num) }
                } else {
                    let mut num: <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I =
                        unsafe { mem::zeroed() };
                    let num_slice = unsafe {
                        mem::transmute::<
                            &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I,
                            &mut [u8; mem::size_of::<
                                <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I,
                            >()],
                        >(&mut num)
                    };
                    num_slice[0..slice.len()].copy_from_slice(slice);
                    num <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - <Self>::BITS - offset;
                    num >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::I>() * 8 - <Self>::BITS;
                    unsafe { mem::transmute_copy(&num) }
                }
            }
        }
    };

    ($($n: literal), +$(,)?) => {
        $(impl_bit_type!{$n})+
    };
}
impl_bit_type! {
    2, 3, 4, 5, 6, 7,
    9, 10, 11, 12, 13, 14, 15,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
}

impl BitType for I<1> {
    const BITS: usize = 1;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        bool::from_aligned(&(aligned.0 & 1 == 1), slice, offset)
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        I(bool::to_aligned(slice, offset) as i8)
    }
}
impl BitType for I<8> {
    const BITS: usize = 8;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        i8::from_aligned(&aligned.0, slice, offset)
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        I(i8::to_aligned(slice, offset))
    }
}
impl BitType for I<16> {
    const BITS: usize = 16;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        i16::from_aligned(&aligned.0, slice, offset)
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        I(i16::to_aligned(slice, offset))
    }
}
impl BitType for I<32> {
    const BITS: usize = 32;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        i32::from_aligned(&aligned.0, slice, offset)
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        I(i32::to_aligned(slice, offset))
    }
}
impl BitType for I<64> {
    const BITS: usize = 64;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        i64::from_aligned(&aligned.0, slice, offset)
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        I(i64::to_aligned(slice, offset))
    }
}
