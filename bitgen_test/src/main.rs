#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use bitgen::prelude::*;
use bitgen_derive::BitType;

#[derive(BitType, Debug, PartialEq, Clone)]
enum Test {
    A,
    B,
    C(bool, bool),
    D,
    E(bool, bool),
    E0 {},
    E1(),
}

#[derive(BitType, Debug, PartialEq, Clone)]
struct Haha {
    a: bool,
    b: Test,
    c: bool,
}

fn main() {
    let unbit = Haha {
        a: true,
        b: Test::C(true, false),
        c: false,
    };
    dbg!(std::mem::size_of_val(&unbit)); // = 5
    let bit = Bit::from(unbit.clone());
    dbg!(std::mem::size_of_val(&bit)); // = 1
    assert_eq!(bit!(bit).extract(), unbit); // ok
}
