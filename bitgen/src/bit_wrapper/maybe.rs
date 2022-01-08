use super::*;
pub struct AccessMaybe<
    'a,
    P: BitPredicate + Default,
    M: Mutability,
    O: BitType,
    T: BitType,
    const OFFSET: usize,
> where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    bits: Address<M, Bit<O>>,
    _marker: PhantomData<&'a (P, T)>,
}

impl<'a, P: BitPredicate + Default, M: Mutability, O: BitType, T: BitType, const OFFSET: usize>
    AccessMaybe<'a, P, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    pub(crate) fn new(bits: Address<M, Bit<O>>) -> Self {
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
        O: BitType,
        T: MaybeAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccessMaybe<I> for AccessMaybe<'a, P, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
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
        O,
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
        O: BitType,
        T: BitType + DynAccess,
        const OFFSET: usize,
    > ChildAccessDyn for AccessMaybe<'a, P, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, O, T::Element>;
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
        O: BitType,
        T: TupleAccess<I> + BitType,
        const OFFSET: usize,
        const I: usize,
    > ChildAccess<I> for AccessMaybe<'a, P, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    <T as TupleAccess<I>>::Element: BitType,
    [u8; OFFSET + <T as TupleAccess<I>>::BIT_OFFSET]: Sized,
{
    type Child = AccessMaybe<
        'a,
        P,
        M,
        O,
        <T as TupleAccess<I>>::Element,
        { OFFSET + <T as TupleAccess<I>>::BIT_OFFSET },
    >;

    fn get_child(self) -> Self::Child {
        Self::Child::new(self.bits)
    }
}

impl<'a, P: BitPredicate + Default, M: Mutability, O: BitType, T: BitType, const OFFSET: usize>
    Accessor<O, T, M> for AccessMaybe<'a, P, M, O, T, OFFSET>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = Option<T>;

    type InsertResult = Result<(), ()>;

    fn extract(&self) -> Self::Extracted {
        if P::default().is_true(&unsafe { &*self.bits.to_const() }.mem) {
            Some(T::to_aligned(
                &unsafe { &*self.bits.to_const() }.mem[get_byte_range(OFFSET, T::BITS)],
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
        if P::default().is_true(&unsafe { &*self.bits.to_const() }.mem) {
            T::from_aligned(
                &aligned,
                &mut unsafe { &mut *self.bits.assert_mut().to_mut() }.mem
                    [get_byte_range(OFFSET, T::BITS)],
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
        if P::default().is_true(&unsafe { &*self.bits.to_const() }.mem) {
            let extracted = T::to_aligned(
                &unsafe { &*self.bits.to_const() }.mem[get_byte_range(OFFSET, T::BITS)],
                OFFSET % 8,
            );
            let mapped = f(extracted);
            T::from_aligned(
                &mapped,
                &mut unsafe { &mut *self.bits.assert_mut().to_mut() }.mem
                    [get_byte_range(OFFSET, T::BITS)],
                OFFSET % 8,
            );
            Ok(())
        } else {
            Err(())
        }
    }

    type CastAccess<U: BitType, C: Mutability> = AccessMaybe<'a, P, C, O, U, OFFSET>;

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
