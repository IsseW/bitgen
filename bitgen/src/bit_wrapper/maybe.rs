use crate::BitContainer;

use super::*;
pub struct AccessMaybe<
    'a,
    P: BitPredicate + Default,
    M: Mutability,
    BC: BitContainer,
    T: BitType,
    const OFFSET: usize,
> {
    bits: Address<M, BC>,
    _marker: PhantomData<&'a (P, T)>,
}

impl<
        'a,
        P: BitPredicate + Default,
        M: Mutability,
        BC: BitContainer,
        T: BitType,
        const OFFSET: usize,
    > Clone for AccessMaybe<'a, P, M, BC, T, OFFSET>
{
    fn clone(&self) -> Self {
        Self {
            bits: self.bits.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<
        'a,
        P: BitPredicate + Default,
        M: Mutability,
        BC: BitContainer,
        T: BitType,
        const OFFSET: usize,
    > AccessMaybe<'a, P, M, BC, T, OFFSET>
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
        P: BitPredicate + Default,
        M: Mutability,
        BC: 'a + BitContainer,
        T: MaybeAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccessMaybe<I> for AccessMaybe<'a, P, M, BC, T, OFFSET>
where
    [u8; OFFSET + <T as MaybeAccess<I>>::BIT_OFFSET]: Sized,
    <T as MaybeAccess<I>>::Element: BitType,
    BitCheck<OFFSET, { <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>:
        BitPredicate,
{
    type Child = AccessMaybe<
        'a,
        PredicateAnd<
            BitCheck<
                OFFSET,
                { <T as MaybeAccess<I>>::BIT_OFFSET },
                { <T as MaybeAccess<I>>::EXPECTED },
            >,
            P,
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

impl<
        'a,
        P: BitPredicate + Default,
        M: Mutability,
        BC: BitContainer,
        T: BitType + DynAccess,
        const OFFSET: usize,
    > ChildAccessDyn for AccessMaybe<'a, P, M, BC, T, OFFSET>
where
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, BC, T::Element>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= T::MAX {
            panic!("index out of bounds");
        }
        Self::Child::new(self.bits, OFFSET + T::offset(index), P::default())
    }
    fn get_len(&self) -> usize {
        T::MAX
    }
}

impl<
        'a,
        P: BitPredicate + Default,
        M: Mutability,
        BC: BitContainer,
        T: TupleAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccess<I> for AccessMaybe<'a, P, M, BC, T, OFFSET>
where
    <T as TupleAccess<I>>::Element: BitType,
    [u8; OFFSET + <T as TupleAccess<I>>::BIT_OFFSET]: Sized,
{
    type Child = AccessMaybe<
        'a,
        P,
        M,
        BC,
        <T as TupleAccess<I>>::Element,
        { OFFSET + <T as TupleAccess<I>>::BIT_OFFSET },
    >;

    fn get_child(self) -> Self::Child {
        Self::Child::new(self.bits)
    }
}

impl<
        'a,
        P: BitPredicate + Default,
        M: Mutability,
        BC: BitContainer,
        T: BitType,
        const OFFSET: usize,
    > Accessor<BC, T, M> for AccessMaybe<'a, P, M, BC, T, OFFSET>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = Option<T>;

    type InsertResult = Result<(), ()>;

    fn extract(&self) -> Self::Extracted {
        if P::default().is_true(unsafe { &*self.bits.to_const() }.get_full()) {
            Some(T::to_aligned(
                unsafe { &*self.bits.to_const() }.get_range(get_byte_range(OFFSET, T::BITS)),
                OFFSET % 8,
            ))
        } else {
            None
        }
    }

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        if P::default().is_true(unsafe { &*self.bits.to_const() }.get_full()) {
            T::from_aligned(
                &aligned,
                unsafe { &mut *self.bits.assert_mut().to_mut() }
                    .get_range_mut(get_byte_range(OFFSET, T::BITS)),
                OFFSET % 8,
            );
            Ok(())
        } else {
            Ok(())
        }
    }

    fn map(&self, mut f: impl FnMut(T) -> T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        if P::default().is_true(unsafe { &*self.bits.to_const() }.get_full()) {
            let extracted = T::to_aligned(
                unsafe { &*self.bits.to_const() }.get_range(get_byte_range(OFFSET, T::BITS)),
                OFFSET % 8,
            );
            let mapped = f(extracted);
            T::from_aligned(
                &mapped,
                unsafe { &mut *self.bits.assert_mut().to_mut() }
                    .get_range_mut(get_byte_range(OFFSET, T::BITS)),
                OFFSET % 8,
            );
            Ok(())
        } else {
            Err(())
        }
    }

    type CastAccess<U: BitType, C: Mutability> = AccessMaybe<'a, P, C, BC, U, OFFSET>;

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
