#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(const_for)]
#![feature(const_fn_trait_bound)]
#![feature(associated_type_defaults)]

mod bit_num;

mod bit_type;
mod bit_wrapper;
mod magic;

pub mod prelude {
    pub use crate::{bit, bit_tail};

    pub use crate::bit_num::{ubits, U};

    #[cfg(feature = "derive")]
    pub use bitgen_derive::BitType;

    pub use crate::bit_type::BitType;
    pub use crate::bit_wrapper::{
        accessors::DynAccess, accessors::MaybeAccess, accessors::TupleAccess, Accessor, Bit,
    };
    pub const fn hash_ident(ident: &str) -> usize {
        const_fnv1a_hash::fnv1a_hash_str_64(ident) as usize
    }
    pub mod internal {
        pub use crate::bit_wrapper::get_byte_range;
    }
}

#[macro_export]
macro_rules! bit_tail {
    ($expr:expr; [$elem:literal] $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; [$elem:expr] $($tail:tt)*) => {
        bit_tail!(Accessor::get_dyn($expr, $elem); $($tail)*)
    };

    ($expr:expr; .$elem:ident $($tail:tt)*) => {
        bit_tail!(Accessor::get::<{hash_ident(stringify!($elem))}>($expr); $($tail)*)
    };
    ($expr:expr; .$elem:literal $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };
    ($expr:expr; .($elem:expr) $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; ?$elem:ident $($tail:tt)*) => {
        bit_tail!(Accessor::get_maybe::<{hash_ident(stringify!($elem))}>($expr); $($tail)*)
    };
    ($expr:expr; ? $($tail:tt)*) => {
        bit_tail!(Accessor::get_maybe::<{hash_ident("Some")}>($expr); $($tail)*)
    };

    ($expr:expr;) => { $expr }
}

#[macro_export]
macro_rules! bit {
    (mut $bit_tuple:ident $($tail:tt)*) => {
        bit_tail!($bit_tuple.access_mut(); $($tail)*)
    };
    ($bit_tuple:ident $($tail:tt)*) => {
        bit_tail!($bit_tuple.access(); $($tail)*)
    };
    (($as:ty) mut $bit_tuple:ident $($tail:tt)*) => {
        bit_tail!($bit_tuple.access_as_mut::<$as>(); $($tail)*)
    };
    (($as:ty)$bit_tuple:ident $($tail:tt)*) => {
        bit_tail!($bit_tuple.access_as::<$as>(); $($tail)*)
    };
}
