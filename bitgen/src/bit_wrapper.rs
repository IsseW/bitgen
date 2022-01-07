pub mod accessors;
use accessors::TupleAccess;
use num_traits::AsPrimitive;
use std::{fmt, marker::PhantomData, mem, ops::RangeInclusive};
use wyz::{Address, Const, Mut, Mutability};

use crate::{
    bit_num::{bts::Bytes, closest_pow_2, Type, Underlying},
    bit_type::BitType,
    magic::{bits_to_bytes, CTuple, InferEq},
    prelude::U,
};

use self::accessors::{DynAccess, MaybeAccess};

mod access;
mod access_dyn;
mod maybe;
mod maybe_dyn;
mod predicate;

use access::Access;
use access_dyn::AccessDyn;
use maybe::AccessMaybe;
use maybe_dyn::AccessMaybeDyn;
use predicate::*;

pub trait ChildAccess<const I: usize> {
    type Child;
    fn get_child(self) -> Self::Child;
}

pub trait ChildAccessDyn {
    type Child;
    fn get_child_dyn(self, index: usize) -> Self::Child;
    fn get_len(&self) -> usize;
}

pub trait ChildAccessMaybe<const I: usize> {
    type Child;
    fn get_child_maybe(self) -> Self::Child;
}

pub trait ChildAccessDynMaybe {
    type Child;
    fn get_child_dyn(self, index: usize) -> Self::Child;
    fn get_len(&self) -> usize;
}

pub const fn get_byte_range(offset: usize, size: usize) -> RangeInclusive<usize> {
    (offset / 8)..=(offset + size - 1) / 8
}
pub trait Accessor<O: BitType, T: BitType, M: Mutability>: Sized {
    type Extracted;
    type InsertResult;

    fn get<const I: usize>(self) -> <Self as ChildAccess<I>>::Child
    where
        Self: ChildAccess<I>,
    {
        self.get_child()
    }

    fn get_maybe<const I: usize>(self) -> <Self as ChildAccessMaybe<I>>::Child
    where
        Self: ChildAccessMaybe<I>,
    {
        self.get_child_maybe()
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

    fn extract(&self) -> Self::Extracted;

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq;

    fn map(&self, f: impl FnMut(T) -> T) -> Self::InsertResult
    where
        (M, Mut): InferEq;
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
    // Try only having one access function
    pub fn access(&self) -> Access<'_, Const, T, T, 0> {
        Access::new(Address::from(self))
    }

    pub fn access_mut(&mut self) -> Access<'_, Mut, T, T, 0> {
        Access::new(Address::from(self))
    }

    pub fn access_as<U: BitType>(&self) -> Access<'_, Const, T, U, 0>
    where
        CTuple<{ T::BITS }, { U::BITS }>: InferEq,
    {
        Access::new(Address::from(self))
    }

    pub fn access_as_mut<U: BitType>(&mut self) -> Access<'_, Mut, T, U, 0>
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
