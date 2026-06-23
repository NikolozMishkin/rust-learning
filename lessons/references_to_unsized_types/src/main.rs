use std::mem::size_of;
trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    println!(
        "Size of a reference to sized type: {}",
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to unsized type: {}",
        size_of::<&[i32]>()
    );

    let num_1: &[i32; 3] = &[10, 12, 30];
    let num_2: &[i32] = &[10, 12, 30];

    let mut sum = 0;
    for num in num_1 {
        sum += num;
    }

    for num in num_2 {
        sum += num;
    }

    println!("Size of &Cricle is: {}", size_of::<&Circle>());
    println!("Size of &Rectangle is: {}", size_of::<&Rectangle>());
    println!("Size of &dyn Shape: {}", size_of::<&dyn Shape>());
}
