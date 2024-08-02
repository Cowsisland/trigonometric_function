use trigonometric_function::trig_functions::*;

fn main() {
    // 角度（弧度法）
    let angle = 45.0;

    // sinを実行
    println!("sin: {}", angle.sin());

    // cosを実行
    println!("cos: {}", angle.cos());

    // tanを実行
    println!("tan: {}", angle.tan());

    // 値
    let val = 0.5;
    // arcsinを実行
    println!("arcsin: {}", val.arcsin());

    // arccosを実行
    println!("arccos: {}", val.arccos());

    // arctanを実行
    println!("arctan: {}", val.arctan());
}