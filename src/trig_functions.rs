use std::f64::consts::PI;
// use std::fmt::{Display, Error};

use crate::number;
use number::Number;

// 三角関数用トレイト
// 演算を行うため、Numberトレイトを継承する
pub trait TrigFunctions: Number {
    type T;
    fn to_radians(self) -> Self::T;
    fn sin(self) -> Self::T;
    fn cos(self) -> Self::T;
    fn tan(self) -> Self::T;
    fn arcsin(self) -> Self::T;
    fn arccos(self) -> Self::T;
    fn arctan(self) -> Self::T;
}

// Number型にTrigFunctionsトレイトを実装
// 返り値の型はf64ただ一つに定まるため、typeの指定
impl TrigFunctions for f64 {
    type T = f64;

    fn to_radians(self) -> Self::T {
        let pi = PI;
        let pi_angle = 180.0;
        self * pi / pi_angle
    }

    fn sin(self) -> Self::T {
        // 角度をラジアンに変換
        let x = self.to_radians();
        // テイラー級数展開によるsinの近似計算
        let mut term = x; // 初項
        let mut result = 0.0;
        let mut n = 1;
        
        while term.abs() > 1e-10 { // 精度を指定
            result += term;
            term *= -x * x / ((2 * n) * (2 * n + 1)).to_f64();
            n += 1;
        }
        
        result
    }

    fn cos(self) -> Self::T {
        // 角度をラジアンに変換
        let x = self.to_radians();
        // テイラー級数展開によるcosの近似計算
        let mut term = 1.0; // 初項
        let mut result = 0.0;
        let mut n = 0;
        
        while term.abs() > 1e-10 { // 精度を指定
            result += term;
            term *= -x * x / ((2 * n + 1) as f64 * (2 * n + 2) as f64);
            n += 1;
        }
        
        result
    }

    fn tan(self) -> Self::T {
        if self.cos() == 0.0 {
            panic!("cos is undefined for values outside the range 0");
        }
        self.sin() / self.cos()
    }

    fn arcsin(self) -> Self::T {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arcsin is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arccos(self) -> Self::T {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arccos is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arctan(self) -> Self::T {
        todo!();
    }
}


// 以下はテストコード
#[cfg(test)]
mod trig_functions_tests {
    use crate::trig_functions::TrigFunctions;

    #[test]
    fn test_trig_functions() {
        let angle = 45.0;
        assert_eq!(angle.sin(), 0.7071067811796194);
        assert_eq!(angle.cos(), 0.5253219888177297);
        assert_eq!(angle.tan(), 1.6197751905438615);
    }
}