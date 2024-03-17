use std::{
    cmp::{self, Ordering},
    num::ParseIntError,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Shl, Shr, Sub},
};

#[derive(Debug)]
pub struct BigInt {
    digits: Vec<u8>,
    positive: bool,
}

impl BigInt {
    pub fn new(vec: Vec<u8>, positive: bool) -> Self {
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
            res *= i.clone();
            if i == self {
                break res;
            }
            i = i.incrument();
        }
    }

    pub fn incrument(self) -> Self {
        self + Self::one()
    }

    pub fn get_digit(&self, index: usize) -> Option<&u8> {
        self.digits.get(index)
    }

    pub fn get_digit_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.digits.get_mut(index)
    }

    pub fn digit_count(&self) -> usize {
        self.digits.len()
    }

    pub fn digits(&self) -> &Vec<u8> {
        self.digits.as_ref()
    }

    pub(super) fn div_by_three(&self) -> Self {
        BigInt::new(div_by_three(&self.digits), self.positive)
    }

    pub(super) fn div_by_two(&self) -> Self {
        BigInt::new(div_by_two(&self.digits), self.positive)
    }
}

impl From<&'static str> for BigInt {
    fn from(value: &'static str) -> Self {
        let mut bytes: Vec<u8> = value.into();
        bytes.retain(|i| *i <= b'9' && *i >= b'0' || *i == b'-');

        let positive = bytes[0] != b'-';

        bytes = bytes
            .iter()
            .skip(if positive { 0 } else { 1 })
            .map(|i| *i - b'0')
            .collect();

        bytes.reverse();

        Self {
            digits: bytes,
            positive,
        }
    }
}

impl Shl<usize> for BigInt {
    type Output = BigInt;

    fn shl(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.digits.push(0);
        }
        self
    }
}

impl Shl<usize> for &BigInt {
    type Output = BigInt;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut x = self.clone();
        for _ in 0..rhs {
            x.digits.push(0);
        }
        x
    }
}

impl Shr<usize> for BigInt {
    type Output = BigInt;

    fn shr(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self.digits.insert(0, 0);
        }
        self
    }
}

impl Shr<usize> for &BigInt {
    type Output = BigInt;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut x = self.clone();
        for _ in 0..rhs {
            x.digits.insert(0, 0);
        }
        x
    }
}

impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        let mut bytes: Vec<u8> = value.to_string().into();
        bytes.retain(|i| *i <= b'9' && *i >= b'0' || *i == b'-');

        let positive = bytes[0] != b'-';

        bytes = bytes
            .iter()
            .skip(if positive { 0 } else { 1 })
            .map(|i| *i - b'0')
            .collect();

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

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
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

