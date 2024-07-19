use trigonometric_function::{number::Number, trig_functions::*};

fn main() {
    // sinを実行
    let test_angle = Number::F64(45.0);
    println!("sin: {}", test_angle.sin().to_f64());

    // let angle = Number::F64(45.0);
    // let sin = calc_sin_function(angle);
    // println!("sin: {}", sin.unwrap().to_f64());

    // let cos = calc_cos_function(45.0);
    // println!("cos: {}", cos.unwrap());

    // let tan = calc_tan_function(45.0);
    // println!("tan: {}", tan.unwrap());

    // let arcsin = calc_arcsin_function(45.0);
    // println!("arcsin: {}", arcsin.unwrap());

    // let arccos = calc_arccos_function(45.0);
    // println!("arccos: {}", arccos.unwrap());

    // let arctan = calc_arctan_function(45.0);
    // println!("arctan: {}", arctan.unwrap());
}