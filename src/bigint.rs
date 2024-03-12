#![allow(dead_code)]

use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

#[derive(Debug)]
pub struct BigInt {
    digits: Vec<u8>,
    positive: bool,
}

impl BigInt {
    pub fn new(vec: Vec<u8>, positive: bool) -> Self {
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
}

impl From<&'static str> for BigInt {
    fn from(value: &'static str) -> Self {
        let bytes: Vec<u8> = value.into();
        let positive = bytes[0] != b'-';

        let mut res = Vec::new();

        bytes
            .iter()
            .skip(if positive { 0 } else { 1 })
            .for_each(|i| {
                let n = *i;
                if n == b'_' {
                    return;
                }
                res.push(n - b'0');
            });

        res.reverse();

        Self {
            digits: res,
            positive,
        }
    }
}

#[macro_export]
macro_rules! bigint {
    ($num:literal) => {
        BigInt::from(stringify!($num))
    };
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), false),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), true),
            },
            (true, false) => BigInt::new(_add(&self.digits, &rhs.digits), true),
            (false, true) => BigInt::new(_add(&self.digits, &rhs.digits), false),
            (false, false) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), true),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), false),
            },
        }
    }
}

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), false),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), true),
            },
            (true, false) => BigInt::new(_add(&self.digits, &rhs.digits), true),
            (false, true) => BigInt::new(_add(&self.digits, &rhs.digits), false),
            (false, false) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), true),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), false),
            },
        }
    }
}

impl AsRef<BigInt> for BigInt {
    fn as_ref(&self) -> &BigInt {
        self
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(_add(&self.digits, &rhs.digits), true),
            (true, false) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), false),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), true),
            },
            (false, true) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), true),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), false),
            },
            (false, false) => BigInt::new(_add(&self.digits, &rhs.digits), false),
        }
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(_add(&self.digits, &rhs.digits), true),
            (true, false) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), false),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), true),
            },
            (false, true) => match _cmp(&self.digits, &rhs.digits) {
                Ordering::Less => BigInt::new(_sub(&rhs.digits, &self.digits), true),
                Ordering::Equal => BigInt::zero(),
                Ordering::Greater => BigInt::new(_sub(&self.digits, &rhs.digits), false),
            },
            (false, false) => BigInt::new(_add(&self.digits, &rhs.digits), false),
        }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(_mul(&self.digits, &rhs.digits), true),
            (true, false) => BigInt::new(_mul(&self.digits, &rhs.digits), false),
            (false, true) => BigInt::new(_mul(&self.digits, &rhs.digits), false),
            (false, false) => BigInt::new(_mul(&self.digits, &rhs.digits), true),
        }
    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(_mul(&self.digits, &rhs.digits), true),
            (true, false) => BigInt::new(_mul(&self.digits, &rhs.digits), false),
            (false, true) => BigInt::new(_mul(&self.digits, &rhs.digits), false),
            (false, false) => BigInt::new(_mul(&self.digits, &rhs.digits), true),
        }
    }
}

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

fn _cmp(lhs: &[u8], rhs: &[u8]) -> Ordering {
    if lhs.len() > rhs.len() {
        return Ordering::Greater;
    } else if lhs.len() < rhs.len() {
        return Ordering::Less;
    }

    for i in lhs.iter().zip(rhs.iter()).rev() {
        if i.0 > i.1 {
            return Ordering::Greater;
        } else if i.0 < i.1 {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn _add(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    let mut carry = 0;
    let mut i = 0;
    let (min, max, max_ref) = if lhs.len() > rhs.len() {
        (rhs.len(), lhs.len(), lhs)
    } else {
        (lhs.len(), rhs.len(), rhs)
    };

    let mut res: Vec<u8> = Vec::with_capacity(max);

    while i < min {
        let a = unsafe { *lhs.get_unchecked(i) + *rhs.get_unchecked(i) + carry };
        if a > 9 {
            res.push(a - 10);
            carry = 1;
        } else {
            res.push(a);
            carry = 0;
        }
        i += 1;
    }

    while i < max {
        let a = unsafe { *max_ref.get_unchecked(i) + carry };
        if a > 9 {
            res.push(a - 10);
            carry = 1;
        } else {
            res.push(a);
            carry = 0;
        }
        i += 1;
    }

    if carry != 0 {
        res.push(carry);
    }

    res
}

fn _sub(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    let min = rhs.len();
    let max = lhs.len();
    let mut res: Vec<u8> = Vec::with_capacity(max);

    lhs[min..max].iter().rev().for_each(|i| res.push(*i));

    let mut i = min - 1;
    loop {
        let l_i = unsafe { *lhs.get_unchecked(i) };
        let r_i = unsafe { *rhs.get_unchecked(i) };
        if l_i < r_i {
            res.push(10 + l_i - r_i);
            let mut j = res.len() - 2;
            loop {
                unsafe {
                    if *res.get_unchecked(j) == 0 {
                        *res.get_unchecked_mut(j) = 9;
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

fn trim_end_zeros(slice: &mut Vec<u8>) {
    let mut i = slice.len() - 1;
    if unsafe { *slice.get_unchecked(i) != 0 } {
        return;
    }
    loop {
        unsafe {
            if *slice.get_unchecked(i) != 0 {
                slice.truncate(i + 1);
            }
        }
        if i == 0 {
            slice.truncate(1);
            return;
        }
        i -= 1;
    }
}

pub fn _mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    let (min, max, min_ref, max_ref) = if lhs.len() > rhs.len() {
        (rhs.len(), lhs.len(), rhs, lhs)
    } else {
        (lhs.len(), rhs.len(), lhs, rhs)
    };

    let mut carry = 0;
    let mut temp: Vec<u8> = Vec::with_capacity(max);
    let mut res: Vec<u8> = Vec::with_capacity(max);

    let mut i = 0;
    while i < min {
        (0..i).for_each(|_| temp.push(0));

        let mut j = 0;
        while j < max {
            let a = unsafe { max_ref.get_unchecked(j) * min_ref.get_unchecked(i) + carry };
            if a > 9 {
                temp.push(a % 10);
                carry = a / 10;
            } else {
                temp.push(a);
                carry = 0;
            }
            j += 1;
        }
        if carry != 0 {
            let digits = to_digits(carry);
            digits.into_iter().rev().for_each(|i| temp.push(i));
            carry = 0;
        }
        res = _add(&res, &temp);
        temp.clear();

        i += 1;
    }

    res
}

fn to_digits(v: u8) -> Vec<u8> {
    v.to_string().bytes().map(|b| b - b'0').collect()
}
