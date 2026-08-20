// Types of References
//     ref: &SomeType;       // immutable binding of an immutable reference
// mut ref: &SomeType;       // mutable binding of an immutable reference
//     ref: &mut SomeType;   // immutable binding of a mutable reference
// mut ref: &mut SomeType;   // mutable binding of a mutable reference
//     ref: &mut &SomeType;  // immutable binding of a mutable reference to an
// immutable reference. mut ref: &mut &SomeType;  // mutable binding of a
// mutable reference to an immutable reference.

// variable: SomeType

// &variable: SomeType
// &mut variable: SomeType

fn some_fn_1(&x: &i32) {
    let y = x;
}

fn some_fn_2(&mut x: &mut i32) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn my_fn(Point { x, y }: &Point) {
    println!("x: {x}, y: {y}");
}

fn main() {
    let mut x = 42;
    let y = &mut x;
    let &mut z = y; // let z = *y;

    // Pattern Matching
    // &mut z = &mut i32
    // z = i32

    let mut x = 42;
    let y = &x;
    let &z = y; // let z = *y;

    // &z = &i32
    // z = i32

    let mut x = 42;
    let y = &&x;
    let &z = y; // let z = &x;

    some_fn_2(&mut x);

    // &z = &&i32
    // z = &i32

    // let mut x = String::from("");
    // let y = &x;
    // let &z = y; // Error since x is a non-copy type
}
