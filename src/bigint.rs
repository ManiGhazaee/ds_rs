#![allow(dead_code)]

use std::{cmp::Ordering, ops::{Add, Sub}};

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

impl From<&'static str> for BigInt {
    fn from(value: &'static str) -> Self {
        let mut bytes: Vec<u8> = value.into();
        let positive = bytes[0] != b'-';

        bytes
            .iter_mut()
            .skip(if positive { 0 } else { 1 })
            .for_each(|i| {
                let n = *i;
                if n == b'_' {
                    return;
                }
                *i = n - b'0';
            });

        bytes.reverse();

        Self {
            digits: bytes,
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
        todo!()
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(_add(&self.digits, &rhs.digits), true),
            (true, false) => todo!(),
            (false, true) => todo!(),
            (false, false) => BigInt::new(_add(&self.digits, &rhs.digits), false),
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

    for i in lhs.iter().zip(rhs.iter()) {
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
    }

    if carry != 0 {
        res.push(carry);
    }

    res
}
