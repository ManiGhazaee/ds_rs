use core::num::ParseIntError;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, MulAssign, Shl, Shr, Sub},
};

type Digit = u64;
type DoubleDigit = u128;
const BASE: Digit = 1000000000000000000;
const BASE_POW: usize = 18;

#[derive(Debug)]
pub struct BigInt {
    digits: Vec<Digit>,
    positive: bool,
}

impl BigInt {
    pub fn new(vec: Vec<Digit>, positive: bool) -> Self {
        let mut vec = if vec.is_empty() { vec![0] } else { vec };
        trim_end_zeros(&mut vec);
        BigInt {
            digits: vec,
            positive,
        }
    }
}

impl BigInt {
    pub fn zero() -> Self {
        Self {
            digits: vec![0],
            positive: true,
        }
    }

    pub fn one() -> Self {
        Self {
            digits: vec![1],
            positive: true,
        }
    }

    pub fn neg_one() -> Self {
        Self {
            digits: vec![1],
            positive: false,
        }
    }

    pub fn two() -> Self {
        Self {
            digits: vec![2],
            positive: true,
        }
    }

    pub fn three() -> Self {
        Self {
            digits: vec![3],
            positive: true,
        }
    }

    pub fn pow(self, exp: usize) -> Self {
        let mut res = BigInt::one();
        for _ in 0..exp {
            res *= self.clone();
        }

        res
    }

    pub fn fact(self) -> Self {
        assert!(self >= Self::zero());
        if self == Self::zero() {
            return Self::one();
        }
        let mut i = Self::one();
        let mut res = Self::one();
        loop {
            res *= &i;
            if i == self {
                break res;
            }
            i += Self::one();
        }
    }

    pub fn to_usize(&self) -> Result<usize, ParseIntError> {
        TryInto::<usize>::try_into(self)
    }

    pub fn get_digit(&self, index: usize) -> Option<&Digit> {
        self.digits.get(index)
    }

    pub fn get_digit_mut(&mut self, index: usize) -> Option<&mut Digit> {
        self.digits.get_mut(index)
    }

    pub fn digits(&self) -> &Vec<Digit> {
        self.digits.as_ref()
    }

    // pub(super) fn div_by_three(&self) -> Self {
    //     BigInt::new(div_by_three(&self.digits), self.positive)
    // }

    // pub(super) fn div_by_two(&self) -> Self {
    //     BigInt::new(div_by_two(&self.digits), self.positive)
    // }
}

