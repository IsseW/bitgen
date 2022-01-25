#![crate_name = "bitgen"]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(const_for)]
#![feature(const_fn_trait_bound)]
#![feature(associated_type_defaults)]

mod bit_num;

mod bit_type;
mod bit_wrapper;
mod containers;
mod magic;

pub use crate::bit_num::{ibits, ubits, I, U};

#[cfg(feature = "derive")]
pub use bitgen_derive::BitType;

pub use crate::bit_type::BitType;
pub use crate::bit_wrapper::{
    accessors::DynAccess, accessors::MaybeAccess, accessors::TupleAccess, Accessor,
};
pub use containers::*;

/// Constant hash function for string
pub const fn hash_ident(ident: &str) -> usize {
    const_fnv1a_hash::fnv1a_hash_str_64(ident) as usize
}
pub mod internal {
    pub use crate::bit_wrapper::get_byte_range;
}

/// Only used internally for the bit! macro.
#[macro_export]
macro_rules! bit_tail {
    ($expr:expr; [$elem:literal] $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; [$elem:expr] $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get_dyn($expr, $elem); $($tail)*)
    };

    ($expr:expr; .$elem:ident $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get::<{hash_ident(stringify!($elem))}>($expr); $($tail)*)
    };
    ($expr:expr; .$elem:literal $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get::<$elem>($expr); $($tail)*)
    };
    ($expr:expr; .($elem:expr) $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; ?$elem:ident $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get_maybe::<{hash_ident(stringify!($elem))}>($expr); $($tail)*)
    };
    ($expr:expr; ? $($tail:tt)*) => {
        bitgen::bit_tail!(bitgen::Accessor::get_maybe::<{hash_ident("Some")}>($expr); $($tail)*)
    };

    ($expr:expr;) => { $expr }
}

/// Used to access bits in the bit wrapper.
///
/// # Examples
///
/// ```
/// #![allow(incomplete_features)]
//  // Required to use this crate
/// #![feature(generic_const_exprs)]
///
/// use bitgen::*;
/// let tuple = (false, true, (false, true));
/// let bit_tuple = Bit::from(tuple);
/// // Access like you would do with the tuple inside the bit! macro
/// assert_eq!(bit!(bit_tuple).extract(), tuple);
/// assert_eq!(bit!(bit_tuple.0).extract(), false);
/// assert_eq!(bit!(bit_tuple.1).extract(), true);
/// assert_eq!(bit!(bit_tuple.2).extract(), (false, true));
///
/// // Can also access mutably
/// let mut tuple = (false, true, (false, true));
/// let mut bit_tuple = Bit::from(tuple);
/// // Use the mut keyword at the start of the macro for mutable access
/// bit!(mut bit_tuple.0).insert(true);
/// tuple.0 = true;
/// assert_eq!(bit!(bit_tuple).extract(), tuple);
/// ```
/// # Accessors
/// When accessing fields in the bit macro you can use the following:
/// - tuple accessor i.e `.0`, `.1`, `.2`, ...
/// - index accessor i.e `[0]`, `[1]`, `[2]`, ... This also accepts expressions so can be used dynamicly contrary to the tuple accessor
/// - struct accessor i.e `.field`, ...
/// - maybe accessor, used for getting an enum variant. i.e `?Some`, `?None`. This will return an Option<T> when later accessing a field.
///
/// # Casting
/// This essentially a transmute and is therefore unsafe.
/// Used by having a type enclosed in parenthesis at the start of the bit! macro.
/// The type that you're accessing as must have the same amount of bits as the wrapped type.
#[macro_export]
macro_rules! bit {
    (mut $bit_tuple:ident $($tail:tt)*) => {
        bitgen::bit_tail!($bit_tuple.access_mut(); $($tail)*)
    };
    ($bit_tuple:ident $($tail:tt)*) => {
        bitgen::bit_tail!($bit_tuple.access(); $($tail)*)
    };
    (($as:ty) mut $bit_tuple:ident $($tail:tt)*) => {
        bitgen::bit_tail!($bit_tuple.access_as_mut::<$as>(); $($tail)*)
    };
    (($as:ty)$bit_tuple:ident $($tail:tt)*) => {
        bitgen::bit_tail!($bit_tuple.access_as::<$as>(); $($tail)*)
    };
}
