use std::f64::consts::{PI, FRAC_PI_2};
// use std::fmt::{Display, Error};
use number::Number;

use crate::{error::TrigError, number};

// 三角関数用トレイト
// 演算を行うため、Numberトレイトを継承する
pub trait TrigFunctions: Number {
    type T;
    fn to_radians(self) -> Self::T;
    fn my_sin(self) -> Option<Self::T>;
    fn my_cos(self) -> Option<Self::T>;
    fn my_tan(self) -> Result<Self::T, TrigError>;
    fn my_arcsin(self) -> Result<Self::T, TrigError>;
    fn my_arccos(self) -> Result<Self::T, TrigError>;
    fn my_arctan(self) -> Result<Self::T, TrigError>;
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

    fn my_sin(self) -> Option<Self::T> {
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
        
        Some(result)
    }

    fn my_cos(self) -> Option<Self::T> {
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
        
        Some(result)
    }

    fn my_tan(self) -> Result<Self::T, TrigError> {
        if self.cos() == 0.0 {
            //panic!("cos is undefined for values outside the range 0");
            return Err(TrigError::OutOfRangeForTan(self));
        }
        Ok(self.sin() / self.cos())
    }

    fn my_arcsin(self) -> Result<Self::T, TrigError> {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            // panic!("arcsin is undefined for values outside the range [-1, 1]");
            return Err(TrigError::OutOfRangeForArcsin(self));
        }

        let x = self;
        let mut term = x;
        let mut result = x;
        let mut n = 1;

        while term.abs() > 1e-10 {
            let prev_term = term;
            term *= (x * x * (2 * n - 1) as f64 * (2 * n - 1) as f64) / ((2 * n) as f64 * (2 * n + 1) as f64);

            // オーバーフローをチェック
            if term.is_infinite() || term.is_nan() || term.abs() > prev_term.abs() {
                return Err(TrigError::Overflow);
            }

            result += term;
            n += 1;
        }

        Ok(result)
    }

    fn my_arccos(self) -> Result<Self::T, TrigError> {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            // panic!("arccos is undefined for values outside the range [-1, 1]");
            return Err(TrigError::OutOfRangeForArccos(self));
        }
        // arccos(x) = π/2 - arcsin(x)
        let half_pi = FRAC_PI_2;
        let arcsin_val = self.my_arcsin()?;
        Ok(half_pi - arcsin_val)
    }

    fn my_arctan(self) -> Result<Self::T, TrigError> {
        let x = self;
        let mut term = x;
        let mut result = x;
        let mut n = 1;

        while term.abs() > 1e-10 {
            let prev_term = term;
            term *= -x * x / ((2 * n) as f64 + 1.0);

            // オーバーフローをチェック
            if term.is_infinite() || term.is_nan() || term.abs() > prev_term.abs() {
                return Err(TrigError::Overflow);
            }

            result += term;
            n += 1;
        }

        Ok(result)
    }
}


// 以下はテストコード
#[cfg(test)]
mod trig_functions_tests {
    use crate::trig_functions::TrigFunctions;

    #[test]
    fn test_trig_functions() {
        let angle = 45.0;
        assert_eq!(angle.my_sin().unwrap(), 0.7071067811796194);
        assert_eq!(angle.my_cos().unwrap(), 0.7071067811869363);
        assert_eq!(angle.my_tan().unwrap(), 1.6197751905438615);

        let val = 0.5;
        assert_eq!(val.my_arcsin().unwrap(), 0.5235987755858897);
        assert_eq!(val.my_arccos().unwrap(), 1.0471975512090068);
        assert_eq!(val.my_arctan().unwrap(), 0.4603442826192663);
    }
}