fn _cmp(lhs: &[Digit], rhs: &[Digit]) -> Ordering {
    let lcmp = _len_cmp(lhs, rhs);
    let Ordering::Equal = lcmp else {
        return lcmp;
    };

    for i in lhs.iter().zip(rhs.iter()).rev() {
        if i.0 > i.1 {
            return Ordering::Greater;
        } else if i.0 < i.1 {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn _len_cmp(lhs: &[Digit], rhs: &[Digit]) -> Ordering {
    if lhs.len() > rhs.len() {
        Ordering::Greater
    } else if lhs.len() < rhs.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn add(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    match (lhs, rhs) {
        (&[], &[]) => vec![0],
        (&[0], &[0]) => vec![0],
        (&[], _) => rhs.to_vec(),
        (_, &[]) => lhs.to_vec(),
        (x, y) => _add(x, y),
    }
}

fn _add(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    let mut carry = 0;
    let mut i = 0;
    let (min, max, max_ref) = if lhs.len() > rhs.len() {
        (rhs.len(), lhs.len(), lhs)
    } else {
        (lhs.len(), rhs.len(), rhs)
    };

    let mut res: Vec<Digit> = Vec::with_capacity(max + 1);

    while i < min {
        let a = unsafe { *lhs.get_unchecked(i) + *rhs.get_unchecked(i) + carry };
        res.push(a % BASE);
        carry = a / BASE;
        i += 1;
    }

    while i < max {
        let a = unsafe { *max_ref.get_unchecked(i) + carry };
        res.push(a % BASE);
        carry = a / BASE;
        i += 1;
    }

    if carry != 0 {
        res.push(carry);
    }

    res
}

fn sub(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    match (lhs, rhs) {
        (&[], &[]) => vec![0],
        (&[0], &[0]) => vec![0],
        (&[], _) => rhs.to_vec(),
        (_, &[]) => lhs.to_vec(),
        (x, y) => _sub(x, y),
    }
}

fn _sub(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    let min = rhs.len();
    let max = lhs.len();
    let mut res: Vec<Digit> = Vec::with_capacity(max);

    lhs[min..max].iter().rev().for_each(|i| res.push(*i));

    let mut i = min - 1;
    loop {
        let l_i = unsafe { *lhs.get_unchecked(i) };
        let r_i = unsafe { *rhs.get_unchecked(i) };
        if l_i < r_i {
            res.push(BASE + l_i - r_i);
            let mut j = res.len() - 2;
            loop {
                unsafe {
                    if *res.get_unchecked(j) == 0 {
                        *res.get_unchecked_mut(j) = BASE - 1;
                    } else {
                        *res.get_unchecked_mut(j) -= 1;
                        break;
                    }
                }
                j -= 1;
            }
        } else {
            res.push(l_i - r_i);
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }

    res.reverse();
    trim_end_zeros(&mut res);

    res
}

fn trim_end_zeros(slice: &mut Vec<Digit>) {
    let mut i = slice.len() - 1;
    if unsafe { *slice.get_unchecked(i) != 0 } {
        return;
    }
    loop {
        unsafe {
            if *slice.get_unchecked(i) != 0 {
                slice.truncate(i + 1);
                return;
            }
        }
        if i == 0 {
            slice.truncate(1);
            return;
        }
        i -= 1;
    }
}

// fn xx_mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
//     let (min, max, min_ref, max_ref) = if lhs.len() > rhs.len() {
//         (rhs.len(), lhs.len(), rhs, lhs)
//     } else {
//         (lhs.len(), rhs.len(), lhs, rhs)
//     };

//     let mut carry = 0;
//     let mut temp: Vec<u8> = Vec::with_capacity(max);
//     let mut res: Vec<u8> = Vec::with_capacity(max);

//     let mut i = 0;
//     while i < min {
//         (0..i).for_each(|_| temp.push(0));

//         let mut j = 0;
//         while j < max {
//             let a = unsafe { max_ref.get_unchecked(j) * min_ref.get_unchecked(i) + carry };
//             if a > 9 {
//                 temp.push(a % 10);
//                 carry = a / 10;
//             } else {
//                 temp.push(a);
//                 carry = 0;
//             }
//             j += 1;
//         }
//         if carry != 0 {
//             temp.push(carry);
//             carry = 0;
//         }
//         res = _add(&res, &temp);
//         temp.clear();

//         i += 1;
//     }

//     trim_end_zeros(&mut res);

//     res
// }

// fn x_mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
//     let (min, max, min_ref, max_ref) = if lhs.len() > rhs.len() {
//         (rhs.len(), lhs.len(), rhs, lhs)
//     } else {
//         (lhs.len(), rhs.len(), lhs, rhs)
//     };

//     let mut res: Vec<u8> = vec![0; max + min];

//     for i in 0..min {
//         let mut carry = 0;
//         let mut j = 0;
//         while j < max {
//             let a = unsafe { max_ref.get_unchecked(j) * min_ref.get_unchecked(i) + carry };
//             let sum = unsafe { *res.get_unchecked(i + j) } + a;
//             unsafe { *res.get_unchecked_mut(i + j) = sum % 10 };
//             carry = sum / 10;
//             j += 1;
//         }
//         let mut k = i + j;
//         while carry != 0 {
//             let sum = unsafe { *res.get_unchecked(k) } + carry;
//             unsafe { *res.get_unchecked_mut(k) = sum % 10 };
//             carry = sum / 10;
//             k += 1;
//         }
//     }

//     trim_end_zeros(&mut res);

//     res
// }

// fn y_mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
//     let (min_ref, max_ref) = if lhs.len() > rhs.len() {
//         (rhs, lhs)
//     } else {
//         (lhs, rhs)
//     };

//     let mut res = vec![0; lhs.len() + rhs.len()];

//     for (i, &digit_i) in min_ref.iter().enumerate() {
//         let mut carry = 0;
//         for (j, &digit_j) in max_ref.iter().enumerate() {
//             let index = i + j;
//             let product = unsafe { *res.get_unchecked(index) } + digit_i * digit_j + carry;
//             unsafe { *res.get_unchecked_mut(index) = product % 10 };
//             carry = product / 10;
//         }
//         let mut k = i + max_ref.len();
//         while carry != 0 {
//             let sum = unsafe { res.get_unchecked(k) } + carry;
//             unsafe { *res.get_unchecked_mut(k) = sum % 10 };
//             carry = sum / 10;
//             k += 1;
//         }
//     }

//     trim_end_zeros(&mut res);

//     res
// }

fn _mul(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    let mut result = vec![0; lhs.len() + rhs.len()];

    for (i, &digit1) in lhs.iter().enumerate() {
        let mut carry = 0;
        for (j, &digit2) in rhs.iter().enumerate() {
            let product = digit1 as DoubleDigit * digit2 as DoubleDigit
                + unsafe { *result.get_unchecked(i + j) } as DoubleDigit
                + carry as DoubleDigit;
            unsafe { *result.get_unchecked_mut(i + j) = (product % BASE as DoubleDigit) as Digit };
            carry = product / BASE as DoubleDigit;
        }
        unsafe { *result.get_unchecked_mut(i + rhs.len()) = carry as Digit };
    }

    trim_end_zeros(&mut result);

    result
}

// fn _mul_t3(lhs: &[u8], rhs: &[u8]) -> BigInt {
//     let max = if let Ordering::Greater = _len_cmp(lhs, rhs) {
//         lhs
//     } else {
//         rhs
//     };

//     let div = max.len() / 3 + 1;

//     let (p0, p1, p2, _p1, _p2) = _t3_eval(lhs, div);
//     let (q0, q1, q2, _q1, _q2) = _t3_eval(rhs, div);

//     let r_0 = &p0 * &q0;
//     let r_1 = &p1 * &q1;
//     let r_2 = &p2 * &q2;
//     let _r_1 = &_p1 * &_q1;
//     let _r_2 = &_p2 * &_q2;

//     let r0 = &r_0;
//     let r4 = &r_2;
//     let r3 = (&_r_2 - &r_1).div_by_three();
//     let r1 = (&r_1 - &_r_1).div_by_two();
//     let r2 = &_r_1 - &r_0;
//     let r3 = (&r2 - &r3).div_by_two() + (&BigInt::two() * &r_2);
//     let r2 = &(&r2 + &r1) - r4;
//     let r1 = &r1 - &r3;

//     let mut res = BigInt::zero();
//     res += r0;
//     res += r1 >> div;
//     res += r2 >> (2 * div);
//     res += r3 >> (3 * div);
//     res += r4 >> (4 * div);

//     trim_end_zeros(&mut res.digits);

//     res
// }

fn mul(lhs: &[Digit], rhs: &[Digit]) -> Vec<Digit> {
    match (lhs, rhs) {
        (&[], _) => return vec![0],
        (_, &[]) => return vec![0],
        (&[0], _) => return vec![0],
        (_, &[0]) => return vec![0],
        (x, y) => {
            // return if cmp::min(x.len(), y.len()) > 32 {
            //     // _mul_t3(x, y).digits
            //     panic!();
            // } else {
            _mul(x, y)
            // };
        }
    }
}

// #[inline]
// fn _t3_eval<'a>(num: &'a [u8], div: usize) -> (BigInt, BigInt, BigInt, BigInt, BigInt) {
//     let m0 = BigInt::new(num[0..cmp::min(div, num.len())].to_vec(), true);
//     let m1 = BigInt::new(
//         num[cmp::min(div, num.len())..cmp::min(div * 2, num.len())].to_vec(),
//         true,
//     );
//     let m2 = BigInt::new(
//         num[cmp::min(div * 2, num.len())..cmp::min(div * 3, num.len())].to_vec(),
//         true,
//     );
//     let p0 = &m0 + &m2;
//     let p_0 = &m0;
//     let p_1 = &p0 + &m1;
//     let p_2 = &m2;
//     let _p_1 = &p0 - &m1;
//     let _p_2 = &((&_p_1 + &m2) * BigInt::two()) - &m0;
//     (p_0.clone(), p_1, p_2.clone(), _p_1, _p_2)
// }

// pub fn div_by_two(num: &[u8]) -> Vec<u8> {
//     let mut num = Vec::from(num);
//     num.reverse();
//     let mut result = Vec::with_capacity(num.len());
//     let mut carry = 0;

//     for &digit in num.iter() {
//         let value = digit / 2 + carry;
//         carry = digit % 2 * 5;

//         if value == 0 && result.is_empty() {
//             continue;
//         }

//         result.push(value);
//     }

//     if result.is_empty() {
//         result.push(0);
//     }

//     result.reverse();
//     result
// }

// pub fn div_by_three(num: &[u8]) -> Vec<u8> {
//     let mut num = Vec::from(num);
//     num.reverse();
//     let mut result = Vec::with_capacity(num.len());
//     let mut carry = 0;

//     for &digit in num.iter() {
//         let value = carry * 10 + digit;
//         carry = value % 3;

//         if value == 0 && result.is_empty() {
//             continue;
//         }

//         result.push(value / 3);
//     }

//     if result.is_empty() {
//         result.push(0);
//     }

//     result.reverse();
//     trim_end_zeros(&mut result);

//     result
// }

impl From<&'static str> for BigInt {
    fn from(value: &'static str) -> Self {
        BigInt::from(value.to_string())
    }
}

impl From<Vec<u8>> for BigInt {
    fn from(value: Vec<u8>) -> Self {
        BigInt::from(String::from_utf8(value).unwrap())
    }
}

impl From<String> for BigInt {
    fn from(value: String) -> Self {
        let mut bytes: Vec<u8> = value.into();
        bytes.retain(|i| *i <= b'9' && *i >= b'0' || *i == b'-');

        let positive = bytes[0] != b'-';

        if !positive {
            bytes.remove(0);
        }

        let rem_len = bytes.len() % BASE_POW;
        let mut rem: Digit = 0;
        if rem_len > 0 {
            let mut _rem: Vec<u8> = vec![0; rem_len];
            for i in 0..rem_len {
                _rem[i] = bytes.remove(0);
            }
            rem = String::from_utf8(_rem).unwrap().parse().unwrap();
        }

        let mut bytes: Vec<Digit> = bytes
            .chunks(BASE_POW)
            .map(|i| {
                String::from_utf8(i.to_vec())
                    .unwrap()
                    .parse::<Digit>()
                    .unwrap()
            })
            .collect();

        bytes.reverse();

        if rem_len != 0 {
            bytes.push(rem);
        }

        Self {
            digits: bytes,
            positive,
        }
    }
}

pub trait ToBigInt {
    fn to_bigint(&self) -> BigInt;
}

macro_rules! impl_to_bigint {
    ($($t:ty)+) => ($(
        impl ToBigInt for $t {
            fn to_bigint(&self) -> BigInt {
                BigInt::from(*self)
            }
        }
    )+)
}

impl_to_bigint! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_to_bigint! { &usize &u8 &u16 &u32 &u64 &u128 &isize &i8 &i16 &i32 &i64 &i128 }

macro_rules! impl_from_int {
    ($($t:ty)+) => ($(
        impl From<$t> for BigInt {
            fn from(value: $t) -> Self {
                let bytes = value.to_string();
                BigInt::from(bytes)
            }
        }
    )+)
}

impl_from_int! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_from_int! { &usize &u8 &u16 &u32 &u64 &u128 &isize &i8 &i16 &i32 &i64 &i128 }
impl_from_int! { &mut usize &mut u8 &mut u16 &mut u32 &mut u64 &mut u128 &mut isize &mut i8 &mut i16 &mut i32 &mut i64 &mut i128 }

macro_rules! impl_shl {
    ($($t:ty)+) => ($(
        impl Shl<$t> for BigInt {
            type Output = BigInt;

            fn shl(mut self, rhs: $t) -> Self::Output {
                for _ in 0..rhs {
                    self.digits.push(0);
                }
                self
            }
        }
    )+);
}

macro_rules! impl_shl_ref {
    ($t1:ty, $t2:ty) => {
        impl Shl<$t1> for $t2 {
            type Output = BigInt;

            fn shl(self, rhs: $t1) -> Self::Output {
                let mut x = self.clone();
                for _ in 0..rhs {
                    x.digits.push(0);
                }
                x
            }
        }
    };
}

impl_shl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_shl_ref!(usize, &BigInt);
impl_shl_ref!(u8, &BigInt);
impl_shl_ref!(u16, &BigInt);
impl_shl_ref!(u32, &BigInt);
impl_shl_ref!(u64, &BigInt);
impl_shl_ref!(u128, &BigInt);
impl_shl_ref!(isize, &BigInt);
impl_shl_ref!(i8, &BigInt);
impl_shl_ref!(i16, &BigInt);
impl_shl_ref!(i32, &BigInt);
impl_shl_ref!(i64, &BigInt);
impl_shl_ref!(i128, &BigInt);
impl_shl_ref!(usize, &mut BigInt);
impl_shl_ref!(u8, &mut BigInt);
impl_shl_ref!(u16, &mut BigInt);
impl_shl_ref!(u32, &mut BigInt);
impl_shl_ref!(u64, &mut BigInt);
impl_shl_ref!(u128, &mut BigInt);
impl_shl_ref!(isize, &mut BigInt);
impl_shl_ref!(i8, &mut BigInt);
impl_shl_ref!(i16, &mut BigInt);
impl_shl_ref!(i32, &mut BigInt);
impl_shl_ref!(i64, &mut BigInt);
impl_shl_ref!(i128, &mut BigInt);

macro_rules! impl_shr {
    ($($t:ty)+) => ($(
        impl Shr<$t> for BigInt {
            type Output = BigInt;

            fn shr(mut self, rhs: $t) -> Self::Output {
                for _ in 0..rhs {
                    self.digits.insert(0, 0);
                }
                self
            }
        }
    )+);
}

macro_rules! impl_shr_ref {
    ($t1:ty, $t2:ty) => {
        impl Shr<$t1> for $t2 {
            type Output = BigInt;

            fn shr(self, rhs: $t1) -> Self::Output {
                let mut x = self.clone();
                for _ in 0..rhs {
                    x.digits.insert(0, 0);
                }
                x
            }
        }
    };
}

impl_shr! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
impl_shr_ref!(usize, &BigInt);
impl_shr_ref!(u8, &BigInt);
impl_shr_ref!(u16, &BigInt);
impl_shr_ref!(u32, &BigInt);
impl_shr_ref!(u64, &BigInt);
impl_shr_ref!(u128, &BigInt);
impl_shr_ref!(isize, &BigInt);
impl_shr_ref!(i8, &BigInt);
impl_shr_ref!(i16, &BigInt);
impl_shr_ref!(i32, &BigInt);
impl_shr_ref!(i64, &BigInt);
impl_shr_ref!(i128, &BigInt);
impl_shr_ref!(usize, &mut BigInt);
impl_shr_ref!(u8, &mut BigInt);
impl_shr_ref!(u16, &mut BigInt);
impl_shr_ref!(u32, &mut BigInt);
impl_shr_ref!(u64, &mut BigInt);
impl_shr_ref!(u128, &mut BigInt);
impl_shr_ref!(isize, &mut BigInt);
impl_shr_ref!(i8, &mut BigInt);
impl_shr_ref!(i16, &mut BigInt);
impl_shr_ref!(i32, &mut BigInt);
impl_shr_ref!(i64, &mut BigInt);
impl_shr_ref!(i128, &mut BigInt);

#[macro_export]
macro_rules! bigint {
    ($num:literal) => {
        BigInt::from(stringify!($num))
    };
}

macro_rules! impl_sub {
    ($t1:ty, $t2:ty) => {
        impl Sub<$t1> for $t2 {
            type Output = BigInt;

            fn sub(self, rhs: $t1) -> Self::Output {
                match (self.positive, rhs.positive) {
                    (true, true) => match _cmp(&self.digits, &rhs.digits) {
                        Ordering::Less => BigInt::new(sub(&rhs.digits, &self.digits), false),
                        Ordering::Equal => BigInt::zero(),
                        Ordering::Greater => BigInt::new(sub(&self.digits, &rhs.digits), true),
                    },
                    (true, false) => BigInt::new(add(&self.digits, &rhs.digits), true),
                    (false, true) => BigInt::new(add(&self.digits, &rhs.digits), false),
                    (false, false) => match _cmp(&self.digits, &rhs.digits) {
                        Ordering::Less => BigInt::new(sub(&rhs.digits, &self.digits), true),
                        Ordering::Equal => BigInt::zero(),
                        Ordering::Greater => BigInt::new(sub(&self.digits, &rhs.digits), false),
                    },
                }
            }
        }
    };
}

impl_sub!(BigInt, BigInt);
impl_sub!(BigInt, &BigInt);
impl_sub!(&BigInt, BigInt);
impl_sub!(&BigInt, &BigInt);
impl_sub!(&mut BigInt, BigInt);
impl_sub!(BigInt, &mut BigInt);
impl_sub!(&BigInt, &mut BigInt);
impl_sub!(&mut BigInt, &BigInt);
impl_sub!(&mut BigInt, &mut BigInt);

impl AsRef<BigInt> for BigInt {
    fn as_ref(&self) -> &BigInt {
        self
    }
}

macro_rules! impl_add {
    ($t1:ty, $t2:ty) => {
        impl Add<$t1> for $t2 {
            type Output = BigInt;

            fn add(self, rhs: $t1) -> Self::Output {
                match (self.positive, rhs.positive) {
                    (true, true) => BigInt::new(add(&self.digits, &rhs.digits), true),
                    (true, false) => match _cmp(&self.digits, &rhs.digits) {
                        Ordering::Less => BigInt::new(sub(&rhs.digits, &self.digits), false),
                        Ordering::Equal => BigInt::zero(),
                        Ordering::Greater => BigInt::new(sub(&self.digits, &rhs.digits), true),
                    },
                    (false, true) => match _cmp(&self.digits, &rhs.digits) {
                        Ordering::Less => BigInt::new(sub(&rhs.digits, &self.digits), true),
                        Ordering::Equal => BigInt::zero(),
                        Ordering::Greater => BigInt::new(sub(&self.digits, &rhs.digits), false),
                    },
                    (false, false) => BigInt::new(add(&self.digits, &rhs.digits), false),
                }
            }
        }
    };
}

impl_add!(BigInt, BigInt);
impl_add!(BigInt, &BigInt);
impl_add!(&BigInt, BigInt);
impl_add!(&BigInt, &BigInt);
impl_add!(&mut BigInt, BigInt);
impl_add!(BigInt, &mut BigInt);
impl_add!(&BigInt, &mut BigInt);
impl_add!(&mut BigInt, &BigInt);
impl_add!(&mut BigInt, &mut BigInt);

macro_rules! impl_add_assign {
    ($t1:ty, $t2:ty) => {
        impl AddAssign<$t1> for $t2 {
            fn add_assign(&mut self, rhs: $t1) {
                *self = self.add(rhs);
            }
        }
    };
}

impl_add_assign!(BigInt, BigInt);
impl_add_assign!(&BigInt, BigInt);
impl_add_assign!(&mut BigInt, BigInt);

macro_rules! impl_mul {
    ($t1:ty, $t2:ty) => {
        impl Mul<$t1> for $t2 {
            type Output = BigInt;

            fn mul(self, rhs: $t1) -> Self::Output {
                let mul = mul(&self.digits, &rhs.digits);
                match (self.positive, rhs.positive) {
                    (true, true) => BigInt::new(mul, true),
                    (true, false) => BigInt::new(mul, false),
                    (false, true) => BigInt::new(mul, false),
                    (false, false) => BigInt::new(mul, true),
                }
            }
        }
    };
}

impl_mul!(BigInt, BigInt);
impl_mul!(BigInt, &BigInt);
impl_mul!(&BigInt, BigInt);
impl_mul!(&BigInt, &BigInt);
impl_mul!(&mut BigInt, BigInt);
impl_mul!(BigInt, &mut BigInt);
impl_mul!(&BigInt, &mut BigInt);
impl_mul!(&mut BigInt, &BigInt);
impl_mul!(&mut BigInt, &mut BigInt);

macro_rules! impl_mul_assign {
    ($t1:ty, $t2:ty) => {
        impl MulAssign<$t1> for $t2 {
            fn mul_assign(&mut self, rhs: $t1) {
                *self = self.clone().mul(rhs);
            }
        }
    };
}

impl_mul_assign!(BigInt, BigInt);
impl_mul_assign!(&BigInt, BigInt);
impl_mul_assign!(&mut BigInt, BigInt);

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits && self.positive == other.positive
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.positive, other.positive) {
            (true, true) => _cmp(&self.digits, &other.digits),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => _cmp(&other.digits, &self.digits),
        }
    }
}

impl Eq for BigInt {}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
            positive: self.positive.clone(),
        }
    }
}

