#![allow(incomplete_features)]
// Required to use this crate
#![feature(generic_const_exprs)]
#![feature(test)]

extern crate test;
#[cfg(test)]
mod tests {
    use bitgen::*;
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

        assert_eq!(bit!(bit_tuple.0).extract(), false);
        assert_eq!(bit!(bit_tuple.1).extract(), true);
        assert_eq!(bit!(bit_tuple.2).extract(), false);
        assert_eq!(bit!(bit_tuple.3).extract(), true);

        let tuple = (false, true, false, [true, true, true, false, true, true]);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(bit!(bit_tuple.0).extract(), false);
        assert_eq!(bit!(bit_tuple.1).extract(), true);
        assert_eq!(bit!(bit_tuple.2).extract(), false);
        for i in 0..6 {
            assert_eq!(bit!(bit_tuple.3[i]).extract(), i != 3);
        }
    }

    #[test]
    fn test_mutability_access() {
        let tuple = (false, true, false, true);
        let mut bit_tuple = Bit::from(tuple);

        assert_eq!(bit!(bit_tuple).extract(), tuple);

        bit!(mut bit_tuple.0).insert(true);
        bit!(mut bit_tuple.2).insert(true);

        assert_eq!(bit!(bit_tuple).extract(), (true, true, true, true));

        let mut tuple = (false, true, false, [false; 61]);
        let mut bit_tuple = Bit::from(tuple);

        tuple.3[32] = true;
        bit!(mut bit_tuple.3[32]).insert(true);

        assert_eq!(bit!(bit_tuple.3[32]).extract(), true);

        assert_eq!(bit!(bit_tuple).extract(), tuple);
    }

    #[test]
    fn test_extreme_size() {
        let arr = [false; 1024 * 128];
        let mut bit_arr = Bit::from(arr);

        bit!(mut bit_arr[69420]).insert(true);

        assert_eq!(bit!(bit_arr[69420]).extract(), true);
    }

    #[test]
    fn test_derived_types() {
        #[derive(BitType, PartialEq, Debug, Clone, Copy)]
        struct Test0 {
            a: bool,
            b: U<3>,
            c: bool,
            d: (U<4>, bool),
        }
        let test = Test0 {
            a: false,
            b: ubits(5),
            c: true,
            d: (ubits(15), true),
        };
        let bit_test = Bit::from(test.clone());
        assert_eq!(test, bit!(bit_test).extract());

        assert_eq!(test.a, bit!(bit_test.a).extract());
        assert_eq!(test.b, bit!(bit_test.b).extract());
        assert_eq!(test.c, bit!(bit_test.c).extract());
        assert_eq!(test.d, bit!(bit_test.d).extract());
        assert_eq!(test.d.0, bit!(bit_test.d.0).extract());
        assert_eq!(test.d.1, bit!(bit_test.d.1).extract());

        #[derive(BitType, PartialEq, Debug, Clone, Copy)]
        struct Test1(bool, U<7>, bool, (U<2>, bool));

        let test = Test1(false, ubits(69), true, (ubits(1), true));
        let bit_test = Bit::from(test.clone());
        assert_eq!(test, bit!(bit_test).extract());

        assert_eq!(test.0, bit!(bit_test.0).extract());
        assert_eq!(test.1, bit!(bit_test.1).extract());
        assert_eq!(test.2, bit!(bit_test.2).extract());
        assert_eq!(test.3, bit!(bit_test.3).extract());
        assert_eq!(test.3 .0, bit!(bit_test.3 .0).extract());
        assert_eq!(test.3 .1, bit!(bit_test.3 .1).extract());

        #[derive(BitType, PartialEq, Debug, Clone, Copy)]
        enum Test2 {
            A,
            B(U<3>, bool, bool),
            C { a: bool, b: U<4> },
            D(Test0, Test1),
        }

        let test = (
            Test2::A,
            Test2::B(ubits(1), true, true),
            Test2::C {
                a: false,
                b: ubits::<4>(5),
            },
            Test2::D(
                Test0 {
                    a: true,
                    b: ubits(6),
                    c: true,
                    d: (ubits(10), false),
                },
                Test1(true, ubits(42), false, (ubits(2), true)),
            ),
        );
        let bit_test = Bit::from(test.clone());
        assert_eq!(test, bit!(bit_test).extract());
        assert_eq!(test.0, bit!(bit_test.0).extract());
        assert_eq!(test.1, bit!(bit_test.1).extract());
        assert_eq!(test.2, bit!(bit_test.2).extract());
        assert_eq!(test.3, bit!(bit_test.3).extract());

        assert!(bit!(bit_test.0?B).extract().is_none());
        assert_eq!(Some(()), bit!(bit_test.0?A).extract());

        assert_eq!(Some(ubits(1)), bit!(bit_test.1?B.0).extract());
        assert_eq!(Some(true), bit!(bit_test.1?B.1).extract());
        assert_eq!(Some(true), bit!(bit_test.1?B.2).extract());

        assert_eq!(Some(false), bit!(bit_test.2?C.a).extract());
        assert_eq!(Some(ubits(5)), bit!(bit_test.2?C.b).extract());

        assert_eq!(
            Some(Test0 {
                a: true,
                b: ubits(6),
                c: true,
                d: (ubits(10), false),
            },),
            bit!(bit_test.3?D.0).extract()
        );
        assert_eq!(Some(true), bit!(bit_test.3?D.0.a).extract());
        assert_eq!(Some(ubits(6)), bit!(bit_test.3?D.0.b).extract());
        assert_eq!(Some(true), bit!(bit_test.3?D.0.c).extract());
        assert_eq!(Some((ubits(10), false)), bit!(bit_test.3?D.0.d).extract());
        assert_eq!(Some(ubits(10)), bit!(bit_test.3?D.0.d.0).extract());
        assert_eq!(Some(false), bit!(bit_test.3?D.0.d.1).extract());

        assert_eq!(
            Some(Test1(true, ubits(42), false, (ubits(2), true))),
            bit!(bit_test.3?D.1).extract()
        );
        assert_eq!(Some(true), bit!(bit_test.3?D.1 .0).extract());
        assert_eq!(Some(ubits(42)), bit!(bit_test.3?D.1 .1).extract());
        assert_eq!(Some(false), bit!(bit_test.3?D.1 .2).extract());
        assert_eq!(Some((ubits(2), true)), bit!(bit_test.3?D.1 .3).extract());
        assert_eq!(Some(ubits(2)), bit!(bit_test.3?D.1 .3 .0).extract());
        assert_eq!(Some(true), bit!(bit_test.3?D.1 .3 .1).extract());

        #[derive(BitType, PartialEq, Debug, Clone, Copy)]
        struct Test3;

        let test = Test3;
        let bit_test = Bit::from(test.clone());
        assert_eq!(test, bit!(bit_test).extract());

        #[derive(BitType, PartialEq, Debug, Clone)]
        struct BigStruct {
            a: [Test0; 99],
            b: [Test1; 99],
            c: [Test2; 99],
            d: [Test3; 99],
        }
        let aligned_size = (5 + 5 + 11) * 99;
        let unaligned_size = ((10 + 12 + (2 + 10 + 12)) * 99 - 1) / 8 + 1;

        let test = BigStruct {
            a: [Test0 {
                a: true,
                b: ubits(6),
                c: true,
                d: (ubits(10), false),
            }; 99],
            b: [Test1(true, ubits(42), false, (ubits(2), true)); 99],
            c: [Test2::B(ubits(3), false, true); 99],
            d: [Test3; 99],
        };
        let bit_test = Bit::from(test.clone());

        assert_eq!(std::mem::size_of_val(&test), aligned_size);
        assert_eq!(std::mem::size_of_val(&bit_test), unaligned_size);

        assert_eq!(test, bit!(bit_test).extract());

        #[derive(BitType, PartialEq, Debug, Clone)]
        enum Test4 {
            A(u8),
        }

        let test = Test4::A(42);
        let bit_test = Bit::from(test.clone());
        assert_eq!(test, bit!(bit_test).extract());
        assert_eq!(Some(42), bit!(bit_test?A.0).extract());
    }

    #[test]
    fn test_iterator() {
        let mut arr = [false; 32];
        for i in (0..32).step_by(3) {
            arr[i] = true;
        }

        let mut bit_arr = Bit::from(arr);
        assert_eq!(arr, bit!(bit_arr).extract());

        for (i, bit) in bit!(bit_arr).iter().enumerate() {
            assert_eq!(i % 3 == 0, bit.extract());
        }
        assert_eq!(arr, bit!(bit_arr).extract());

        for bit in bit!(mut bit_arr).iter() {
            bit.insert(true);
        }
        assert_eq!([true; 32], bit!(bit_arr).extract());
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
