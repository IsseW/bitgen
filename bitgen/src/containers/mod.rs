mod bit;
mod bit_vec;
mod raw_vec;

use std::ops::Range;

pub use bit::Bit;
pub use bit_vec::BitVec;

pub trait BitContainer {
    fn get_range(&self, range: Range<usize>) -> &[u8];
    fn get_range_mut(&mut self, range: Range<usize>) -> &mut [u8];
    fn get_full(&self) -> &[u8];
}
