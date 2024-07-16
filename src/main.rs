use trigonometric_function::model::*;

use std::f64::consts::PI;

trait TrigFunctions {
    fn sin(self) -> f64;
    fn cos(self) -> f64;
    fn tan(self) -> f64;
    fn arcsin(self) -> f64;
    fn arccos(self) -> f64;
    fn arctan(self) -> f64;
}

impl TrigFunctions for f64 {
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
        self.to_radians().arcsin()
    }

    fn arccos(self) -> f64 {
        self.to_radians().arccos()
    }

    fn arctan(self) -> f64 {
        self.to_radians().arctan()
    }
}

trait ToRadians {
    fn to_radians(self) -> f64;
}

impl ToRadians for f64 {
    fn to_radians(self) -> f64 {
        self * PI / 180.0
    }
}


fn main() {
    let rad = 45.0;
    println!("sin: {}", rad.sin());
    println!("cos: {}", rad.cos());
    println!("tan: {}", rad.tan());
}