macro_rules! impl_try_into_int {
    ($t1:ty, $t2:ty) => {
        impl TryInto<$t1> for $t2 {
            type Error = ParseIntError;

            fn try_into(self) -> Result<$t1, Self::Error> {
                let s = self.to_string();
                let n = s.parse::<$t1>()?;
                Ok(n)
            }
        }
    };
}

impl_try_into_int!(usize, BigInt);
impl_try_into_int!(u8, BigInt);
impl_try_into_int!(u16, BigInt);
impl_try_into_int!(u32, BigInt);
impl_try_into_int!(u64, BigInt);
impl_try_into_int!(u128, BigInt);
impl_try_into_int!(isize, BigInt);
impl_try_into_int!(i8, BigInt);
impl_try_into_int!(i16, BigInt);
impl_try_into_int!(i32, BigInt);
impl_try_into_int!(i64, BigInt);
impl_try_into_int!(i128, BigInt);
impl_try_into_int!(usize, &BigInt);
impl_try_into_int!(u8, &BigInt);
impl_try_into_int!(u16, &BigInt);
impl_try_into_int!(u32, &BigInt);
impl_try_into_int!(u64, &BigInt);
impl_try_into_int!(u128, &BigInt);
impl_try_into_int!(isize, &BigInt);
impl_try_into_int!(i8, &BigInt);
impl_try_into_int!(i16, &BigInt);
impl_try_into_int!(i32, &BigInt);
impl_try_into_int!(i64, &BigInt);
impl_try_into_int!(i128, &BigInt);
impl_try_into_int!(usize, &mut BigInt);
impl_try_into_int!(u8, &mut BigInt);
impl_try_into_int!(u16, &mut BigInt);
impl_try_into_int!(u32, &mut BigInt);
impl_try_into_int!(u64, &mut BigInt);
impl_try_into_int!(u128, &mut BigInt);
impl_try_into_int!(isize, &mut BigInt);
impl_try_into_int!(i8, &mut BigInt);
impl_try_into_int!(i16, &mut BigInt);
impl_try_into_int!(i32, &mut BigInt);
impl_try_into_int!(i64, &mut BigInt);
impl_try_into_int!(i128, &mut BigInt);

macro_rules! impl_to_string {
    ($($t:ty)+) => ($(
        impl ToString for $t {
            fn to_string(&self) -> String {
                let digits_len = self.digits.len();
                let mut x: String = self.digits.iter().enumerate().map(|(i, digit)| {
                    let mut s = digit.to_string();
                    if s.len() < BASE_POW && i != digits_len - 1 {
                        let diff = BASE_POW - s.len();
                        s.insert_str(0, &"0".repeat(diff));
                    }
                    s
                }).rev().collect();
                if !self.positive {
                    x.insert(0, '-');
                }
                x
            }
        }
    )+);
}

impl_to_string! { BigInt &BigInt &mut BigInt}
