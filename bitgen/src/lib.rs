#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(const_for)]
#![feature(const_fn_trait_bound)]

mod bit_num;
mod bit_type;
mod bit_wrapper;
mod magic;

pub mod prelude {
    pub use crate::{bit, bit_tail};

    pub use crate::bit_num::{ubits, U};
    pub use crate::bit_type::BitType;
    pub use crate::bit_wrapper::{Accessor, Bit};
}

#[macro_export]
macro_rules! bit_tail {
    ($expr:expr; ($elem:literal) $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };
    ($expr:expr; ($elem:expr) $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; [$elem:literal] $($tail:tt)*) => {
        bit_tail!(Accessor::get::<$elem>($expr); $($tail)*)
    };

    ($expr:expr; [$elem:expr] $($tail:tt)*) => {
        bit_tail!(Accessor::get_dyn($expr, $elem); $($tail)*)
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
}
