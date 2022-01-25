use crate::BitContainer;

use super::*;

pub struct Access<'a, M: Mutability, BC: BitContainer, T: BitType, const OFFSET: usize> {
    bits: Address<M, BC>,
    _marker: PhantomData<&'a T>,
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType, const OFFSET: usize> Clone
    for Access<'a, M, BC, T, OFFSET>
{
    fn clone(&self) -> Self {
        Self {
            bits: self.bits,
            _marker: self._marker,
        }
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType, const OFFSET: usize>
    Access<'a, M, BC, T, OFFSET>
{
    pub(crate) fn new(bits: Address<M, BC>) -> Self {
        Self {
            bits,
            _marker: PhantomData,
        }
    }
}

impl<
        'a,
        M: Mutability,
        BC: 'a + BitContainer,
        T: MaybeAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccessMaybe<I> for Access<'a, M, BC, T, OFFSET>
where
    [u8; OFFSET + <T as MaybeAccess<I>>::BIT_OFFSET]: Sized,
    <T as MaybeAccess<I>>::Element: BitType,
    BitCheck<OFFSET, { <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>:
        BitPredicate,
{
    type Child = AccessMaybe<
        'a,
        BitCheck<
            OFFSET,
            { <T as MaybeAccess<I>>::BIT_OFFSET },
            { <T as MaybeAccess<I>>::EXPECTED },
        >,
        M,
        BC,
        <T as MaybeAccess<I>>::Element,
        { OFFSET + <T as MaybeAccess<I>>::BIT_OFFSET },
    >;

    fn get_child_maybe(self) -> Self::Child {
        Self::Child::new(self.bits)
    }
}

impl<'a, M: Mutability, BC: BitContainer, T: BitType + DynAccess, const OFFSET: usize>
    ChildAccessDyn for Access<'a, M, BC, T, OFFSET>
where
    T::Element: BitType,
{
    type Child = AccessDyn<'a, M, BC, T::Element>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= T::MAX {
            panic!("index out of bounds");
        }
        Self::Child::new(self.bits, OFFSET + T::offset(index))
    }
    fn get_len(&self) -> usize {
        T::MAX
    }
}

impl<
        'a,
        M: Mutability,
        BC: BitContainer,
        T: TupleAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccess<I> for Access<'a, M, BC, T, OFFSET>
where
    <T as TupleAccess<I>>::Element: BitType,
    [u8; OFFSET + <T as TupleAccess<I>>::BIT_OFFSET]: Sized,
{
    type Child = Access<
        'a,
        M,
        BC,
        <T as TupleAccess<I>>::Element,
        { OFFSET + <T as TupleAccess<I>>::BIT_OFFSET },
    >;

    fn get_child(self) -> Self::Child {
        Self::Child::new(self.bits)
    }
}

impl<'a, M: Mutability, T: BitType, BC: BitContainer, const OFFSET: usize> Accessor<BC, T, M>
    for Access<'a, M, BC, T, OFFSET>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = T;

    type InsertResult = ();

    fn extract(&self) -> Self::Extracted {
        T::to_aligned(
            unsafe { &*self.bits.to_const() }.get_range(get_byte_range(OFFSET, T::BITS)),
            OFFSET % 8,
        )
    }

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        T::from_aligned(
            &aligned,
            unsafe { &mut *self.bits.assert_mut().to_mut() }
                .get_range_mut(get_byte_range(OFFSET, T::BITS)),
            OFFSET % 8,
        )
    }

    fn map(&self, mut f: impl FnMut(T) -> T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        self.insert(f(self.extract()))
    }

    type CastAccess<U: BitType, C: Mutability> = Access<'a, C, BC, U, OFFSET>;

    fn access(self) -> Self::CastAccess<T, Const> {
        Self::CastAccess::<T, Const>::new(self.bits.immut())
    }

    unsafe fn access_as<U: BitType>(self) -> Self::CastAccess<U, Const>
    where
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Const>::new(self.bits.immut())
    }

    unsafe fn access_as_mut<U: BitType>(self) -> Self::CastAccess<U, Mut>
    where
        (M, Mut): InferEq,
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Mut>::new(self.bits.assert_mut())
    }
}
