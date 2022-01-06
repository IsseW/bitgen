mod tuple_access;
use std::{fmt, marker::PhantomData, mem, ops::RangeInclusive};
use tuple_access::TupleAccess;
use wyz::{Address, Const, Mut, Mutability};

use crate::{
    bit_type::BitType,
    magic::{bits_to_bytes, InferEq},
};

#[derive(Clone)]
pub struct Access<'a, M: Mutability, O: BitType, T: BitType, const OFFSET: usize>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    bits: Address<M, Bit<O>>,
    _marker: PhantomData<&'a T>,
}

#[derive(Clone)]
pub struct AccessDyn<'a, M: Mutability, O: BitType, T: BitType>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    bits: Address<M, Bit<O>>,
    offset: usize,
    _marker: PhantomData<&'a T>,
}

pub trait ChildAccess<const I: usize> {
    type Child;
    fn get_child(self) -> Self::Child;
}

pub trait ChildAccessDyn {
    type Child;
    fn get_child_dyn(self, index: usize) -> Self::Child;
    fn get_len(&self) -> usize;
}

impl<'a, M: Mutability, O: 'a + BitType, T: BitType, const N: usize, const OFFSET: usize>
    ChildAccessDyn for Access<'a, M, O, [T; N], OFFSET>
where
    [T; N]: BitType,
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    type Child = AccessDyn<'a, M, O, T>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= N {
            panic!("index out of bounds");
        }
        Self::Child {
            bits: self.bits,
            offset: OFFSET + index * T::BITS,
            _marker: PhantomData,
        }
    }
    fn get_len(&self) -> usize {
        N
    }
}

impl<'a, M: Mutability, O: 'a + BitType, T: BitType, const N: usize> ChildAccessDyn
    for AccessDyn<'a, M, O, [T; N]>
where
    [T; N]: BitType,
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    type Child = AccessDyn<'a, M, O, T>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= N {
            panic!("index out of bounds");
        }
        Self::Child {
            bits: self.bits,
            offset: self.offset + index * T::BITS,
            _marker: PhantomData,
        }
    }
    fn get_len(&self) -> usize {
        N
    }
}

impl<'a, M: Mutability, O: 'a + BitType, T: TupleAccess<I> + BitType, const I: usize> ChildAccess<I>
    for AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    <T as TupleAccess<I>>::Element: BitType,
{
    type Child = AccessDyn<'a, M, O, <T as TupleAccess<I>>::Element>;

    fn get_child(self) -> Self::Child {
        let offset = self.offset + <T as TupleAccess<I>>::BIT_OFFSET;
        AccessDyn {
            bits: self.bits,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<
        'a,
        M: Mutability,
        O: 'a + BitType,
        T: TupleAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccess<I> for Access<'a, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    <T as TupleAccess<I>>::Element: BitType,
    [u8; OFFSET + <T as TupleAccess<I>>::BIT_OFFSET]: Sized,
{
    type Child = Access<
        'a,
        M,
        O,
        <T as TupleAccess<I>>::Element,
        { OFFSET + <T as TupleAccess<I>>::BIT_OFFSET },
    >;

    fn get_child(self) -> Self::Child {
        Self::Child {
            bits: self.bits,
            _marker: PhantomData,
        }
    }
}

pub trait Accessor<O: BitType, T: BitType, M: Mutability>: Sized {
    fn get<const I: usize>(self) -> <Self as ChildAccess<I>>::Child
    where
        Self: ChildAccess<I>,
    {
        self.get_child()
    }

    fn get_dyn(self, index: usize) -> <Self as ChildAccessDyn>::Child
    where
        Self: ChildAccessDyn,
    {
        self.get_child_dyn(index)
    }

    fn len(&self) -> usize
    where
        Self: ChildAccessDyn,
    {
        self.get_len()
    }

    fn get_bits(&self) -> Address<M, Bit<O>>
    where
        [u8; bits_to_bytes(O::BITS)]: Sized;

    fn get_offset(&self) -> usize;

    fn get_byte_range(&self) -> RangeInclusive<usize> {
        (self.get_offset() / 8)..=(self.get_offset() + T::BITS - 1) / 8
    }

    fn extract(&self) -> T
    where
        [u8; mem::size_of::<T>()]: Sized,
        [u8; bits_to_bytes(O::BITS)]: Sized,
    {
        T::to_aligned(
            &unsafe { &*self.get_bits().to_const() }.mem[self.get_byte_range()],
            self.get_offset() % 8,
        )
    }

    fn insert(&self, aligned: T)
    where
        (M, Mut): InferEq,
        [u8; mem::size_of::<T>()]: Sized,
        [u8; bits_to_bytes(O::BITS)]: Sized,
    {
        T::from_aligned(
            &aligned,
            &mut unsafe { &mut *self.get_bits().assert_mut().to_mut() }.mem[self.get_byte_range()],
            self.get_offset() % 8,
        )
    }

    fn map(&self, mut f: impl FnMut(T) -> T)
    where
        (M, Mut): InferEq,
        [u8; mem::size_of::<T>()]: Sized,
        [u8; bits_to_bytes(O::BITS)]: Sized,
    {
        self.insert(f(self.extract()));
    }
}

impl<'a, M: Mutability, T: BitType, O: BitType, const OFFSET: usize> Accessor<O, T, M>
    for Access<'a, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn get_bits(&self) -> Address<M, Bit<O>> {
        self.bits
    }

    fn get_offset(&self) -> usize {
        OFFSET
    }
}

impl<'a, M: Mutability, T: BitType, O: BitType> Accessor<O, T, M> for AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn get_bits(&self) -> Address<M, Bit<O>> {
        self.bits
    }

    fn get_offset(&self) -> usize {
        self.offset
    }
}

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
    pub fn access(&self) -> Access<'_, Const, T, T, 0> {
        Access {
            bits: Address::from(self),
            _marker: PhantomData,
        }
    }

    pub fn access_mut(&mut self) -> Access<'_, Mut, T, T, 0> {
        Access {
            bits: Address::from(self),
            _marker: PhantomData,
        }
    }
}

impl<T: BitType> fmt::Debug for Bit<T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bit[")?;
        let mut i = 0;
        for byte in self.mem {
            if i > 0 {
                write!(f, "_")?;
            }
            write!(f, "{:08b}", byte)?;
            i += 1;
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
