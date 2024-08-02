use trigonometric_function::trig_functions::*;

fn main() {
    // 角度（弧度法）
    let angle = 90.0;

    // sinを実行
    println!("sin: {}", angle.my_sin().unwrap());

    // cosを実行
    println!("cos: {}", angle.my_cos().unwrap());

    // tanを実行
    match angle.my_tan() {
        Ok(result) => println!("tan: {}", result),
        Err(e) => println!("Error computing tan({}): {}", angle, e),
    }

    // arcsinを実行
    match angle.my_arcsin() {
        Ok(result) => println!("arcsin: {}", result),
        Err(e) => println!("Error computing arcsin({}): {}", angle, e),
    }

    // arccosを実行
    match angle.my_arccos() {
        Ok(result) => println!("arccos: {}", result),
        Err(e) => println!("Error computing arccos({}): {}", angle, e),
    }

    // arctanを実行
    println!("arctan: {}", angle.my_arctan().unwrap());
}