#![cfg(test)]

use ds_rs::bigint;
use ds_rs::bigint::BigInt;

#[test]
fn test_add() {
    let big = bigint!(9_800_999);
    let mid = bigint!(230_420);
    let small = bigint!(10_023);

    let nbig = bigint!(-9_800_999);
    let nmid = bigint!(-230_420);
    let nsmall = bigint!(-10_023);

    assert_eq!(&big + &big, bigint!(19601998));
    assert_eq!(&big + &nbig, bigint!(0));
    assert_eq!(&big + &mid, bigint!(10031419));
    assert_eq!(&big + &nmid, bigint!(9570579));
    assert_eq!(&small + &nbig, bigint!(-9790976));
    assert_eq!(&small + &nmid, bigint!(-220397));
    assert_eq!(&small + &nsmall, bigint!(0));
}

#[test]
fn test_sub() {
    let big = bigint!(9_800_999);
    let mid = bigint!(230_420);
    let small = bigint!(10_023);

    let nbig = bigint!(-9_800_999);
    let nmid = bigint!(-230_420);
    let nsmall = bigint!(-10_023);

    assert_eq!(&big - &big, bigint!(0));
    assert_eq!(&big - &nbig, bigint!(19601998));
    assert_eq!(&big - &mid, bigint!(9570579));
    assert_eq!(&big - &nmid, bigint!(10031419));
    assert_eq!(&small - &nbig, bigint!(9811022));
    assert_eq!(&small - &nmid, bigint!(240443));
    assert_eq!(&small - &nsmall, bigint!(20046));
}

#[test]
fn test_mul() {
    let big = bigint!(9_800_999);
    let mid = bigint!(230_420);
    let small = bigint!(10_023);

    let nbig = bigint!(-9_800_999);
    let nmid = bigint!(-230_420);
    let nsmall = bigint!(-10_023);

    assert_eq!(&big * &big, bigint!(96_059_581_398_001));
    assert_eq!(&big * &nbig, bigint!(-96_059_581_398_001));
    assert_eq!(&big * &mid, bigint!(2_258_346_189_580));
    assert_eq!(&big * &nmid, bigint!(-2_258_346_189_580));
    assert_eq!(&small * &nbig, bigint!(-98_235_412_977));
    assert_eq!(&small * &nmid, bigint!(-2_309_499_660));
    assert_eq!(&small * &nsmall, bigint!(-100_460_529));
}
