// todo: Error処理は後で書く
use std::{f64::consts::PI, fmt::Error};

// 一旦traitで書く
trait TrigFunctions {
    fn to_radians(self) -> f64;
    fn sin(self) -> f64;
    fn cos(self) -> f64;
    fn tan(self) -> f64;
    fn arcsin(self) -> f64;
    fn arccos(self) -> f64;
    fn arctan(self) -> f64;
}

impl TrigFunctions for f64 {
    fn to_radians(self) -> f64 {
        self * PI / 180.0
    }

    fn sin(self) -> f64 {
        self.to_radians().sin()
    }

    fn cos(self) -> f64 {
        self.to_radians().cos()
    }

    fn tan(self) -> f64 {
        self.to_radians().tan()
    }

    fn arcsin(self) -> f64 {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arcsin is undefined for values outside the range [-1, 1]");
        }
        self.to_radians().asin().to_degrees()
    }

    fn arccos(self) -> f64 {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arccos is undefined for values outside the range [-1, 1]");
        }
        self.to_radians().acos().to_degrees()
    }

    fn arctan(self) -> f64 {
        self.to_radians().atan().to_degrees()
    }
}

// sinを計算して返す
pub fn calc_sin_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.sin())
}

// cosを計算して返す
pub fn calc_cos_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.cos())
}

// tanを計算して返す
pub fn calc_tan_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.tan())
}

// arcsinを計算して返す
pub fn calc_arcsin_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.arcsin())
}

// arccosを計算して返す
pub fn calc_arccos_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.arccos())
}

// arctanを計算して返す
pub fn calc_arctan_function(angle: f64) -> Result<f64, Error> {
    Ok(angle.arctan())
}