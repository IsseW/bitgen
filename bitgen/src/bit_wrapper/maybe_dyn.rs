use crate::BitContainer;

use super::*;

pub struct AccessMaybeDyn<'a, P: BitPredicate, M: Mutability, BC: BitContainer, T: BitType> {
    bits: Address<M, BC>,
    offset: usize,
    predicate: P,
    _marker: PhantomData<&'a T>,
}

impl<'a, P: BitPredicate, M: Mutability, BC: BitContainer, T: BitType> Clone
    for AccessMaybeDyn<'a, P, M, BC, T>
{
    fn clone(&self) -> Self {
        Self {
            bits: self.bits,
            offset: self.offset,
            predicate: self.predicate.clone(),
            _marker: self._marker,
        }
    }
}

impl<'a, P: BitPredicate, M: Mutability, BC: BitContainer, T: BitType>
    AccessMaybeDyn<'a, P, M, BC, T>
{
    pub(crate) fn new(bits: Address<M, BC>, offset: usize, predicate: P) -> Self {
        Self {
            bits,
            offset,
            predicate,
            _marker: PhantomData,
        }
    }
}

impl<
        'a,
        P: BitPredicate,
        M: Mutability,
        BC: BitContainer,
        T: MaybeAccess<I> + BitType,
        const I: usize,
    > ChildAccessMaybe<I> for AccessMaybeDyn<'a, P, M, BC, T>
where
    <T as MaybeAccess<I>>::Element: BitType,
    BitCheckDyn<{ <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>:
        BitPredicate,
{
    type Child = AccessMaybeDyn<
        'a,
        PredicateAnd<
            BitCheckDyn<{ <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>,
            P,
        >,
        M,
        BC,
        <T as MaybeAccess<I>>::Element,
    >;

    fn get_child_maybe(self) -> Self::Child {
        Self::Child::new(
            self.bits,
            self.offset + <T as MaybeAccess<I>>::BIT_OFFSET,
            PredicateAnd(BitCheckDyn(self.offset), self.predicate),
        )
    }
}

impl<'a, P: BitPredicate, M: Mutability, BC: BitContainer, T: BitType + DynAccess> ChildAccessDyn
    for AccessMaybeDyn<'a, P, M, BC, T>
where
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, BC, T::Element>;
    fn get_child_dyn(self, index: usize) -> Self::Child {
        if index >= T::MAX {
            panic!("index out of bounds");
        }
        Self::Child::new(self.bits, self.offset + T::offset(index), self.predicate)
    }
    fn get_len(&self) -> usize {
        T::MAX
    }
}

impl<
        'a,
        P: BitPredicate,
        M: Mutability,
        BC: BitContainer,
        T: BitType + TupleAccess<I>,
        const I: usize,
    > ChildAccess<I> for AccessMaybeDyn<'a, P, M, BC, T>
where
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, BC, <T as TupleAccess<I>>::Element>;
    fn get_child(self) -> Self::Child {
        Self::Child::new(
            self.bits,
            self.offset + <T as TupleAccess<I>>::BIT_OFFSET,
            self.predicate,
        )
    }
}

impl<'a, P: BitPredicate, M: Mutability, BC: BitContainer, T: BitType> Accessor<BC, T, M>
    for AccessMaybeDyn<'a, P, M, BC, T>
where
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = Option<T>;

    type InsertResult = Result<(), ()>;

    fn extract(&self) -> Self::Extracted {
        if self
            .predicate
            .is_true(unsafe { &*self.bits.to_const() }.get_full())
        {
            Some(T::to_aligned(
                unsafe { &*self.bits.to_const() }.get_range(get_byte_range(self.offset, T::BITS)),
                self.offset % 8,
            ))
        } else {
            None
        }
    }

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        if self
            .predicate
            .is_true(unsafe { &*self.bits.to_const() }.get_full())
        {
            T::from_aligned(
                &aligned,
                unsafe { &mut *self.bits.assert_mut().to_mut() }
                    .get_range_mut(get_byte_range(self.offset, T::BITS)),
                self.offset % 8,
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
        if self
            .predicate
            .is_true(unsafe { &*self.bits.to_const() }.get_full())
        {
            let extracted = T::to_aligned(
                unsafe { &*self.bits.to_const() }.get_range(get_byte_range(self.offset, T::BITS)),
                self.offset % 8,
            );
            let mapped = f(extracted);
            T::from_aligned(
                &mapped,
                unsafe { &mut *self.bits.assert_mut().to_mut() }
                    .get_range_mut(get_byte_range(self.offset, T::BITS)),
                self.offset % 8,
            );
            Ok(())
        } else {
            Err(())
        }
    }

    type CastAccess<U: BitType, C: Mutability> = AccessMaybeDyn<'a, P, C, BC, U>;

    fn access(self) -> Self::CastAccess<T, Const> {
        Self::CastAccess::<T, Const>::new(self.bits.immut(), self.offset, self.predicate)
    }

    unsafe fn access_as<U: BitType>(self) -> Self::CastAccess<U, Const>
    where
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Const>::new(self.bits.immut(), self.offset, self.predicate)
    }

    unsafe fn access_as_mut<U: BitType>(self) -> Self::CastAccess<U, Mut>
    where
        (M, Mut): InferEq,
        CTuple<{ <U as BitType>::BITS }, { <T as BitType>::BITS }>: InferEq,
    {
        Self::CastAccess::<U, Mut>::new(self.bits.assert_mut(), self.offset, self.predicate)
    }
}
