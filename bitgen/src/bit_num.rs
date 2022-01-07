use std::fmt;

use num_traits::AsPrimitive;

use crate::magic::bits_to_bytes;

pub use self::unsigned::U;

fn max_with_bits<T: num_traits::PrimInt>(num_bits: usize) -> T {
    !((!T::zero()) << num_bits)
}
const fn log2(n: usize) -> usize {
    std::mem::size_of::<usize>() * 8 - n.leading_zeros() as usize
}
pub const fn closest_pow_2(n: usize) -> usize {
    let n = if n == 0 { 1 } else { n };
    1 << (log2(n - 1))
}

pub trait Type {
    type Higher: Type;
    type U: num_traits::Unsigned + num_traits::PrimInt + fmt::Debug + Default + AsPrimitive<u32>;

    type I: num_traits::Signed + num_traits::PrimInt + fmt::Debug + Default;

    const BITS: usize;
}

pub mod bts {
    use super::Type;

    pub struct Bytes<const N: usize>;
    impl Type for Bytes<1> {
        type Higher = Bytes<2>;
        type U = u8;
        type I = i8;
        const BITS: usize = 8;
    }
    impl Type for Bytes<2> {
        type Higher = Bytes<4>;
        type U = u16;
        type I = i16;
        const BITS: usize = 16;
    }
    impl Type for Bytes<4> {
        type Higher = Bytes<8>;
        type U = u32;
        type I = i32;
        const BITS: usize = 32;
    }
    impl Type for Bytes<8> {
        type Higher = Bytes<16>;
        type U = u64;
        type I = i64;
        const BITS: usize = 64;
    }
    impl Type for Bytes<16> {
        type Higher = Bytes<16>;
        type U = u128;
        type I = i128;
        const BITS: usize = 128;
    }
    //  impl<const N: usize> Type for Bytes<N>
    //  where
    //      If<{ N > 16 }>: True,
    //  {
    //      // TODO: replace with custom type
    //      type U = u128;
    //      type I = u128;
    //      const BITS: usize = 128;
    //  }
}

pub struct Underlying<const N: usize>;

impl<const N: usize> Type for Underlying<N>
where
    bts::Bytes<{ closest_pow_2(bits_to_bytes(N)) }>: Type,
{
    type U = <bts::Bytes<{ closest_pow_2(bits_to_bytes(N)) }> as Type>::U;
    type I = <bts::Bytes<{ closest_pow_2(bits_to_bytes(N)) }> as Type>::I;
    const BITS: usize = <bts::Bytes<{ closest_pow_2(bits_to_bytes(N)) }> as Type>::BITS;
    type Higher = <bts::Bytes<{ closest_pow_2(bits_to_bytes(N)) }> as Type>::Higher;
}

mod unsigned {
    use std::{mem, ops};

    use num_traits::{AsPrimitive, Num, One, PrimInt, Unsigned, Zero};

    use super::*;
    use crate::bit_type::BitType;

    use super::max_with_bits;

    #[derive(Default, Clone, Copy, Debug)]
    pub struct U<const N: usize>(pub <Underlying<N> as Type>::U)
    where
        Underlying<N>: Type;
    impl<const N: usize> U<N>
    where
        Underlying<N>: Type,
    {
        pub fn new(value: <Underlying<N> as Type>::U) -> Self {
            U(value)
        }
    }

