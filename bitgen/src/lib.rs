#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_associated_types)]
#![feature(const_for)]
#![feature(const_fn_trait_bound)]
#![feature(test)]

mod bit_num;
mod bit_type;
mod bit_wrapper;
mod magic;
extern crate test;

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

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use test::Bencher;
    #[test]
    fn test_size() {
        // 4 bits so should use 1 byte
        let tuple = (false, true, false, true);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(std::mem::size_of_val(&bit_tuple), 1);

        // 16 bits so should use 2 bytes
        let tuple = (false, ubits::<14>(1337), true);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(std::mem::size_of_val(&bit_tuple), 2);

        // 25 bits so should use 4 bytes
        let tuple = (false, [true; 23], true);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(std::mem::size_of_val(&bit_tuple), 4);
    }

    #[test]
    fn test_persistence() {
        let tuple = (false, true, false, true);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(tuple, bit!(bit_tuple).extract());

        let tuple = (true, ubits::<2>(3), true, true, true);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(tuple, bit!(bit_tuple).extract());

        let tuple = (
            false,
            ubits::<14>(10),
            false,
            [true, false, true, true, false],
        );
        let bit_tuple = Bit::from(tuple);
        assert_eq!(tuple, bit!(bit_tuple).extract());
    }

    #[test]
    fn test_access() {
        let tuple = (false, true, false, true);
        let bit_tuple = Bit::from(tuple);

        assert_eq!(bit!(bit_tuple(1)).extract(), true);

        let tuple = (false, true, false, [true, true, true, false, true, true]);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(bit!(bit_tuple(3)[3]).extract(), false);
    }

    #[test]
    fn test_mutability_access() {
        let tuple = (false, true, false, true);
        let mut bit_tuple = Bit::from(tuple);

        bit!(mut bit_tuple(0)).insert(true);
        bit!(mut bit_tuple(2)).insert(true);

        assert_eq!(bit!(bit_tuple).extract(), (true, true, true, true));

        let tuple = (false, true, false, [false; 61]);
        let mut bit_tuple = Bit::from(tuple);
        bit!(mut bit_tuple(3)[32]).insert(true);
        assert_eq!(bit!(bit_tuple(3)[32]).extract(), true);
    }

    #[test]
    fn test_extreme_size() {
        let arr = [false; 1024 * 128];
        let mut bit_arr = Bit::from(arr);

        bit!(mut bit_arr[69420]).insert(true);

        assert_eq!(bit!(bit_arr[69420]).extract(), true);
    }

    #[bench]
    fn bench_access(b: &mut Bencher) {
        let tuple = ([false; 22], [true; 22], [false; 22], [true; 22]);
        let bit_tuple = Bit::from(tuple);

        b.iter(|| {
            bit!(bit_tuple).extract();
        });
    }

    #[bench]
    fn bench_access_mut(b: &mut Bencher) {
        let tuple = ([false; 22], [true; 22], [false; 22], [true; 22]);
        let mut bit_tuple = Bit::from(tuple);

        b.iter(|| {
            bit!(mut bit_tuple).insert(([true; 22], [false; 22], [true; 22], [false; 22]));
        });
    }

    #[bench]
    fn bench_access_remap(b: &mut Bencher) {
        let tuple = ([false; 22], [true; 22], [false; 22], [true; 22]);
        let mut bit_tuple = Bit::from(tuple);
        b.iter(|| {
            bit!(mut bit_tuple).map(|x| x);
        });
    }
}