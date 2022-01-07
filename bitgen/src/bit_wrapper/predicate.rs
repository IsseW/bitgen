use super::*;
pub trait BitPredicate {
    fn is_true(&self, slice: &[u8]) -> bool;
}
#[derive(Default)]
pub struct PredicateAnd<A: BitPredicate, B: BitPredicate>(pub A, pub B);

impl<A: BitPredicate, B: BitPredicate> BitPredicate for PredicateAnd<A, B> {
    fn is_true(&self, slice: &[u8]) -> bool {
        self.0.is_true(slice) && self.1.is_true(slice)
    }
}

#[derive(Default)]
pub struct BitCheck<const OFFSET: usize, const NUM_BITS: usize, const BITS: u32>;

pub struct BitCheckDyn<const NUM_BITS: usize, const BITS: u32>(pub usize);

impl<const OFFSET: usize, const NUM_BITS: usize, const BITS: u32> BitPredicate
    for BitCheck<OFFSET, NUM_BITS, BITS>
where
    Underlying<NUM_BITS>: Type,
    U<NUM_BITS>: BitType,
{
    fn is_true(&self, slice: &[u8]) -> bool {
        U::<NUM_BITS>::to_aligned(&slice[get_byte_range(OFFSET, NUM_BITS)], OFFSET % 8)
            .0
            .as_()
            == BITS
    }
}

impl<const NUM_BITS: usize, const BITS: u32> BitPredicate for BitCheckDyn<NUM_BITS, BITS>
where
    Underlying<NUM_BITS>: Type,
    U<NUM_BITS>: BitType,
{
    fn is_true(&self, slice: &[u8]) -> bool {
        U::<NUM_BITS>::to_aligned(&slice[get_byte_range(self.0, NUM_BITS)], self.0 % 8)
            .0
            .as_()
            == BITS
    }
}