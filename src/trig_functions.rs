// todo: Error処理は後で書く
use std::{f64::consts::PI, fmt::Error};

// 一旦traitで書く
pub trait TrigFunctions {
    fn to_radians(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn arcsin(self) -> Self;
    fn arccos(self) -> Self;
    fn arctan(self) -> Self;
}

// f64型にTrigFunctionsトレイトを実装
impl TrigFunctions for f64 {
    fn to_radians(self) -> Self {
        self * PI / 180.0
    }

    fn sin(self) -> Self {
        // 角度をラジアンに変換
        let x = self.to_radians();
        // テイラー級数展開によるsinの近似計算
        let mut term = x; // 初項
        let mut result = 0.0;
        let mut n = 1;
        
        while term.abs() > 1e-10 { // 精度を指定
            result += term;
            term *= -x * x / ((2 * n) as f64 * (2 * n + 1) as f64);
            n += 1;
        }
        
        result
    }

    fn cos(self) -> Self {
        todo!();
    }

    fn tan(self) -> Self {
        todo!();
    }

    fn arcsin(self) -> Self {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arcsin is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arccos(self) -> Self {
        if self.to_radians() < -1.0 || self.to_radians() > 1.0 {
            panic!("arccos is undefined for values outside the range [-1, 1]");
        }
        todo!();
    }

    fn arctan(self) -> Self {
        todo!();
    }
}

// sinを計算して返す
// TrigFunctionsトレイトはf64にしか実装していないが、例えばi32型にも実装すると、i32も受け取れるようになる
pub fn calc_sin_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.sin())
}

// cosを計算して返す
pub fn calc_cos_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.cos())
}

// tanを計算して返す
pub fn calc_tan_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.tan())
}

// arcsinを計算して返す
pub fn calc_arcsin_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.arcsin())
}

// arccosを計算して返す
pub fn calc_arccos_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.arccos())
}

// arctanを計算して返す
pub fn calc_arctan_function<T: TrigFunctions>(angle: T) -> Result<T, Error> {
    Ok(angle.arctan())
}