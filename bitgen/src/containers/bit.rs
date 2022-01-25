use std::{fmt, marker::PhantomData, mem};

use wyz::{Address, Const, Mut};

use crate::{
    bit_wrapper::access::Access,
    magic::{bits_to_bytes, CTuple, InferEq},
    BitContainer, BitType,
};

pub struct Bit<T: BitType>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    mem: [u8; bits_to_bytes(T::BITS)],
    _marker: PhantomData<T>,
}

impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    /// Get an immutable accessor
    pub fn access(&self) -> Access<'_, Const, Self, T, 0> {
        Access::new(Address::from(self))
    }

    /// Get a mutable accessor
    pub fn access_mut(&mut self) -> Access<'_, Mut, Self, T, 0> {
        Access::new(Address::from(self))
    }

    /// Get an immutable accessor with a certain type
    /// # Safety
    /// This is basically a `mem::transmute`, therefore it's very unsafe.
    pub unsafe fn access_as<U: BitType>(&self) -> Access<'_, Const, Self, U, 0>
    where
        CTuple<{ T::BITS }, { U::BITS }>: InferEq,
    {
        Access::new(Address::from(self))
    }

    /// Get a mutable accessor with a certain type
    /// # Safety
    /// This is basically a `mem::transmute`, therefore it's very unsafe.
    pub unsafe fn access_as_mut<U: BitType>(&mut self) -> Access<'_, Mut, Self, U, 0>
    where
        CTuple<{ T::BITS }, { U::BITS }>: InferEq,
    {
        Access::new(Address::from(self))
    }
}

impl<T: BitType> fmt::Debug for Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bit[")?;
        for (i, byte) in self.mem.iter().enumerate() {
            if i > 0 {
                write!(f, "_")?;
            }
            write!(f, "{:08b}", byte)?;
        }
        write!(f, "]")
    }
}

impl<T: BitType> Default for Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn default() -> Self {
        Self {
            mem: [0; bits_to_bytes(T::BITS)],
            _marker: PhantomData,
        }
    }
}

impl<T: BitType> From<T> for Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    fn from(value: T) -> Self {
        let mut mem: [u8; bits_to_bytes(T::BITS)] = unsafe { mem::zeroed() };
        T::from_aligned(&value, &mut mem, 0);
        Self {
            mem,
            _marker: PhantomData,
        }
    }
}

impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    CTuple<{ T::BITS }, 8>: InferEq,
{
    pub fn as_u8(&self) -> u8 {
        self.mem[0]
    }
}
impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    CTuple<{ T::BITS }, 16>: InferEq,
{
    pub fn as_u16(&self) -> u16 {
        unsafe { mem::transmute_copy(&self.mem) }
    }
}
impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    CTuple<{ T::BITS }, 32>: InferEq,
{
    pub fn as_u32(&self) -> u32 {
        unsafe { mem::transmute_copy(&self.mem) }
    }
}
impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    CTuple<{ T::BITS }, 64>: InferEq,
{
    pub fn as_u64(&self) -> u64 {
        unsafe { mem::transmute_copy(&self.mem) }
    }
}
impl<T: BitType> Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    CTuple<{ T::BITS }, 128>: InferEq,
{
    pub fn as_u128(&self) -> u128 {
        unsafe { mem::transmute_copy(&self.mem) }
    }
}

impl<T: BitType> BitContainer for Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn get_range(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.mem[range]
    }

    fn get_range_mut(&mut self, range: std::ops::Range<usize>) -> &mut [u8] {
        &mut self.mem[range]
    }

    fn get_full(&self) -> &[u8] {
        &self.mem
    }
}
