use crate::{
    bit_type::BitType,
    magic::{If, True},
};

pub trait TupleAccess<const I: usize> {
    type Element;
    const BIT_OFFSET: usize;
}

impl<T: BitType, const I: usize, const N: usize> TupleAccess<I> for [T; N]
where
    If<{ I < N }>: True,
{
    type Element = T;
    const BIT_OFFSET: usize = I * T::BITS;
}

impl<A: BitType> TupleAccess<0> for (A,) {
    type Element = A;
    const BIT_OFFSET: usize = 0;
}

impl<A: BitType, B: BitType> TupleAccess<0> for (A, B) {
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType> TupleAccess<1> for (A, B) {
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}

impl<A: BitType, B: BitType, C: BitType> TupleAccess<0> for (A, B, C) {
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType, C: BitType> TupleAccess<1> for (A, B, C) {
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<A: BitType, B: BitType, C: BitType> TupleAccess<2> for (A, B, C) {
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}

impl<A: BitType, B: BitType, C: BitType, D: BitType> TupleAccess<0> for (A, B, C, D) {
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType> TupleAccess<1> for (A, B, C, D) {
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType> TupleAccess<2> for (A, B, C, D) {
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType> TupleAccess<3> for (A, B, C, D) {
    type Element = D;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS;
}

impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType> TupleAccess<0>
    for (A, B, C, D, E)
{
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType> TupleAccess<1>
    for (A, B, C, D, E)
{
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType> TupleAccess<2>
    for (A, B, C, D, E)
{
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType> TupleAccess<3>
    for (A, B, C, D, E)
{
    type Element = D;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType> TupleAccess<4>
    for (A, B, C, D, E)
{
    type Element = E;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS;
}

impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<0>
    for (A, B, C, D, E, F)
{
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<1>
    for (A, B, C, D, E, F)
{
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<2>
    for (A, B, C, D, E, F)
{
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<3>
    for (A, B, C, D, E, F)
{
    type Element = D;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<4>
    for (A, B, C, D, E, F)
{
    type Element = E;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType> TupleAccess<5>
    for (A, B, C, D, E, F)
{
    type Element = F;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS + E::BITS;
}

impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<0> for (A, B, C, D, E, F, G)
{
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<1> for (A, B, C, D, E, F, G)
{
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<2> for (A, B, C, D, E, F, G)
{
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<3> for (A, B, C, D, E, F, G)
{
    type Element = D;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<4> for (A, B, C, D, E, F, G)
{
    type Element = E;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<5> for (A, B, C, D, E, F, G)
{
    type Element = F;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS + E::BITS;
}
impl<A: BitType, B: BitType, C: BitType, D: BitType, E: BitType, F: BitType, G: BitType>
    TupleAccess<6> for (A, B, C, D, E, F, G)
{
    type Element = G;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS + E::BITS + F::BITS;
}

impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<0> for (A, B, C, D, E, F, G, H)
{
    type Element = A;
    const BIT_OFFSET: usize = 0;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<1> for (A, B, C, D, E, F, G, H)
{
    type Element = B;
    const BIT_OFFSET: usize = A::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<2> for (A, B, C, D, E, F, G, H)
{
    type Element = C;
    const BIT_OFFSET: usize = A::BITS + B::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<3> for (A, B, C, D, E, F, G, H)
{
    type Element = D;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<4> for (A, B, C, D, E, F, G, H)
{
    type Element = E;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<5> for (A, B, C, D, E, F, G, H)
{
    type Element = F;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS + E::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<6> for (A, B, C, D, E, F, G, H)
{
    type Element = G;
    const BIT_OFFSET: usize = A::BITS + B::BITS + C::BITS + D::BITS + E::BITS + F::BITS;
}
impl<
        A: BitType,
        B: BitType,
        C: BitType,
        D: BitType,
        E: BitType,
        F: BitType,
        G: BitType,
        H: BitType,
    > TupleAccess<7> for (A, B, C, D, E, F, G, H)
{
    type Element = G;
    const BIT_OFFSET: usize =
        A::BITS + B::BITS + C::BITS + D::BITS + E::BITS + F::BITS + G::BITS;
}
