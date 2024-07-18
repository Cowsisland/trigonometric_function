use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Number {
    // f64への変換
    // 各型への変換を行ってもよいかも
    pub fn to_f64(self) -> f64 {
        match self {
            Number::I32(n) => n as f64,
            Number::I64(n) => n as f64,
            Number::F32(n) => n as f64,
            Number::F64(n) => n,
        }
    }

    pub fn to_i32(self) -> i32 {
        todo!()
    }

    pub fn to_i64(self) -> i64 {
        todo!()
    }

    pub fn to_f32(self) -> f32 {
        todo!()
    }

    pub fn abs(self) -> Self {
        match self {
            Number::I32(n) => Number::I32(n.abs()),
            Number::I64(n) => Number::I64(n.abs()),
            Number::F32(n) => Number::F32(n.abs()),
            Number::F64(n) => Number::F64(n.abs()),
        }
    }
}

// displayトレイトをNumberに実装
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// 四則演算の実装
// 下記の理解を深める
impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Number::F64(self.to_f64() + other.to_f64())
    }
}

impl Sub for Number {
 type Output = Self;

 fn sub(self, other: Self) -> Self::Output {
    Number::F64(self.to_f64() - other.to_f64())
 }
}

impl Mul for Number {
    type Output = Self;
   
    fn mul(self, other: Self) -> Self::Output {
       Number::F64(self.to_f64() * other.to_f64())
    }
}

impl Div for Number {
    type Output = Self;
   
    fn div(self, other: Self) -> Self::Output {
       Number::F64(self.to_f64() / other.to_f64())
    }
}

// AddAssignの実装
impl AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        *self = Number::F64(self.to_f64() + other.to_f64());
    }
}

// MulAssignの実装
impl MulAssign for Number {
    fn mul_assign(&mut self, other: Self) {
        *self = Number::F64(self.to_f64() * other.to_f64());
    }
}

// Negの実装
impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number::F64(-self.to_f64())
    }
}

// 比較の実装
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.to_f64() == other.to_f64()
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_f64().partial_cmp(&other.to_f64())
    }
}