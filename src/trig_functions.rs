use std::{f64::consts::PI, fmt::{Display, Error}};

use crate::number;
use number::Number;

// 三角関数用トレイト
// 演算を行うため、enum型のNumber型に対して実装
pub trait TrigFunctions<T = Number> {
    fn to_radians(self) -> T;
    fn sin(self) -> T;
    fn cos(self) -> T;
    fn tan(self) -> T;
    fn arcsin(self) -> T;
    fn arccos(self) -> T;
    fn arctan(self) -> T;
}

// Number型にTrigFunctionsトレイトを実装
impl TrigFunctions for Number {
    fn to_radians(self) -> Number {
        let pi = Number::F64(PI);
        let pi_angle = Number::F64(180.0);
        self * pi / pi_angle
    }

    fn sin(self) -> Number {
        // 角度をラジアンに変換
        let x = self.to_radians();
        // テイラー級数展開によるsinの近似計算
        let mut term = x; // 初項
        let mut result = Number::F64(0.0);
        let mut n = Number::I32(1);
        
        while term.abs() > Number::F64(1e-10) { // 精度を指定
            result += term;
            term *= -x * x / ((Number::I32(2) * n) * (Number::I32(2) * n + Number::I32(1)));
            n += Number::I32(1);
        }
        
        result
    }

    fn cos(self) -> Number {
        todo!();
    }

    fn tan(self) -> Number {
        todo!();
    }

    fn arcsin(self) -> Number {
        if self.to_radians() < Number::F64(-1.0) || self.to_radians() > Number::F64(1.0) {
            panic!("arcsin is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arccos(self) -> Number {
        if self.to_radians() < Number::F64(-1.0) || self.to_radians() > Number::F64(1.0) {
            panic!("arccos is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arctan(self) -> Number {
        todo!();
    }
}

// sinを計算して返す
pub fn calc_sin_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.sin())
}

// cosを計算して返す
pub fn calc_cos_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.cos())
}

// tanを計算して返す
pub fn calc_tan_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.tan())
}

// arcsinを計算して返す
pub fn calc_arcsin_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.arcsin())
}

// arccosを計算して返す
pub fn calc_arccos_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.arccos())
}

// arctanを計算して返す
pub fn calc_arctan_function<T: TrigFunctions + Display>(angle: T) -> Result<Number, Error> {
    Ok(angle.arctan())
}