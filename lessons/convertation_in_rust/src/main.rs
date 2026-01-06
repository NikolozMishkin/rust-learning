fn main() {
    let a: i32 = 16_777_217;
    let b: f32 = a as f32;
    let c: i32 = b as i32;

    println!("a = {}, b = {}, c = {}", a, b, c);

    let z: f32 = 19.999;
    let x: i32 = z as i32;
    println!("x = {}", x);
}

// float rounds to the nearest numbet to 0 when it can't represent the number exactly
// every integer up to 16,777,216 (2²⁴) is exac above that, integers cannot be represented exactly
