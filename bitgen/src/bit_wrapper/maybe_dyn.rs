use super::*;
#[derive(Clone)]
pub struct AccessMaybeDyn<'a, P: BitPredicate, M: Mutability, O: BitType, T: BitType>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    bits: Address<M, Bit<O>>,
    offset: usize,
    predicate: P,
    _marker: PhantomData<&'a T>,
}

impl<'a, P: BitPredicate, M: Mutability, O: BitType, T: BitType> AccessMaybeDyn<'a, P, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    pub(crate) fn new(bits: Address<M, Bit<O>>, offset: usize, predicate: P) -> Self {
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
        O: BitType,
        T: MaybeAccess<I> + BitType,
        const I: usize,
    > ChildAccessMaybe<I> for AccessMaybeDyn<'a, P, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; <T as MaybeAccess<I>>::EXPECTED as usize]: Sized,
    <T as MaybeAccess<I>>::Element: BitType,
    U<{ <T as MaybeAccess<I>>::BIT_OFFSET }>: BitType,
    Bytes<{ closest_pow_2(bits_to_bytes(<T as MaybeAccess<I>>::BIT_OFFSET)) }>: Type,
{
    type Child = AccessMaybeDyn<
        'a,
        PredicateAnd<
            BitCheckDyn<{ <T as MaybeAccess<I>>::BIT_OFFSET }, { <T as MaybeAccess<I>>::EXPECTED }>,
            P,
        >,
        M,
        O,
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

impl<'a, P: BitPredicate, M: Mutability, O: BitType, T: BitType + DynAccess> ChildAccessDyn
    for AccessMaybeDyn<'a, P, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, O, T::Element>;
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
        O: BitType,
        T: BitType + TupleAccess<I>,
        const I: usize,
    > ChildAccess<I> for AccessMaybeDyn<'a, P, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    T::Element: BitType,
{
    type Child = AccessMaybeDyn<'a, P, M, O, <T as TupleAccess<I>>::Element>;
    fn get_child(self) -> Self::Child {
        Self::Child::new(
            self.bits,
            self.offset + <T as TupleAccess<I>>::BIT_OFFSET,
            self.predicate,
        )
    }
}

impl<'a, P: BitPredicate, M: Mutability, O: BitType, T: BitType> Accessor<O, T, M>
    for AccessMaybeDyn<'a, P, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = Option<T>;

    type InsertResult = Result<(), ()>;

    fn extract(&self) -> Self::Extracted {
        if self
            .predicate
            .is_true(&unsafe { &*self.bits.to_const() }.mem)
        {
            Some(T::to_aligned(
                &unsafe { &*self.bits.to_const() }.mem[get_byte_range(self.offset, T::BITS)],
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
            .is_true(&unsafe { &*self.bits.to_const() }.mem)
        {
            T::from_aligned(
                &aligned,
                &mut unsafe { &mut *self.bits.assert_mut().to_mut() }.mem
                    [get_byte_range(self.offset, T::BITS)],
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
            .is_true(&unsafe { &*self.bits.to_const() }.mem)
        {
            let extracted = T::to_aligned(
                &unsafe { &*self.bits.to_const() }.mem[get_byte_range(self.offset, T::BITS)],
                self.offset % 8,
            );
            let mapped = f(extracted);
            T::from_aligned(
                &mapped,
                &mut unsafe { &mut *self.bits.assert_mut().to_mut() }.mem
                    [get_byte_range(self.offset, T::BITS)],
                self.offset % 8,
            );
            Ok(())
        } else {
            Err(())
        }
    }

    type CastAccess<U: BitType, C: Mutability> = AccessMaybeDyn<'a, P, C, O, U>;

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
        Self::CastAccess::<U, Mut>::new(
            self.bits.assert_mut(),
            self.offset,
            self.predicate,
        )
    }
}
