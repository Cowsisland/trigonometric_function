use trigonometric_function::trig_functions::*;

fn main() {
    let sin = calc_sin_function(45.0);
    println!("sin: {}", sin.unwrap());

    let cos = calc_cos_function(45.0);
    println!("cos: {}", cos.unwrap());

    let tan = calc_tan_function(45.0);
    println!("tan: {}", tan.unwrap());

    let arcsin = calc_arcsin_function(45.0);
    println!("arcsin: {}", arcsin.unwrap());

    let arccos = calc_arccos_function(45.0);
    println!("arccos: {}", arccos.unwrap());

    let arctan = calc_arctan_function(45.0);
    println!("arctan: {}", arctan.unwrap());
}