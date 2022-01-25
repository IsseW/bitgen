mod signed;
mod unsigned;

use std::fmt;

use num_traits::AsPrimitive;

use crate::magic::bits_to_bytes;

pub use self::signed::I;
pub use self::unsigned::U;

fn max_with_bits<T: num_traits::PrimInt>(num_bits: usize) -> T {
    if num_bits >= std::mem::size_of::<T>() * 8 {
        T::max_value()
    } else {
        !((!T::zero()) << num_bits)
    }
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

pub fn ubits<const N: usize>(value: <Underlying<N> as Type>::U) -> U<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::U: std::fmt::Display,
{
    U::new(value)
}

pub fn ibits<const N: usize>(value: <Underlying<N> as Type>::I) -> I<N>
where
    Underlying<N>: Type,
    <Underlying<N> as Type>::U: std::fmt::Display,
{
    I::new(value)
}