impl AsRef<BigInt> for BigInt {
    fn as_ref(&self) -> &BigInt {
        self
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
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

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
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

impl Add<BigInt> for &mut BigInt {
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> Self::Output {
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

impl Add<&BigInt> for &mut BigInt {
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> Self::Output {
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

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.add(rhs);
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul = mul(&self.digits, &rhs.digits);
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(mul, true),
            (true, false) => BigInt::new(mul, false),
            (false, true) => BigInt::new(mul, false),
            (false, false) => BigInt::new(mul, true),
        }
    }
}

impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul = mul(&self.digits, &rhs.digits);
        match (self.positive, rhs.positive) {
            (true, true) => BigInt::new(mul, true),
            (true, false) => BigInt::new(mul, false),
            (false, true) => BigInt::new(mul, false),
            (false, false) => BigInt::new(mul, true),
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone().mul(rhs);
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        todo!()
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

impl Clone for BigInt {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
            positive: self.positive.clone(),
        }
    }
}

impl TryInto<isize> for BigInt {
    type Error = ParseIntError;

    fn try_into(self) -> Result<isize, Self::Error> {
        let mut x = self.digits;
        x.iter_mut().for_each(|i| *i = *i + b'0');
        if !self.positive {
            x.push(b'-');
        }
        x.reverse();
        let s = String::from_utf8(x).unwrap();

        let n = s.parse::<isize>()?;
        Ok(n)
    }
}

fn _cmp(lhs: &[u8], rhs: &[u8]) -> Ordering {
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

fn _len_cmp(lhs: &[u8], rhs: &[u8]) -> Ordering {
    if lhs.len() > rhs.len() {
        Ordering::Greater
    } else if lhs.len() < rhs.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn add(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    match (lhs, rhs) {
        (&[], &[]) => vec![0],
        (&[], _) => rhs.to_vec(),
        (_, &[]) => lhs.to_vec(),
        (x, y) => _add(x, y),
    }
}

fn _add(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    let mut carry = 0;
    let mut i = 0;
    let (min, max, max_ref) = if lhs.len() > rhs.len() {
        (rhs.len(), lhs.len(), lhs)
    } else {
        (lhs.len(), rhs.len(), rhs)
    };

    let mut res: Vec<u8> = Vec::with_capacity(max + 1);

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

    trim_end_zeros(&mut res);

    res
}

fn sub(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    match (lhs, rhs) {
        (&[], &[]) => vec![0],
        (&[], _) => rhs.to_vec(),
        (_, &[]) => lhs.to_vec(),
        (&[0], &[0]) => vec![0],
        (x, y) => _sub(x, y),
    }
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

fn _mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
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
            temp.push(carry);
            carry = 0;
        }
        res = _add(&res, &temp);
        temp.clear();

        i += 1;
    }

    trim_end_zeros(&mut res);

    res
}

fn _mul_t3(lhs: &[u8], rhs: &[u8]) -> BigInt {
    let max = if let Ordering::Greater = _len_cmp(lhs, rhs) {
        lhs
    } else {
        rhs
    };

    let div = max.len() / 3 + 1;

    let (p0, p1, p2, _p1, _p2) = _t3_eval(lhs, div);
    let (q0, q1, q2, _q1, _q2) = _t3_eval(rhs, div);

    let r_0 = &p0 * &q0;
    let r_1 = &p1 * &q1;
    let r_2 = &p2 * &q2;
    let _r_1 = &_p1 * &_q1;
    let _r_2 = &_p2 * &_q2;

    let r0 = &r_0;
    let r4 = &r_2;
    let r3 = (&_r_2 - &r_1).div_by_three();
    let r1 = (&r_1 - &_r_1).div_by_two();
    let r2 = &_r_1 - &r_0;
    let r3 = (&r2 - &r3).div_by_two() + (&BigInt::two() * &r_2);
    let r2 = &(&r2 + &r1) - r4;
    let r1 = &r1 - &r3;

    let mut res = BigInt::zero();
    res += r0;
    res += r1 >> div;
    res += r2 >> (2 * div);
    res += r3 >> (3 * div);
    res += r4 >> (4 * div);

    trim_end_zeros(&mut res.digits);

    res
}

fn mul(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    match (lhs, rhs) {
        (&[], &[]) => return vec![0],
        (&[], _) => return vec![0],
        (_, &[]) => return vec![0],
        (x, y) => {
            return if cmp::min(x.len(), y.len()) > 4 {
                _mul_t3(x, y).digits
            } else {
                _mul(x, y)
            }
        }
    }
}

#[inline]
fn _t3_eval<'a>(num: &'a [u8], div: usize) -> (BigInt, BigInt, BigInt, BigInt, BigInt) {
    let m0 = BigInt::new(num[0..cmp::min(div, num.len())].to_vec(), true);
    let m1 = BigInt::new(
        num[cmp::min(div, num.len())..cmp::min(div * 2, num.len())].to_vec(),
        true,
    );
    let m2 = BigInt::new(
        num[cmp::min(div * 2, num.len())..cmp::min(div * 3, num.len())].to_vec(),
        true,
    );
    let p0 = &m0 + &m2;
    let p_0 = &m0;
    let p_1 = &p0 + &m1;
    let p_2 = &m2;
    let _p_1 = &p0 - &m1;
    let _p_2 = &((&_p_1 + &m2) * BigInt::two()) - &m0;
    (p_0.clone(), p_1, p_2.clone(), _p_1, _p_2)
}

pub fn div_by_two(num: &[u8]) -> Vec<u8> {
    let mut num = Vec::from(num);
    num.reverse();
    let mut result = Vec::with_capacity(num.len());
    let mut carry = 0;

    for &digit in num.iter() {
        let value = digit / 2 + carry;
        carry = digit % 2 * 5;

        if value == 0 && result.is_empty() {
            continue;
        }

        result.push(value);
    }

    if result.is_empty() {
        result.push(0);
    }

    result.reverse();
    result
}

pub fn div_by_three(num: &[u8]) -> Vec<u8> {
    let mut num = Vec::from(num);
    num.reverse();
    let mut result = Vec::with_capacity(num.len());
    let mut carry = 0;

    for &digit in num.iter() {
        let value = carry * 10 + digit;
        carry = value % 3;

        if value == 0 && result.is_empty() {
            continue;
        }

        result.push(value / 3);
    }

    if result.is_empty() {
        result.push(0);
    }

    result.reverse();
    trim_end_zeros(&mut result);

    result
}
