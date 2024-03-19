#![cfg(test)]

use std::env;

use ds_rs::bigint;
// use ds_rs::bigint::div_by_three;
// use ds_rs::bigint::div_by_two;
use ds_rs::bigint::BigInt;
use pretty_assertions::assert_eq;
use rand::Rng;

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
    env::set_var("RUST_BACKTRACE", "1");
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

#[test]
fn test_fact() {
    assert_eq!(bigint!(0).fact(), bigint!(1));
    assert_eq!(bigint!(4).fact(), bigint!(24));
    assert_eq!(bigint!(20).fact(), bigint!(2432902008176640000));
    assert_eq!(bigint!(100).fact(), bigint!(93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000))
}

#[test]
fn test_try_into() {
    assert_eq!(
        TryInto::<isize>::try_into(bigint!(123456789)).unwrap(),
        123456789isize
    );
    assert_eq!(
        TryInto::<isize>::try_into(bigint!(-123456789)).unwrap(),
        -123456789isize
    );
}

#[test]
#[should_panic]
fn test_try_into_panic() {
    let _ = TryInto::<isize>::try_into(bigint!(123456789123456789123456789)).unwrap();
}

// #[test]
// fn test_div_by_two_random() {
//     let mut rng = rand::thread_rng();
//     for _ in 0..1000 {
//         let x: usize = rng.gen();
//         let e = x / 2;
//         let be = BigInt::from(e);
//         let expected = be.digits();
//         let b = BigInt::from(x);
//         let b = b.digits();
//         let res = div_by_two(b);
//         assert_eq!(res, expected.to_owned());
//     }
// }

// #[test]
// fn test_div_by_three_random() {
//     let mut rng = rand::thread_rng();
//     for _ in 0..1000 {
//         let x: usize = rng.gen();
//         let e = x / 3;
//         let be = BigInt::from(e);
//         let expected = be.digits();
//         let b = BigInt::from(x);
//         let b = b.digits();
//         let res = div_by_three(b);
//         assert_eq!(res, expected.to_owned());
//     }
// }

#[test]
fn test_mul_random() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x: i32 = rng.gen();
        let y: i32 = rng.gen();
        let e = x as i128 * y as i128;
        let e = BigInt::from(e);
        let x = BigInt::from(x as i128);
        let y = BigInt::from(y as i128);
        let res = x * y;
        assert_eq!(res, e);
    }
}

#[test]
fn test_add_random() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x: i32 = rng.gen();
        let y: i32 = rng.gen();
        let e = x as i128 + y as i128;
        let e = BigInt::from(e);
        let x = BigInt::from(x);
        let y = BigInt::from(y);
        let res = x + y;
        assert_eq!(res, e);
    }
}

#[test]
fn test_sub_random() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x: i32 = rng.gen();
        let y: i32 = rng.gen();
        let e = x as i128 - y as i128;
        let e = BigInt::from(e);
        let x = BigInt::from(x);
        let y = BigInt::from(y);
        let res = x - y;
        assert_eq!(res, e);
    }
}

#[test]
fn test_to_string() {
    let x = bigint!(93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000);

    let e = "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000".to_string();

    assert_eq!(x.to_string(), e);
}

#[test]
fn test_big_const() {
    let x = bigint!(3541684060098168196989198191000000000989181894981000000000000089649481000000054699999999999999999999999900000035416161200000000000000000000000);
    let y = bigint!(6874987465132030656161861320320661811203020299919909000000000000019919199199412491249029409090949099090900000000000000000000004909409490940949094090000000000000000000000000000099999999999999999999999999999999999999940490490240000000000000000000000000000000000000000);

    let mul = &x * &y;
    let add = &x + &y;
    let sub = &x - &y;

    let mul_e = "24349033518632823894135154533098628543238993281598648025932643066935739961545462551892500582936697736673341989210952246192679844089045094769120419529079036384042995251978680573534538619308125502461919135653186042318555194142681751142182383450929229721379401555773520885485439994655003338993178304829816128000000000000005950948868401609406866688000000000000000000000000000000000000000000000000000000000000000";
    let add_e = "6874987465132030656161861320320661811203020299919909000000000000019919199199412491249029409090949099090900000000000000000003546593469589109146083288191000000000989181894981000100000000089649481000000054699999999999940490490239900000035416161200000000000000000000000";
    let sub_e = "-6874987465132030656161861320320661811203020299919909000000000000019919199199412491249029409090949099090899999999999999999996463225349392772752104891808999999999010818105019000099999999910350518999999945299999999999940490490240099999964583838800000000000000000000000";

    assert_eq!(mul.to_string(), mul_e);
    assert_eq!(mul, BigInt::from(mul_e));

    assert_eq!(add.to_string(), add_e);
    assert_eq!(add, BigInt::from(add_e));

    assert_eq!(sub.to_string(), sub_e);
    assert_eq!(sub, BigInt::from(sub_e));
}

/// t1: len type
/// t2: element type
#[macro_export]
macro_rules! random_vec {
    ($t1:ty, $t2:ty) => {{
        type T = $t2;
        let mut rng = rand::thread_rng();
        let len: $t1 = rng.gen();
        let mut v: Vec<T> = vec![T::default(); len as usize];
        for i in v.iter_mut() {
            *i = rng.gen();
        }
        v
    }};
}