    impl<const N: usize> ops::Add<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn add(self, rhs: U<N>) -> Self::Output {
            U(self.0 + rhs.0)
        }
    }
    impl<const N: usize> ops::Sub<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn sub(self, rhs: U<N>) -> Self::Output {
            U(self.0 - rhs.0)
        }
    }
    impl<const N: usize> ops::Mul<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn mul(self, rhs: U<N>) -> Self::Output {
            U(self.0 * rhs.0)
        }
    }
    impl<const N: usize> ops::Div<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn div(self, rhs: U<N>) -> Self::Output {
            U(self.0 / rhs.0)
        }
    }
    impl<const N: usize> ops::Rem<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn rem(self, rhs: U<N>) -> Self::Output {
            U(self.0 % rhs.0)
        }
    }
    impl<const N: usize> ops::BitAnd<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn bitand(self, rhs: U<N>) -> Self::Output {
            U(self.0 & rhs.0)
        }
    }
    impl<const N: usize> ops::BitOr<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn bitor(self, rhs: U<N>) -> Self::Output {
            U(self.0 | rhs.0)
        }
    }
    impl<const N: usize> ops::BitXor<U<N>> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn bitxor(self, rhs: U<N>) -> Self::Output {
            U(self.0 ^ rhs.0)
        }
    }
    impl<const N: usize> ops::Shl<usize> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn shl(self, rhs: usize) -> Self::Output {
            U(self.0 << rhs)
        }
    }
    impl<const N: usize> ops::Shr<usize> for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn shr(self, rhs: usize) -> Self::Output {
            U(self.0 >> rhs)
        }
    }

    impl<const N: usize> ops::AddAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::AddAssign<<Underlying<N> as Type>::U>,
    {
        fn add_assign(&mut self, rhs: U<N>) {
            self.0 += rhs.0;
        }
    }
    impl<const N: usize> ops::SubAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::SubAssign<<Underlying<N> as Type>::U>,
    {
        fn sub_assign(&mut self, rhs: U<N>) {
            self.0 -= rhs.0;
        }
    }
    impl<const N: usize> ops::MulAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::MulAssign<<Underlying<N> as Type>::U>,
    {
        fn mul_assign(&mut self, rhs: U<N>) {
            self.0 *= rhs.0;
        }
    }
    impl<const N: usize> ops::DivAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::DivAssign<<Underlying<N> as Type>::U>,
    {
        fn div_assign(&mut self, rhs: U<N>) {
            self.0 /= rhs.0;
        }
    }
    impl<const N: usize> ops::RemAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::RemAssign<<Underlying<N> as Type>::U>,
    {
        fn rem_assign(&mut self, rhs: U<N>) {
            self.0 %= rhs.0;
        }
    }
    impl<const N: usize> ops::BitAndAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::BitAndAssign<<Underlying<N> as Type>::U>,
    {
        fn bitand_assign(&mut self, rhs: U<N>) {
            self.0 &= rhs.0;
        }
    }
    impl<const N: usize> ops::BitOrAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::BitOrAssign<<Underlying<N> as Type>::U>,
    {
        fn bitor_assign(&mut self, rhs: U<N>) {
            self.0 |= rhs.0;
        }
    }
    impl<const N: usize> ops::BitXorAssign<U<N>> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::BitXorAssign<<Underlying<N> as Type>::U>,
    {
        fn bitxor_assign(&mut self, rhs: U<N>) {
            self.0 ^= rhs.0;
        }
    }
    impl<const N: usize> ops::ShlAssign<usize> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::ShlAssign<usize>,
    {
        fn shl_assign(&mut self, rhs: usize) {
            self.0 <<= rhs;
        }
    }
    impl<const N: usize> ops::ShrAssign<usize> for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: ops::ShrAssign<usize>,
    {
        fn shr_assign(&mut self, rhs: usize) {
            self.0 >>= rhs;
        }
    }

    impl<const N: usize> ops::Not for U<N>
    where
        Underlying<N>: Type,
    {
        type Output = Self;

        fn not(self) -> Self::Output {
            U(!self.0)
        }
    }

    impl<const N: usize> Zero for U<N>
    where
        Underlying<N>: Type,
    {
        fn zero() -> Self {
            U(<Underlying<N> as Type>::U::zero())
        }

        fn is_zero(&self) -> bool {
            (self.0 & max_with_bits(N)).is_zero()
        }
    }
    impl<const N: usize> One for U<N>
    where
        Underlying<N>: Type,
    {
        fn one() -> Self {
            U(<Underlying<N> as Type>::U::one())
        }
    }

    impl<const N: usize> PartialEq for U<N>
    where
        Underlying<N>: Type,
    {
        fn eq(&self, other: &Self) -> bool {
            self.0 & max_with_bits(N) == other.0 & max_with_bits(N)
        }
    }

    impl<const N: usize> Eq for U<N> where Underlying<N>: Type {}

    impl<const N: usize> Num for U<N>
    where
        Underlying<N>: Type,
    {
        type FromStrRadixErr = <<Underlying<N> as Type>::U as Num>::FromStrRadixErr;

        fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            <Underlying<N> as Type>::U::from_str_radix(str, radix).map(U)
        }
    }
    impl<const N: usize> Unsigned for U<N> where Underlying<N>: Type {}

    impl<const N: usize> fmt::Display for U<N>
    where
        Underlying<N>: Type,
        <Underlying<N> as Type>::U: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0 & max_with_bits(N))
        }
    }

    impl<const A: usize, const B: usize> AsPrimitive<U<A>> for U<B>
    where
        Underlying<A>: Type,
        Underlying<B>: Type,
        <Underlying<B> as Type>::U: AsPrimitive<<Underlying<A> as Type>::U>,
    {
        fn as_(self) -> U<A> {
            U::new((self.0 & max_with_bits(B)).as_())
        }
    }

    impl<T: PrimInt + 'static, const B: usize> AsPrimitive<T> for U<B>
    where
        Underlying<B>: Type,
        <Underlying<B> as Type>::U: AsPrimitive<T>,
    {
        fn as_(self) -> T {
            (self.0 & max_with_bits(B)).as_()
        }
    }

    macro_rules! impl_bit_type {
        ($n:literal) => {
            impl BitType for U<$n> {
                const BITS: usize = $n;

                fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
                    if slice.len() == (Self::BITS + 7) / 8 {
                        let mut num: <Underlying<{ Self::BITS }> as Type>::U =
                            unsafe { mem::zeroed() };
                        let mut bits = !num;
                        let num_slice = unsafe {
                            mem::transmute::<
                                &mut <Underlying<{ Self::BITS }> as Type>::U,
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
                                (&mut <Underlying<{ Self::BITS }> as Type>::U, usize),
                            >(slice)
                            .0
                        };

                        *target_num &= !bits;
                        *target_num |= num;
                    } else {
                        let mut num: <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U =
                            unsafe { mem::zeroed() };
                        let mut bits = !num;
                        let num_slice = unsafe {
                            mem::transmute::<
                                &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U,
                                &mut [u8; mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>()],
                            >(&mut num)
                        };

                        num_slice[0..mem::size_of::<Self>()].copy_from_slice(unsafe {
                            mem::transmute::<&Self, &[u8; mem::size_of::<Self>()]>(aligned)
                        });

                        num <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - Self::BITS;
                        bits <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - Self::BITS;

                        num >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - Self::BITS - offset;
                        bits >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - Self::BITS - offset;

                        let target_num = unsafe {
                            mem::transmute::<
                                &mut [u8],
                                (
                                    &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U,
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
                        let mut num: <Underlying<{ Self::BITS }> as Type>::U =
                            unsafe { mem::zeroed() };
                        let num_slice = unsafe {
                            mem::transmute::<
                                &mut <Underlying<{ Self::BITS }> as Type>::U,
                                &mut [u8; mem::size_of::<<Underlying<{ Self::BITS }> as Type>::U>(
                                )],
                            >(&mut num)
                        };
                        num_slice[0..slice.len()].copy_from_slice(slice);
                        num <<= mem::size_of::<Self>() * 8 - <Self>::BITS - offset;
                        num >>= mem::size_of::<Self>() * 8 - <Self>::BITS;
                        unsafe { mem::transmute_copy(&num) }
                    } else {
                        let mut num: <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U =
                            unsafe { mem::zeroed() };
                        let num_slice = unsafe {
                            mem::transmute::<
                                &mut <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U,
                                &mut [u8; mem::size_of::<
                                    <<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U,
                                >()],
                            >(&mut num)
                        };
                        num_slice[0..slice.len()].copy_from_slice(slice);
                        num <<= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - <Self>::BITS - offset;
                        num >>= mem::size_of::<<<Underlying<{ Self::BITS }> as Type>::Higher as Type>::U>() * 8 - <Self>::BITS;
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

    impl BitType for U<1> {
        const BITS: usize = 1;

        fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
            bool::from_aligned(&(aligned.0 & 1 == 1), slice, offset)
        }

        fn to_aligned(slice: &[u8], offset: usize) -> Self {
            U(bool::to_aligned(slice, offset) as u8)
        }
    }
    impl BitType for U<8> {
        const BITS: usize = 8;

        fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
            u8::from_aligned(&aligned.0, slice, offset)
        }

        fn to_aligned(slice: &[u8], offset: usize) -> Self {
            U(u8::to_aligned(slice, offset))
        }
    }
    impl BitType for U<16> {
        const BITS: usize = 16;

        fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
            u16::from_aligned(&aligned.0, slice, offset)
        }

        fn to_aligned(slice: &[u8], offset: usize) -> Self {
            U(u16::to_aligned(slice, offset))
        }
    }
    impl BitType for U<32> {
        const BITS: usize = 32;

        fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
            u32::from_aligned(&aligned.0, slice, offset)
        }

        fn to_aligned(slice: &[u8], offset: usize) -> Self {
            U(u32::to_aligned(slice, offset))
        }
    }
    impl BitType for U<64> {
        const BITS: usize = 64;

        fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
            u64::from_aligned(&aligned.0, slice, offset)
        }

        fn to_aligned(slice: &[u8], offset: usize) -> Self {
            U(u64::to_aligned(slice, offset))
        }
    }
}

pub fn ubits<const N: usize>(value: <Underlying<N> as Type>::U) -> U<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::U: std::fmt::Display,
{
    #[cfg(debug_assertions)]
    {
        use num_traits::One;
        assert!(
            value < <Underlying<N> as Type>::U::one() << N,
            "Value too large for {} bits, {} < {}",
            N,
            value,
            <Underlying<N> as Type>::U::one() << N,
        );
    }
    U::new(value)
}
