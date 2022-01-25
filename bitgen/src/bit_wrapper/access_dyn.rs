use crate::BitContainer;

use super::*;

pub struct AccessDyn<'a, M: Mutability, BC: BitContainer, T: BitType> {
    bits: Address<M, BC>,
    offset: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType> Clone for AccessDyn<'a, M, BC, T> {
    fn clone(&self) -> Self {
        Self {
            bits: self.bits,
            offset: self.offset,
            _marker: self._marker,
        }
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType> AccessDyn<'a, M, BC, T> {
    pub(crate) fn new(bits: Address<M, BC>, offset: usize) -> Self {
        Self {
            bits,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType + DynAccess> ChildAccessDyn
    for AccessDyn<'a, M, BC, T>
where
    T::Element: BitType,
{
    type Child = AccessDyn<'a, M, BC, T::Element>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= T::MAX {
            panic!("index out of bounds");
        }
        Self::Child::new(self.bits, self.offset + T::offset(index))
    }
    fn get_len(&self) -> usize {
        T::MAX
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType + MaybeAccess<I>, const I: usize>
    ChildAccessMaybe<I> for AccessDyn<'a, M, BC, T>
where
    <T as MaybeAccess<I>>::Element: BitType,
    BitCheckDyn<{ <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>:
        BitPredicate,
{
    type Child = AccessMaybeDyn<
        'a,
        BitCheckDyn<{ <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>,
        M,
        BC,
        <T as MaybeAccess<I>>::Element,
    >;
    fn get_child_maybe(self) -> Self::Child {
        Self::Child::new(self.bits, self.offset, BitCheckDyn(self.offset))
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: TupleAccess<I> + BitType, const I: usize>
    ChildAccess<I> for AccessDyn<'a, M, BC, T>
where
    <T as TupleAccess<I>>::Element: BitType,
{
    type Child = AccessDyn<'a, M, BC, <T as TupleAccess<I>>::Element>;

    fn get_child(self) -> Self::Child {
        Self::Child::new(self.bits, self.offset + <T as TupleAccess<I>>::BIT_OFFSET)
    }
}

impl<'a, M: Mutability, T: BitType, BC: BitContainer> Accessor<BC, T, M> for AccessDyn<'a, M, BC, T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = T;

    type InsertResult = ();

    fn extract(&self) -> Self::Extracted {
        T::to_aligned(
            unsafe { &*self.bits.to_const() }.get_range(get_byte_range(self.offset, T::BITS)),
            self.offset % 8,
        )
    }

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        T::from_aligned(
            &aligned,
            unsafe { &mut *self.bits.assert_mut().to_mut() }
                .get_range_mut(get_byte_range(self.offset, T::BITS)),
            self.offset % 8,
        )
    }

    fn map(&self, mut f: impl FnMut(T) -> T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        self.insert(f(self.extract()))
    }

    type CastAccess<U: BitType, C: Mutability> = AccessDyn<'a, C, BC, U>;

    fn access(self) -> Self::CastAccess<T, Const> {
        Self::CastAccess::<T, Const>::new(self.bits.immut(), self.offset)
    }

    unsafe fn access_as<U: BitType>(self) -> Self::CastAccess<U, Const>
    where
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Const>::new(self.bits.immut(), self.offset)
    }

    unsafe fn access_as_mut<U: BitType>(self) -> Self::CastAccess<U, Mut>
    where
        (M, Mut): InferEq,
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Mut>::new(self.bits.assert_mut(), self.offset)
    }
}
