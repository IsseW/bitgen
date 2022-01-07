use super::*;
#[derive(Clone)]
pub struct AccessDyn<'a, M: Mutability, O: BitType, T: BitType>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    bits: Address<M, Bit<O>>,
    offset: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, M: Mutability, O: BitType, T: BitType> AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
{
    pub(crate) fn new(bits: Address<M, Bit<O>>, offset: usize) -> Self {
        Self {
            bits,
            offset,
            _marker: PhantomData,
        }
    }
}

impl<'a, M: Mutability, O: 'a + BitType, T: BitType + DynAccess> ChildAccessDyn
    for AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    T::Element: BitType,
{
    type Child = AccessDyn<'a, M, O, T::Element>;
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

impl<'a, M: Mutability, O: 'a + BitType, T: TupleAccess<I> + BitType, const I: usize> ChildAccess<I>
    for AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    <T as TupleAccess<I>>::Element: BitType,
{
    type Child = AccessDyn<'a, M, O, <T as TupleAccess<I>>::Element>;

    fn get_child(self) -> Self::Child {
        Self::Child::new(self.bits, self.offset + <T as TupleAccess<I>>::BIT_OFFSET)
    }
}

impl<'a, M: Mutability, T: BitType, O: BitType> Accessor<O, T, M> for AccessDyn<'a, M, O, T>
where
    [u8; bits_to_bytes(O::BITS)]: Sized,
    [u8; bits_to_bytes(T::BITS)]: Sized,
    [u8; mem::size_of::<T>()]: Sized,
{
    type Extracted = T;

    type InsertResult = ();

    fn extract(&self) -> Self::Extracted {
        T::to_aligned(
            &unsafe { &*self.bits.to_const() }.mem[get_byte_range(self.offset, T::BITS)],
            self.offset % 8,
        )
    }

    fn insert(&self, aligned: T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        T::from_aligned(
            &aligned,
            &mut unsafe { &mut *self.bits.assert_mut().to_mut() }.mem
                [get_byte_range(self.offset, T::BITS)],
            self.offset % 8,
        )
    }

    fn map(&self, mut f: impl FnMut(T) -> T) -> Self::InsertResult
    where
        (M, Mut): InferEq,
    {
        self.insert(f(self.extract()))
    }
}
