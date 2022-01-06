use std::fmt::Debug;

pub struct If<const B: bool>;
pub trait True {}
impl True for If<true> {}
pub trait False {}
impl False for If<false> {}

pub struct Or<A, B> {
    _marker: std::marker::PhantomData<(A, B)>,
}
impl True for Or<If<true>, If<false>> {}
impl True for Or<If<false>, If<true>> {}
impl True for Or<If<true>, If<true>> {}
impl False for Or<If<false>, If<false>> {}

pub struct CTuple<const A: usize, const B: usize>;

// Inclusive
pub struct InRange<const N: usize, const A: usize, const B: usize>;

impl<const N: usize, const A: usize, const B: usize> True for InRange<N, A, B>
where
    If<{ A <= B }>: True,
    If<{ A <= N }>: True,
    If<{ N <= B }>: True,
{
}

impl<const N: usize, const A: usize, const B: usize> False for InRange<N, A, B>
where
    If<{ A <= B }>: True,
    Or<If<{ N < A }>, If<{ N > B }>>: True,
{
}

pub trait InferEq {}

impl<const N: usize> InferEq for CTuple<N, N> {}

impl<T> InferEq for (T, T) {}
impl<T> InferEq for (T, T, T) {}

pub const fn log2(n: usize) -> usize {
    std::mem::size_of::<usize>() * 8 - n.leading_zeros() as usize
}

pub const fn closest_pow_2(n: usize) -> usize {
    let n = if n == 0 { 1 } else { n };
    1 << (log2(n - 1))
}

pub const fn bits_to_bytes(n: usize) -> usize {
    (n + 7) / 8
}

pub trait Type {
    type Higher: Type;
    type U: num_traits::Unsigned + num_traits::PrimInt + Debug + Default;

    type I: num_traits::Signed + num_traits::PrimInt + Debug + Default;

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
