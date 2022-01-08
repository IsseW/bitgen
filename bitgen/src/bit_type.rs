use std::mem;

use crate::prelude::internal::get_byte_range;

pub trait BitType: Sized + 'static {
    const BITS: usize;

    // Offset should be in the range 0..8
    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize);

    fn to_aligned(slice: &[u8], offset: usize) -> Self;
}

macro_rules! impl_bit_tuple {
    ($($ty:literal), *) => {
        paste::paste! {
            impl<$([< T $ty >]: BitType), *> BitType for ($([< T $ty >],) *)
            where
                [u8; mem::size_of::<Self>()]: Sized,
                $([u8; mem::size_of::<[< T $ty >]>()]: Sized), * {
                const BITS: usize = 0 $(+ [< T $ty >]::BITS) *;

                #[allow(unused_assignments)]
                fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                    $(
                        [<T $ty>]::from_aligned(&aligned.$ty, &mut slice[get_byte_range(offset, [<T $ty>]::BITS)], offset % 8);
                        offset += [<T $ty>]::BITS;
                    )*
                }

                #[allow(unused_assignments)]
                fn to_aligned(slice: &[u8], mut offset: usize) -> Self {
                    (
                        $(
                            {
                                let res = [<T $ty>]::to_aligned(&slice[get_byte_range(offset, [<T $ty>]::BITS)], offset % 8);
                                offset += [<T $ty>]::BITS;
                                res
                            },
                        )*
                    )
                }
            }
        }
    };
}

impl_bit_tuple!(0);
impl_bit_tuple!(0, 1);
impl_bit_tuple!(0, 1, 2);
impl_bit_tuple!(0, 1, 2, 3);
impl_bit_tuple!(0, 1, 2, 3, 4);
impl_bit_tuple!(0, 1, 2, 3, 4, 5);
impl_bit_tuple!(0, 1, 2, 3, 4, 5, 6);
impl_bit_tuple!(0, 1, 2, 3, 4, 5, 6, 7);

impl<T: BitType, const N: usize> BitType for [T; N]
where
    [u8; mem::size_of::<T>()]: Sized,
    [u8; mem::size_of::<Self>()]: Sized,
{
    const BITS: usize = T::BITS * N;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        for i in 0..N {
            T::from_aligned(
                &aligned[i],
                &mut slice[get_byte_range(offset + i * T::BITS, T::BITS)],
                (offset + i * T::BITS) % 8,
            )
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        let mut result: Self = unsafe { mem::zeroed() };
        for i in 0..N {
            result[i] = T::to_aligned(
                &slice[get_byte_range(offset + i * T::BITS, T::BITS)],
                (offset + i * T::BITS) % 8,
            );
        }
        result
    }
}

impl BitType for () {
    const BITS: usize = 0;

    fn from_aligned(_: &Self, _: &mut [u8], _: usize) {}

    fn to_aligned(_: &[u8], _: usize) -> Self {}
}

impl BitType for bool {
    const BITS: usize = 1;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        let byte = u8::MAX & (*aligned as u8) << offset;
        let bits = !(1 << offset);
        slice[0] &= bits;
        slice[0] |= byte;
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        let byte = slice[0] & (1 << offset);
        byte != 0
    }
}

impl BitType for u8 {
    const BITS: usize = 8;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        if slice.len() == 1 {
            slice[0] = *aligned;
        } else {
            let byte = u16::MAX & (*aligned as u16) << offset;
            let bits = !(u16::MAX << offset);
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u16, usize)>(slice).0 };
            *slice_ref &= bits;
            *slice_ref |= byte;
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        if slice.len() == 1 {
            slice[0]
        } else {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u16, usize)>(slice).0 };
            let num = slice_ref.clone();
            ((num >> offset) & (u8::MAX as u16)) as u8
        }
    }
}
impl BitType for u16 {
    const BITS: usize = 16;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u16, usize)>(slice).0 };
            *slice_ref = *aligned;
        } else {
            let byte = u32::MAX & (*aligned as u32) << offset;
            let bits = !(u32::MAX << offset);
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u32, usize)>(slice).0 };
            *slice_ref &= bits;
            *slice_ref |= byte;
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u16, usize)>(slice).0 };
            *slice_ref
        } else {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u32, usize)>(slice).0 };
            let num = slice_ref.clone();
            ((num >> offset) & (u16::MAX as u32)) as u16
        }
    }
}
impl BitType for u32 {
    const BITS: usize = 32;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u32, usize)>(slice).0 };
            *slice_ref = *aligned;
        } else {
            let byte = u64::MAX & (*aligned as u64) << offset;
            let bits = !(u64::MAX << offset);
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u64, usize)>(slice).0 };
            *slice_ref &= bits;
            *slice_ref |= byte;
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u32, usize)>(slice).0 };
            *slice_ref
        } else {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u64, usize)>(slice).0 };
            let num = slice_ref.clone();
            ((num >> offset) & (u32::MAX as u64)) as u32
        }
    }
}
impl BitType for u64 {
    const BITS: usize = 64;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u64, usize)>(slice).0 };
            *slice_ref = *aligned;
        } else {
            let byte = u128::MAX & (*aligned as u128) << offset;
            let bits = !(u128::MAX << offset);
            let slice_ref = unsafe { mem::transmute::<&mut [u8], (&mut u128, usize)>(slice).0 };
            *slice_ref &= bits;
            *slice_ref |= byte;
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        if slice.len() == 1 {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u64, usize)>(slice).0 };
            *slice_ref
        } else {
            let slice_ref = unsafe { mem::transmute::<&[u8], (&u128, usize)>(slice).0 };
            let num = slice_ref.clone();
            ((num >> offset) & (u64::MAX as u128)) as u64
        }
    }
}

impl<T: BitType> BitType for Option<T> {
    const BITS: usize = 1 + T::BITS;

    fn from_aligned(aligned: &Self, slice: &mut [u8], offset: usize) {
        if let Some(value) = aligned {
            slice[0] |= 1 << offset;
            T::from_aligned(value, &mut slice[(offset + 1) / 8..], (offset + 1) % 8);
        } else {
            slice[0] &= !(1 << offset);
        }
    }

    fn to_aligned(slice: &[u8], offset: usize) -> Self {
        if ((slice[0] >> offset) & 1) == 0 {
            None
        } else {
            Some(T::to_aligned(&slice[(offset + 1) / 8..], (offset + 1) % 8))
        }
    }
}
