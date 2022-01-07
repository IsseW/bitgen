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
        O: 'a + BitType,
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

impl<'a, P: BitPredicate, M: Mutability, O: 'a + BitType, T: BitType + DynAccess> ChildAccessDyn
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

impl<'a, P: BitPredicate + Default, M: Mutability, O: BitType, T: BitType> Accessor<O, T, M>
    for AccessMaybeDyn<'a, P, M, O, T>
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
        if P::default().is_true(&unsafe { &*self.bits.to_const() }.mem) {
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
        self.insert(f(self.extract().ok_or(())?))
    }
}
