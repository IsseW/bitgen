#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(test)]
use bitgen::prelude::*;

#[derive(BitType, Debug, PartialEq, Clone)]
enum Haha {
    Hi,
    Ha,
    Ho,
    He,
    Hu,
    Hy,
}

#[derive(BitType, Debug, PartialEq, Clone)]
enum Test {
    A,
    B,
    C(bool, bool),
    D,
    E0(bool, Haha, Option<[u8; 4]>),
    E1,
}

#[derive(BitType, Debug, PartialEq, Clone)]
struct T2 {}

fn main() {
    let test = (
        true,
        [
            Test::A,
            Test::C(true, true),
            Test::E0(false, Haha::Hi, Some(0xCAE15694u32.to_le_bytes())),
        ],
        false,
        true,
        false,
    );
    let bits = Bit::from(test);
    for i in 0..4 {
        dbg!(bit!(bits.1[2]?E0.2?Some[i]).extract());
    }
}
extern crate test;
#[cfg(test)]
mod tests {
    use bitgen::prelude::*;
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

        assert_eq!(bit!(bit_tuple.1).extract(), true);

        let tuple = (false, true, false, [true, true, true, false, true, true]);
        let bit_tuple = Bit::from(tuple);
        assert_eq!(bit!(bit_tuple.3[3]).extract(), false);
    }

    #[test]
    fn test_mutability_access() {
        let tuple = (false, true, false, true);
        let mut bit_tuple = Bit::from(tuple);

        bit!(mut bit_tuple.0).insert(true);
        bit!(mut bit_tuple.2).insert(true);

        assert_eq!(bit!(bit_tuple).extract(), (true, true, true, true));

        let tuple = (false, true, false, [false; 61]);
        let mut bit_tuple = Bit::from(tuple);
        bit!(mut bit_tuple.3[32]).insert(true);
        assert_eq!(bit!(bit_tuple.3[32]).extract(), true);
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
