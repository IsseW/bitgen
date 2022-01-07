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

pub const fn bits_to_bytes(n: usize) -> usize {
    (n + 7) / 8
}
