fn main() {
    //variables and data types

    let x: i16 = 10;
    println!("The value of x is: {x}");

    //mutability

    let mut y = 5;
    y = 20;
    println!("The value of x is: {y}");

    //scopes

    {
        let z = 50;
    }
    //let s = z; cant do that because z is out of scope

    //shadowing

    let t = 10;
    let t = t + 50;
    println!("The value of t is: {t}");

    let u: i32 = 3;
    let u: f64 = 3.0;

    let v: i32 = 30;
    {
        let v: i32 = 40;
        println!("The value of v in inner scope is: {v}");
    }
    println!("The value of v in outer scope is: {v}");

    //constants

    const MAX_VALUE: u32 = 100;
}
