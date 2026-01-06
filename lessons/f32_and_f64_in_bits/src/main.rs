fn main() {
    let a: f32 = 0.1 + 0.2;
    let b: f64 = 0.1 + 0.2;

    println!("{}", a); // 0.300000012
    println!("{}", b); // 0.30000000000000004
}

//f32 = 32 bits (1-8-23), f64 = 64 bits (1-11-52); more bits → more precision and range.
//f32 = 1 sign bit, 8 exponent bits, 23 mantissa bits
//f64 = 1 sign bit, 11 exponent bits, 52 mantissa bits
//The mantissa (or significand) represents the precision bits of the number.
