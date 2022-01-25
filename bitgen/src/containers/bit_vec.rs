use std::{marker::PhantomData, ptr};

use wyz::{Address, Const, Mut};

use crate::{
    bit_wrapper::{access_dyn::AccessDyn, get_byte_range},
    magic::bits_to_bytes,
    BitContainer, BitType,
};

use super::raw_vec::RawVec;

pub struct BitVec<T: BitType>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    buf: RawVec<[u8; bits_to_bytes(T::BITS)]>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: BitType> BitVec<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn ptr(&self) -> *mut u8 {
        self.buf.ptr.as_ptr() as *mut u8
    }

    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn allocated_bits(&self) -> usize {
        self.buf.cap * bits_to_bytes(T::BITS) * 8
    }

    pub fn push(&mut self, t: T) {
        if (self.len + 1) * T::BITS > self.allocated_bits() {
            self.buf.grow()
        }
        let bit_offset = self.len * T::BITS;
        let slice = self.get_range_mut(get_byte_range(bit_offset, T::BITS));
        T::from_aligned(&t, slice, bit_offset % 8);

        self.len += 1;
    }

    /// Remove the top element, to pop and return use `pop_back`
    pub fn pop(&mut self) {
        if self.len == 0 {
            return;
        }
        self.len -= 1;
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let bit_offset = self.len * T::BITS;
        let slice = self.get_range(get_byte_range(bit_offset, T::BITS));
        Some(T::to_aligned(slice, bit_offset % 8))
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let bit_offset = index * T::BITS;
        let slice = self.get_range(get_byte_range(bit_offset, T::BITS));
        Some(T::to_aligned(slice, bit_offset % 8))
    }

    pub fn access(&self, index: usize) -> AccessDyn<'_, Const, Self, T> {
        AccessDyn::new(Address::from(self), index * T::BITS)
    }
    pub fn access_mut(&mut self, index: usize) -> AccessDyn<'_, Mut, Self, T> {
        AccessDyn::new(Address::from(self), index * T::BITS)
    }
}

impl<T: BitType> BitContainer for BitVec<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn get_range(&self, range: std::ops::Range<usize>) -> &[u8] {
        unsafe { &*ptr::slice_from_raw_parts(self.ptr().add(range.start), range.end - range.start) }
    }

    fn get_range_mut(&mut self, range: std::ops::Range<usize>) -> &mut [u8] {
        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(
                self.ptr().add(range.start),
                range.end - range.start,
            )
        }
    }

    fn get_full(&self) -> &[u8] {
        unsafe { &*ptr::slice_from_raw_parts(self.ptr(), self.len) }
    }
}
