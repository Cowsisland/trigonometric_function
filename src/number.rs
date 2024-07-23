use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

pub trait Number:
    Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Sized
    + Copy
{
    fn to_f64(self) -> f64;
    fn to_i32(self) -> i32;
    fn to_i64(self) -> i64;
    fn to_f32(self) -> f32;
    fn abs(self) -> Self;
}

impl Number for i32 {
    fn to_f64(self) -> f64 {
        let n = self;
        n as f64
    }

    fn to_i32(self) -> i32 {
        self
    }

    fn to_i64(self) -> i64 {
        let n = self;
        n as i64
    }

    fn to_f32(self) -> f32 {
        let n = self;
        n as f32
    }

    fn abs(self) -> Self {
        let n = self;
        n.abs()
    }
}

impl Number for i64 {
    fn to_f64(self) -> f64 {
        let n = self;
        n as f64
    }

    fn to_i32(self) -> i32 {
        let n = self;
        n as i32
    }

    fn to_i64(self) -> i64 {
        self
    }

    fn to_f32(self) -> f32 {
        let n = self;
        n as f32
    }

    fn abs(self) -> Self {
        let n = self;
        n.abs()
    }
}

impl Number for f32 {
    fn to_f64(self) -> f64 {
        let n = self;
        n as f64
    }

    fn to_i32(self) -> i32 {
        let n = self;
        n as i32
    }

    fn to_i64(self) -> i64 {
        let n = self;
        n as i64
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn abs(self) -> Self {
        let n = self;
        n.abs()
    }
}

impl Number for f64 {
    fn to_f64(self) -> f64 {
        self
    }

    fn to_i32(self) -> i32 {
        let n = self;
        n as i32
    }

    fn to_i64(self) -> i64 {
        let n = self;
        n as i64
    }

    fn to_f32(self) -> f32 {
        let n = self;
        n as f32
    }

    fn abs(self) -> Self {
        let n = self;
        n.abs()
    }
}